use crate::UIController;
use nalgebra::Vector2;

impl UIController {
    pub fn cursor_moved(&mut self, position: Vector2<f32>) {
        for component in &mut self.components {
            for element in &mut component.objects {
                let (pos, size) = element.position();
                if (position.x > pos.x && position.x < pos.x + size.x)
                    && (position.y > pos.y && position.y < pos.y + size.y)
                {
                    if element.hovered(true) {
                        // Made change, is dirty
                        component.regenerate = true;
                    }
                }
            }
        }
    }
}
