use crate::positioning::Layout;
use crate::UIController;
use nalgebra::Vector2;

impl UIController {
    pub fn cursor_moved(&mut self, mut position: Vector2<f32>) {
        // Normalize the cursor position to the size of the ui
        let current_size = self.pipeline.layout.size;
        position.x *= (current_size.x / self.screen_size.width as f32);
        position.y *= (current_size.y / self.screen_size.height as f32);

        for component in &mut self.components {
            // No click event for invisible components
            if !component.data.lock().unwrap().visible() {
                continue;
            }

            let component_position = component
                .data
                .lock()
                .unwrap()
                .positioning()
                .position_object(&self.pipeline.layout);

            for element in &mut component.objects {
                let (pos, size) = element.data.position();
                let pos = pos + component_position;

                // If element is currently hovered, and the state is hovered them update the component and re-render
                if (position.x > pos.x && position.x < pos.x + size.x)
                    && (position.y > pos.y && position.y < pos.y + size.y)
                {
                    if !element.hovered && element.data.hovered(true) {
                        // Made change, is dirty
                        component.dirty = true;
                    }

                    // Found hovered object, we only want one hovered object so return
                    element.hovered = true;
                    return;
                } else {
                    // If element is not currently hovered, and the state is hovered them update the component and re-render
                    if element.hovered && element.data.hovered(false) {
                        // Made change, is dirty
                        component.dirty = true;
                    }
                    element.hovered = false;
                }
            }
        }
    }
}
