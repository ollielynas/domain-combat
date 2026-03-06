use std::net::UdpSocket;
use std::process::Command;
use std::path::PathBuf;
use tiny_http::Server;

use crate::input_source::{
    dummy_input_device::dummy_input,
    input_device::{InputDevice, InputDeviceData, InputDirectionLeftRight, InputDirectionUpDown},
};

use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Bundled ADB
// ---------------------------------------------------------------------------

#[cfg(target_os = "windows")]
fn get_adb_path() -> PathBuf {
    use std::sync::OnceLock;
    static ADB_PATH: OnceLock<PathBuf> = OnceLock::new();
    ADB_PATH.get_or_init(|| {
        let adb_bytes = include_bytes!("../../bundled/adb.exe");
        let api_bytes = include_bytes!("../../bundled/AdbWinApi.dll");
        let usb_bytes = include_bytes!("../../bundled/AdbWinUsbApi.dll");

        let dir = std::env::temp_dir().join("domain_combat_adb");
        std::fs::create_dir_all(&dir).ok();

        let adb_path = dir.join("adb.exe");
        // Only write if not already there (avoids hammering disk every launch)
        if !adb_path.exists() {
            std::fs::write(&adb_path,                    adb_bytes).ok();
            std::fs::write(dir.join("AdbWinApi.dll"),    api_bytes).ok();
            std::fs::write(dir.join("AdbWinUsbApi.dll"), usb_bytes).ok();
        }
        adb_path
    }).clone()
}

#[cfg(not(target_os = "windows"))]
fn get_adb_path() -> PathBuf {
    // On Mac/Linux adb is installed via package manager and should be on PATH
    PathBuf::from("adb")
}

// ---------------------------------------------------------------------------
// Port pool
// ---------------------------------------------------------------------------

/// Ports cycled through for each new device. Since adb reverse tunnels through
/// USB, the firewall is irrelevant — these are just to distinguish multiple devices.
static FRIENDLY_PORTS: &[i32] = &[8080, 8008, 8888, 3000, 5000, 4000, 4200, 9000];
static NEXT_PORT_INDEX: Mutex<usize> = Mutex::new(0);

fn next_friendly_port() -> Option<i32> {
    let mut idx = NEXT_PORT_INDEX.lock().unwrap();
    if *idx >= FRIENDLY_PORTS.len() {
        return None;
    }
    let port = FRIENDLY_PORTS[*idx];
    *idx += 1;
    Some(port)
}

// ---------------------------------------------------------------------------
// ADB detection
// ---------------------------------------------------------------------------

