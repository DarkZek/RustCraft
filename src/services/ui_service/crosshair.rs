use rc_ui::component::UIComponent;
use std::sync::{Arc, Mutex};

pub struct CrosshairComponent {}

impl CrosshairComponent {
    pub fn new() -> Arc<Mutex<CrosshairComponent>> {
        let component = CrosshairComponent {};

        Arc::new(Mutex::new(component))
    }
}

impl UIComponent for CrosshairComponent {
    fn render(&self) {
        todo!()
    }

    fn rerender(&self) -> bool {
        false
    }
}
