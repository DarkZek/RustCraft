use nalgebra::Vector2;

/// Stores the way an object should present itself in space
#[derive(Clone, Copy)]
pub struct Layout {
    pub size: Vector2<f32>,
    pub offset: Vector2<f32>,
    pub scheme: LayoutScheme,
    pub padding: f32,
}

impl Layout {
    pub fn new(
        size: Vector2<f32>,
        offset: Vector2<f32>,
        scheme: LayoutScheme,
        padding: f32,
    ) -> Layout {
        Layout {
            size,
            offset,
            scheme,
            padding,
        }
    }
}

impl Layout {
    /// Calculates the position of a `Layout` object inside a parent
    pub fn position_object(&self, parent: &Layout) -> Vector2<f32> {
        match self.scheme {
            LayoutScheme::TopLeft => {
                Vector2::new(self.offset.x + self.padding, self.offset.y + self.padding)
            }
            LayoutScheme::Left => {
                let middle_y = (parent.size.y / 2.0) - (self.size.y / 2.0);
                Vector2::new(self.offset.x + self.padding, middle_y + self.offset.y)
            }
            LayoutScheme::BottomLeft => Vector2::new(
                self.offset.x + self.padding,
                parent.size.y - self.size.y + self.offset.y - self.padding,
            ),
            LayoutScheme::Center => {
                let middle_x = (parent.size.x / 2.0) - (self.size.x / 2.0);
                let middle_y = (parent.size.y / 2.0) - (self.size.y / 2.0);
                Vector2::new(middle_x + self.offset.x, middle_y + self.offset.y)
            }
            LayoutScheme::TopRight => Vector2::new(
                parent.size.x - self.size.x + self.offset.x - self.padding,
                self.offset.y + self.padding,
            ),
            LayoutScheme::Right => {
                let middle_y = (parent.size.y / 2.0) - (self.size.y / 2.0 - self.padding);
                Vector2::new(
                    parent.size.x - self.size.x + self.offset.x,
                    middle_y + self.offset.y,
                )
            }
            LayoutScheme::BottomRight => Vector2::new(
                parent.size.x - self.size.x + self.offset.x - self.padding,
                parent.size.y - self.size.y + self.offset.y - self.padding,
            ),
            LayoutScheme::Top => {
                let middle_x = (parent.size.x / 2.0) - (self.size.x / 2.0);
                Vector2::new(middle_x + self.offset.x, self.offset.y + self.padding)
            }
            LayoutScheme::Bottom => {
                let middle_x = (parent.size.x / 2.0) - (self.size.x / 2.0);
                Vector2::new(
                    middle_x + self.offset.x,
                    parent.size.y - self.size.y + self.offset.y - self.padding,
                )
            }
        }
    }
}

/// Stores the scheme the object uses for positioning
#[derive(Clone, Copy)]
pub enum LayoutScheme {
    TopLeft,
    Left,
    BottomLeft,
    Center,
    TopRight,
    Right,
    BottomRight,
    Top,
    Bottom,
}
