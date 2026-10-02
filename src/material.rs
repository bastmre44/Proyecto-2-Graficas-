use crate::math::{hash2, Vec3};

#[derive(Clone, Copy, Debug)]
pub enum TextureKind {
    Painted,
    Wallpaper,
    Wood,
    Metal,
    Glass,
    Fabric,
    Fire,
    Charred,
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub albedo: Vec3,
    pub specular: f32,
    pub shininess: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub ior: f32,
    pub emission: Vec3,
    pub texture: TextureKind,
}

impl Material {
    pub fn sample(self, p: Vec3, n: Vec3) -> Vec3 {
        let (u, v) = if n.x.abs() > 0.5 {
            (p.z, p.y)
        } else if n.y.abs() > 0.5 {
            (p.x, p.z)
        } else {
            (p.x, p.y)
        };
        let pattern = match self.texture {
            TextureKind::Painted => 0.9 + 0.1 * hash2((u * 18.0) as i32, (v * 18.0) as i32),
            TextureKind::Wallpaper => {
                let stripe = ((u * 4.0).sin().abs() * 0.16) + ((v * 8.0).sin().abs() * 0.05);
                0.83 + stripe
            }
            TextureKind::Wood => {
                let grain = ((u * 15.0 + (v * 2.5).sin()).sin() * 0.5 + 0.5) * 0.22;
                0.73 + grain
            }
            TextureKind::Metal => {
                let grid = if (u * 8.0).fract().abs() < 0.04 || (v * 8.0).fract().abs() < 0.04 {
                    0.66
                } else {
                    1.0
                };
                grid
            }
            TextureKind::Glass => 0.75 + 0.25 * ((u + v) * 9.0).sin().abs(),
            TextureKind::Fabric => {
                let weave = ((u * 30.0).sin() * (v * 30.0).sin()).abs();
                0.78 + 0.2 * weave
            }
            TextureKind::Fire => 0.86 + 0.14 * hash2((u * 11.0) as i32, (v * 15.0) as i32),
            TextureKind::Charred => 0.35 + 0.35 * hash2((u * 8.0) as i32, (v * 8.0) as i32),
        };
        self.albedo * pattern
    }
}

pub fn painted(color: Vec3) -> Material {
    Material {
        albedo: color,
        specular: 0.15,
        shininess: 20.0,
        transparency: 0.0,
        reflectivity: 0.04,
        ior: 1.0,
        emission: Vec3::default(),
        texture: TextureKind::Painted,
    }
}
pub fn wallpaper(color: Vec3) -> Material {
    Material {
        albedo: color,
        specular: 0.10,
        shininess: 12.0,
        transparency: 0.0,
        reflectivity: 0.02,
        ior: 1.0,
        emission: Vec3::default(),
        texture: TextureKind::Wallpaper,
    }
}
pub fn wood(color: Vec3) -> Material {
    Material {
        albedo: color,
        specular: 0.24,
        shininess: 30.0,
        transparency: 0.0,
        reflectivity: 0.08,
        ior: 1.0,
        emission: Vec3::default(),
        texture: TextureKind::Wood,
    }
}
pub fn metal(color: Vec3) -> Material {
    Material {
        albedo: color,
        specular: 0.9,
        shininess: 110.0,
        transparency: 0.0,
        reflectivity: 0.58,
        ior: 1.0,
        emission: Vec3::default(),
        texture: TextureKind::Metal,
    }
}
pub fn glass(color: Vec3) -> Material {
    Material {
        albedo: color,
        specular: 0.85,
        shininess: 140.0,
        transparency: 0.78,
        reflectivity: 0.16,
        ior: 1.52,
        emission: Vec3::default(),
        texture: TextureKind::Glass,
    }
}
pub fn fabric(color: Vec3) -> Material {
    Material {
        albedo: color,
        specular: 0.12,
        shininess: 14.0,
        transparency: 0.0,
        reflectivity: 0.02,
        ior: 1.0,
        emission: Vec3::default(),
        texture: TextureKind::Fabric,
    }
}
pub fn fire(color: Vec3, emission: Vec3) -> Material {
    Material {
        albedo: color,
        specular: 0.0,
        shininess: 1.0,
        transparency: 0.05,
        reflectivity: 0.0,
        ior: 1.0,
        emission,
        texture: TextureKind::Fire,
    }
}
pub fn charred() -> Material {
    Material {
        albedo: Vec3::new(0.12, 0.08, 0.07),
        specular: 0.04,
        shininess: 5.0,
        transparency: 0.0,
        reflectivity: 0.0,
        ior: 1.0,
        emission: Vec3::default(),
        texture: TextureKind::Charred,
    }
}
