use winit::dpi::PhysicalSize;
use cgmath::{Vector3, Point3};

pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
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
            eye: Point3 {x: 10.0, y: 51.0, z: 10.0},
            // have it look at the origin
            yaw: 00.0,
            pitch: 0.0,
            // which way is "up"
            up: Vector3::unit_y(),
            aspect: size.width as f32 / size.height as f32,
            fovy: 70.0,
            znear: 0.1,
            zfar: 2000.0
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view_vector = Vector3 {
            x: ((self.yaw - PI / 2.0).cos() * self.pitch.cos()) as f32,
            y: self.pitch.sin() as f32,
            z: (-(self.yaw - PI / 2.0).sin() * -self.pitch.cos()) as f32
        };

        let view = cgmath::Matrix4::look_at_dir(self.eye, view_vector, self.up);

        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn move_first_person(&mut self, pos: &Vector3<f32>) {
        let x = pos.x + FIRST_PERSON_OFFSET[0];
        let y = pos.y + FIRST_PERSON_OFFSET[1];
        let z = pos.z + FIRST_PERSON_OFFSET[2];
        self.eye = (x, y, z).into();
    }
}