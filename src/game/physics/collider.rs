use nalgebra::Point3;

#[derive(Debug)]
pub struct BoxCollider {
    pub p1: Point3<f32>,
    pub p2: Point3<f32>,
    pub center: Point3<f32>,
}

impl BoxCollider {
    pub fn blank() -> BoxCollider {
        BoxCollider {
            p1: Point3::new(0.0, 0.0, 0.0),
            p2: Point3::new(0.0, 0.0, 0.0),
            center: Point3::new(0.0, 0.0, 0.0),
        }
    }
}
