use crate::{geometry::Ray, math::Vec3};

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub target: Vec3,
    pub fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.18,
            distance: 28.0,
            target: Vec3::new(0.0, 3.7, 2.2),
            fov: 52_f32.to_radians(),
        }
    }
    pub fn position(&self) -> Vec3 {
        let cp = self.pitch.cos();
        self.target
            + Vec3::new(self.yaw.sin() * cp, self.pitch.sin(), self.yaw.cos() * cp) * self.distance
    }
    pub fn ray(&self, x: usize, y: usize, w: usize, h: usize) -> Ray {
        let pos = self.position();
        let forward = (self.target - pos).normalized();
        let right = forward.cross(Vec3::new(0., 1., 0.)).normalized();
        let up = right.cross(forward).normalized();
        let aspect = w as f32 / h as f32;
        let scale = (self.fov * 0.5).tan();
        let sx = (2.0 * (x as f32 + 0.5) / w as f32 - 1.0) * aspect * scale;
        let sy = (1.0 - 2.0 * (y as f32 + 0.5) / h as f32) * scale;
        Ray {
            origin: pos,
            direction: (forward + right * sx + up * sy).normalized(),
        }
    }
}
