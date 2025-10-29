use crate::scenes::input_select::InputSelect;



pub enum Scene {
    InputSelectScene(InputSelect)
}


impl Scene {
    pub fn render(&mut self) {
        match self {
            Scene::InputSelectScene(input_select) => {
                input_select.render();
            },
        }
    }

    pub fn change_scene(&mut self) -> Option<Scene> {
        return None;
    }
}