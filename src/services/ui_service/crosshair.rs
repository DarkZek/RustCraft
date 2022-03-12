use rc_ui::component::UIComponent;

pub struct CrosshairComponent {}

impl CrosshairComponent {
    pub fn new() -> CrosshairComponent {
        CrosshairComponent {}
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
