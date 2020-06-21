use winit::dpi::PhysicalSize;
use nalgebra::{Vector3, Matrix4, Perspective3, Isometry3, Point3};

pub struct Camera {
    pub eye: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32
}

const FIRST_PERSON_OFFSET: [f32; 3] = [0.0, 1.0, 0.0];
const PI: f32 = std::f32::consts::PI;

impl Camera {

    pub fn new(size: &PhysicalSize<u32>) -> Camera {
        Camera {
            // position the camera one unit up and 2 units back
            eye: Point3::new(10.0, 51.0, 10.0),
            // have it look at the origin
            yaw: 00.0,
            pitch: 0.0,
            // which way is "up"
            up: Vector3::y(),
            aspect: size.width as f32 / size.height as f32,
            fovy: 70.0,
            znear: 0.1,
            zfar: 2000.0
        }
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {

        let opengl_to_wgpu_matrix: Matrix4<f32> = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        );

        let view_vector = Vector3::new(
            ((self.yaw - PI / 2.0).cos() * self.pitch.cos()) as f32,
            self.pitch.sin() as f32,
            (-(self.yaw - PI / 2.0).sin() * -self.pitch.cos()) as f32
        );

        // No look_at_direction function so I need to do this grr
        let target = Point3::from(view_vector) + self.eye.coords;

        let view = Isometry3::look_at_rh(&self.eye, &target, &Vector3::y());

        let proj = Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);

        return (opengl_to_wgpu_matrix * proj.as_matrix()) * view.to_homogeneous();
    }

    pub fn move_first_person(&mut self, pos: &Point3<f32>) {
        let x = pos.x + FIRST_PERSON_OFFSET[0];
        let y = pos.y + FIRST_PERSON_OFFSET[1];
        let z = pos.z + FIRST_PERSON_OFFSET[2];
        self.eye = Point3::new(x, y, z);
    }
}