static KNOWN_ADB_SERIALS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Returns serial numbers of all fully authorised ADB devices.
/// Returns an empty list immediately if adb is unavailable or times out.
fn get_adb_device_serials() -> Vec<String> {
    let Ok(mut child) = Command::new(get_adb_path())
        .args(["devices"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn() else {
        return vec![];
    };

    // Wait up to 300ms — fast if adb server is already running, instant fail if not
    let timeout = std::time::Duration::from_millis(300);
    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break, // process finished
            Ok(None) if start.elapsed() >= timeout => {
                child.kill().ok();
                return vec![];
            }
            Ok(None) => std::thread::sleep(std::time::Duration::from_millis(10)),
            Err(_) => return vec![],
        }
    }

    let Ok(output) = child.wait_with_output() else {
        return vec![];
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1) // skip "List of devices attached" header
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Ignore "unauthorized" or "offline" devices
            if parts.len() == 2 && parts[1] == "device" {
                Some(parts[0].to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Call once at startup to seed the known list without triggering new device logic.
/// Also pre-warms the ADB server in the background so the first poll is fast.
pub fn init_adb_detection() {
    // Spawn adb start-server in the background so it's ready by the time
    // the user clicks "connect" — this avoids the cold-start delay on first poll.
    let adb = get_adb_path();
    std::thread::spawn(move || {
        Command::new(adb)
            .args(["start-server"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .ok();
    });
    *KNOWN_ADB_SERIALS.lock().unwrap() = vec![];
}

/// Returns serials of any newly connected ADB devices since last call.
fn get_new_adb_serials() -> Vec<String> {
    let current = get_adb_device_serials();
    let mut known = KNOWN_ADB_SERIALS.lock().unwrap();
    let new_serials: Vec<String> = current.iter()
        .filter(|s| !known.contains(s))
        .cloned()
        .collect();
    *known = current;
    new_serials
}

/// Runs `adb -s <serial> reverse tcp:<port> tcp:<port>` so the phone can
/// reach the laptop's server at localhost:<port>.
fn open_browser_on_phone(serial: &str, port: i32) {
    let url = format!("http://localhost:{port}");
    let status = Command::new(get_adb_path())
        .args([
            "-s", serial,
            "shell", "am", "start",
            "-a", "android.intent.action.VIEW",
            "-d", &url,
        ])
        .status();
    match status {
        Ok(s) if s.success() => println!("opened browser on {serial} at {url}"),
        Ok(s) => eprintln!("failed to open browser on {serial} (exit: {s})"),
        Err(e) => eprintln!("failed to run adb shell for {serial}: {e}"),
    }
}

fn setup_adb_reverse(serial: &str, port: i32) -> bool {
    let arg = format!("tcp:{port}");
    let status = Command::new(get_adb_path())
        .args(["-s", serial, "reverse", &arg, &arg])
        .status();
    match status {
        Ok(s) if s.success() => {
            println!("adb reverse set up for device {serial} on port {port}");
            true
        }
        Ok(s) => {
            eprintln!("adb reverse failed for {serial} (exit: {s})");
            false
        }
        Err(e) => {
            eprintln!("failed to run adb for {serial}: {e}");
            false
        }
    }
}

/// Detects newly connected ADB devices, sets up reverse tunnels, and returns
/// the ports that were successfully set up. Call this from a background thread
/// as it blocks while adb runs. Construct devices on the main thread from the ports.
pub fn get_new_adb_serials_and_setup_reverse() -> Vec<i32> {
    get_new_adb_serials()
        .iter()
        .filter_map(|serial| {
            let Some(port) = next_friendly_port() else {
                eprintln!("all ports exhausted, cannot create device for {serial}");
                return None;
            };
            if !setup_adb_reverse(serial, port) {
                return None;
            }
            open_browser_on_phone(serial, port);
            Some(port)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// HTTP input server
// ---------------------------------------------------------------------------

pub struct InputButtonStates {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    fast_attack: bool,
    strong_attack: bool,
    dash: bool,
    jump: bool,

    start_press_up: bool,
    start_press_down: bool,
    start_press_left: bool,
    start_press_right: bool,
    start_press_fast_attack: bool,
    start_press_strong_attack: bool,
    start_press_dash: bool,
    start_press_jump: bool,
}

impl Default for InputButtonStates {
    fn default() -> Self {
        Self {
            up: false, down: false, left: false, right: false,
            fast_attack: false, strong_attack: false, dash: false, jump: false,
            start_press_up: false, start_press_down: false,
            start_press_left: false, start_press_right: false,
            start_press_fast_attack: false, start_press_strong_attack: false,
            start_press_dash: false, start_press_jump: false,
        }
    }
}

pub struct HttpInputDevice {
    device_data: InputDeviceData,
    port: i32,
    server: Server,
    disconnected: bool,
    input_button_states: InputButtonStates,
}

/// Creates an HTTP server bound to 0.0.0.0:<port>. The phone reaches it via
/// the adb reverse tunnel at localhost:<port>.
pub fn create_http_input_server(port: i32) -> Box<dyn InputDevice> {
    let server = match Server::http(format!("0.0.0.0:{port}")) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to bind port {port}: {e}");
            return dummy_input();
        }
    };
    Box::new(HttpInputDevice {
        input_button_states: InputButtonStates::default(),
        server,
        port,
        disconnected: false,
        device_data: InputDeviceData { enabled: false, ready_to_play: false, score: 0 },
    })
}

impl InputDevice for HttpInputDevice {
    fn update(&mut self) {
        while let Ok(Some(req)) = self.server.try_recv() {
            match (req.method(), req.url()) {
                (tiny_http::Method::Get, "/") => {
                    let html = include_str!("../web/controller.html");
                    let response = tiny_http::Response::from_string(html)
                        .with_header("Content-Type: text/html".parse::<tiny_http::Header>().unwrap());
                    req.respond(response).unwrap();
                    continue;
                }
                (tiny_http::Method::Post, "/up/true")      => { if !self.input_button_states.up { self.input_button_states.start_press_up = true; } self.input_button_states.up = true; }
                (tiny_http::Method::Post, "/up/false")     => self.input_button_states.up = false,
                (tiny_http::Method::Post, "/down/true")    => { if !self.input_button_states.down { self.input_button_states.start_press_down = true; } self.input_button_states.down = true; }
                (tiny_http::Method::Post, "/down/false")   => self.input_button_states.down = false,
                (tiny_http::Method::Post, "/left/true")    => { if !self.input_button_states.left { self.input_button_states.start_press_left = true; } self.input_button_states.left = true; }
                (tiny_http::Method::Post, "/left/false")   => self.input_button_states.left = false,
                (tiny_http::Method::Post, "/right/true")   => { if !self.input_button_states.right { self.input_button_states.start_press_right = true; } self.input_button_states.right = true; }
                (tiny_http::Method::Post, "/right/false")  => self.input_button_states.right = false,
                (tiny_http::Method::Post, "/jump/true")    => { if !self.input_button_states.jump { self.input_button_states.start_press_jump = true; } self.input_button_states.jump = true; }
                (tiny_http::Method::Post, "/jump/false")   => self.input_button_states.jump = false,
                (tiny_http::Method::Post, "/fast/true")    => { if !self.input_button_states.fast_attack { self.input_button_states.start_press_fast_attack = true; } self.input_button_states.fast_attack = true; }
                (tiny_http::Method::Post, "/fast/false")   => self.input_button_states.fast_attack = false,
                (tiny_http::Method::Post, "/strong/true")  => { if !self.input_button_states.strong_attack { self.input_button_states.start_press_strong_attack = true; } self.input_button_states.strong_attack = true; }
                (tiny_http::Method::Post, "/strong/false") => self.input_button_states.strong_attack = false,
                (tiny_http::Method::Post, "/dash/true")    => { if !self.input_button_states.dash { self.input_button_states.start_press_dash = true; } self.input_button_states.dash = true; }
                (tiny_http::Method::Post, "/dash/false")   => self.input_button_states.dash = false,
                (tiny_http::Method::Post, "/disconnect")   => self.disconnected = true,
                _ => {}
            }
            req.respond(tiny_http::Response::empty(200)).unwrap();
        }
    }

    fn get_name(&mut self) -> String {
        format!("adb device: http://localhost:{}", self.port)
    }

    fn get_input_device_data(&mut self) -> &mut InputDeviceData {
        &mut self.device_data
    }

    fn get_input_device_data_ref(&self) -> &InputDeviceData {
        &self.device_data
    }

    fn is_disconnected(&self) -> bool {
        self.disconnected
    }

    fn get_id(&self) -> u64 {
        self.port as u64
    }

    fn should_begin_jump(&mut self) -> bool {
        let val = self.input_button_states.start_press_jump;
        self.input_button_states.start_press_jump = false;
        val
    }

    fn get_current_direction_left_right(&mut self) -> InputDirectionLeftRight {
        match (self.input_button_states.left, self.input_button_states.right) {
            (true, false) => InputDirectionLeftRight::Left,
            (false, true) => InputDirectionLeftRight::Right,
            _ => InputDirectionLeftRight::Neutral,
        }
    }

    fn get_current_direction_up_down(&mut self) -> InputDirectionUpDown {
        match (self.input_button_states.up, self.input_button_states.down) {
            (true, false) => InputDirectionUpDown::Up,
            (false, true) => InputDirectionUpDown::Down,
            _ => InputDirectionUpDown::Neutral,
        }
    }

    fn should_begin_dash(&mut self) -> bool {
        let val = self.input_button_states.start_press_dash;
        self.input_button_states.start_press_dash = false;
        val
    }

    fn should_begin_short_attack(&mut self) -> bool {
        let val = self.input_button_states.start_press_fast_attack;
        self.input_button_states.start_press_fast_attack = false;
        val
    }

    fn should_begin_long_attack(&mut self) -> bool {
        let val = self.input_button_states.start_press_strong_attack;
        self.input_button_states.start_press_strong_attack = false;
        val
    }

    fn is_jump_key_down(&mut self) -> bool {
        self.input_button_states.jump
    }

    fn should_begin_move_right(&mut self) -> bool {
        let val = self.input_button_states.start_press_right;
        self.input_button_states.start_press_right = false;
        val
    }

    fn should_begin_move_left(&mut self) -> bool {
        let val = self.input_button_states.start_press_left;
        self.input_button_states.start_press_left = false;
        val
    }

    fn get_fast_attack_keybind(&mut self) -> String { "fast attack".to_string() }
    fn get_jump_keybind(&mut self) -> String { "jump".to_string() }
    fn get_left_keybind(&mut self) -> String { "left".to_string() }
    fn get_right_keybind(&mut self) -> String { "right".to_string() }
    fn get_up_keybind(&mut self) -> String { "up".to_string() }
    fn get_down_keybind(&mut self) -> String { "down".to_string() }
    fn get_strong_attack_keybind(&mut self) -> String { "strong".to_string() }
}
