use crate::{material::Material, math::Vec3};

const EPS: f32 = 0.001;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct Intersect {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray: Ray) -> Option<Intersect>;
}

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
    pub burning_only: bool,
}

impl Cube {
    pub fn new(center: Vec3, size: Vec3, material: Material) -> Self {
        Self {
            min: center - size * 0.5,
            max: center + size * 0.5,
            material,
            burning_only: false,
        }
    }
    pub fn fire(center: Vec3, size: Vec3, material: Material) -> Self {
        Self {
            burning_only: true,
            ..Self::new(center, size, material)
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray: Ray) -> Option<Intersect> {
        // Cada eje define un intervalo; su solapamiento determina el tramo dentro del cubo.
        fn slab(origin: f32, direction: f32, min: f32, max: f32) -> Option<(f32, f32)> {
            if direction.abs() < 1e-8 {
                // Un rayo paralelo solo cruza este eje si ya está dentro de sus límites.
                if origin < min || origin > max {
                    None
                } else {
                    Some((f32::NEG_INFINITY, f32::INFINITY))
                }
            } else {
                let a = (min - origin) / direction;
                let b = (max - origin) / direction;
                Some((a.min(b), a.max(b)))
            }
        }
        let (nx, fx) = slab(ray.origin.x, ray.direction.x, self.min.x, self.max.x)?;
        let (ny, fy) = slab(ray.origin.y, ray.direction.y, self.min.y, self.max.y)?;
        let (nz, fz) = slab(ray.origin.z, ray.direction.z, self.min.z, self.max.z)?;
        let near = nx.max(ny).max(nz);
        let far = fx.min(fy).min(fz);
        if far < near.max(EPS) {
            return None;
        }
        let distance = if near > EPS { near } else { far };
        if distance < EPS {
            return None;
        }
        let point = ray.origin + ray.direction * distance;
        let normal = if (point.x - self.min.x).abs() < 0.002 {
            Vec3::new(-1., 0., 0.)
        } else if (point.x - self.max.x).abs() < 0.002 {
            Vec3::new(1., 0., 0.)
        } else if (point.y - self.min.y).abs() < 0.002 {
            Vec3::new(0., -1., 0.)
        } else if (point.y - self.max.y).abs() < 0.002 {
            Vec3::new(0., 1., 0.)
        } else if (point.z - self.min.z).abs() < 0.002 {
            Vec3::new(0., 0., -1.)
        } else {
            Vec3::new(0., 0., 1.)
        };
        Some(Intersect {
            distance,
            point,
            normal,
            material: self.material,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::painted;

    #[test]
    fn front_ray_hits_cube_at_expected_distance() {
        let cube = Cube::new(
            Vec3::new(0.0, 0.0, -5.0),
            Vec3::splat(2.0),
            painted(Vec3::splat(1.0)),
        );
        let hit = cube
            .ray_intersect(Ray {
                origin: Vec3::default(),
                direction: Vec3::new(0.0, 0.0, -1.0),
            })
            .expect("the ray should hit the cube");
        assert!((hit.distance - 4.0).abs() < 1e-4);
        assert!(hit.normal.z > 0.9);
    }

    #[test]
    fn ray_can_miss_cube() {
        let cube = Cube::new(
            Vec3::new(0.0, 0.0, -5.0),
            Vec3::splat(2.0),
            painted(Vec3::splat(1.0)),
        );
        assert!(cube
            .ray_intersect(Ray {
                origin: Vec3::default(),
                direction: Vec3::new(0.0, 1.0, 0.0),
            })
            .is_none());
    }
}
