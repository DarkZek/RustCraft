use nalgebra::Vector2;

pub struct Layout {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub offset: Vector2<f32>,
    pub scheme: LayoutScheme,
}

impl Layout {
    pub fn new(
        position: Vector2<f32>,
        size: Vector2<f32>,
        offset: Vector2<f32>,
        scheme: LayoutScheme,
    ) -> Layout {
        Layout {
            position,
            size,
            offset,
            scheme,
        }
    }
}

impl Layout {
    pub fn position_object(&self, parent: &Layout) -> Vector2<f32> {
        match self.scheme {
            LayoutScheme::TopLeft => self.offset,
            LayoutScheme::Left => {
                let middle_y = (parent.size.y / 2.0) - (self.size.y / 2.0);
                Vector2::new(self.offset.x, middle_y + self.offset.y)
            }
            LayoutScheme::BottomLeft => {
                Vector2::new(self.offset.x, parent.size.y - self.size.y + self.offset.y)
            }
            LayoutScheme::Center => {
                let middle_x = (parent.size.x / 2.0) - (self.size.x / 2.0);
                let middle_y = (parent.size.y / 2.0) - (self.size.y / 2.0);
                Vector2::new(middle_x + self.offset.x, middle_y + self.offset.y)
            }
            LayoutScheme::TopRight => {
                Vector2::new(parent.size.x - self.size.x + self.offset.x, self.offset.y)
            }
            LayoutScheme::Right => {
                let middle_y = (parent.size.y / 2.0) - (self.size.y / 2.0);
                Vector2::new(
                    parent.size.x - self.size.x + self.offset.x,
                    middle_y + self.offset.y,
                )
            }
            LayoutScheme::BottomRight => Vector2::new(
                parent.size.x - self.size.x + self.offset.x,
                parent.size.y - self.size.y + self.offset.y,
            ),
        }
    }

    fn position_object_padding(&self, parent: &Layout, scheme: LayoutScheme, padding: f32) {
        todo!()
    }
}

pub enum LayoutScheme {
    TopLeft,
    Left,
    BottomLeft,
    Center,
    TopRight,
    Right,
    BottomRight,
}
