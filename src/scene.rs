use crate::{geometry::Cube, material::*, math::Vec3};

#[derive(Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub burning_only: bool,
}

pub struct Scene {
    pub cubes: Vec<Cube>,
    pub lights: Vec<Light>,
}

fn add_lamp(cubes: &mut Vec<Cube>, x: f32, y: f32, color: Vec3) {
    cubes.push(Cube::new(
        Vec3::new(x, y, 0.72),
        Vec3::new(0.12, 0.8, 0.12),
        metal(Vec3::new(0.72, 0.55, 0.15)),
    ));
    cubes.push(Cube::new(
        Vec3::new(x, y + 0.45, 0.72),
        Vec3::new(0.62, 0.5, 0.5),
        fabric(color),
    ));
}

fn add_curtains(cubes: &mut Vec<Cube>, x: f32, y: f32, color: Vec3) {
    cubes.push(Cube::new(
        Vec3::new(x - 1.0, y, 0.76),
        Vec3::new(0.42, 1.85, 0.18),
        fabric(color),
    ));
    cubes.push(Cube::new(
        Vec3::new(x + 1.0, y, 0.76),
        Vec3::new(0.42, 1.85, 0.18),
        fabric(color),
    ));
    cubes.push(Cube::new(
        Vec3::new(x, y + 0.92, 0.76),
        Vec3::new(2.4, 0.18, 0.18),
        wood(color * 0.7),
    ));
}

fn add_tree(cubes: &mut Vec<Cube>, x: f32, z: f32, height: f32) {
    let trunk = wood(Vec3::new(0.28, 0.12, 0.045));
    let leaves = wallpaper(Vec3::new(0.055, 0.28, 0.09));
    cubes.push(Cube::new(
        Vec3::new(x, height * 0.38, z),
        Vec3::new(0.75, height * 0.76, 0.75),
        trunk,
    ));
    cubes.push(Cube::new(
        Vec3::new(x, height * 0.82, z),
        Vec3::new(3.1, 1.5, 3.1),
        leaves,
    ));
    cubes.push(Cube::new(
        Vec3::new(x - 0.65, height * 1.03, z + 0.25),
        Vec3::new(2.1, 1.25, 2.1),
        leaves,
    ));
    cubes.push(Cube::new(
        Vec3::new(x + 0.65, height * 1.03, z - 0.25),
        Vec3::new(2.1, 1.25, 2.1),
        leaves,
    ));
    cubes.push(Cube::new(
        Vec3::new(x, height * 1.24, z),
        Vec3::new(1.55, 1.05, 1.55),
        leaves,
    ));
}

fn add_lantern(cubes: &mut Vec<Cube>, lights: &mut Vec<Light>, x: f32, z: f32) {
    let dark = metal(Vec3::new(0.06, 0.055, 0.07));
    cubes.push(Cube::new(
        Vec3::new(x, 1.05, z),
        Vec3::new(0.16, 2.25, 0.16),
        dark,
    ));
    cubes.push(Cube::new(
        Vec3::new(x, 2.18, z),
        Vec3::new(0.62, 0.62, 0.62),
        glass(Vec3::new(1.0, 0.78, 0.32)),
    ));
    cubes.push(Cube::new(
        Vec3::new(x, 2.55, z),
        Vec3::new(0.78, 0.12, 0.78),
        dark,
    ));
    lights.push(Light {
        position: Vec3::new(x, 2.2, z),
        color: Vec3::new(1.0, 0.55, 0.16),
        intensity: 16.0,
        burning_only: false,
    });
}

impl Scene {
    pub fn lover_house() -> Self {
        let mut cubes = Vec::new();
        let mut lights = Vec::new();
        let black = metal(Vec3::new(0.055, 0.045, 0.065));
        // Terreno
        cubes.push(Cube::new(
            Vec3::new(0.0, -1.05, 3.8),
            Vec3::new(25.0, 1.4, 18.0),
            wood(Vec3::new(0.20, 0.10, 0.045)),
        ));
        cubes.push(Cube::new(
            Vec3::new(0.0, -0.30, 3.8),
            Vec3::new(24.2, 0.22, 17.2),
            wallpaper(Vec3::new(0.08, 0.38, 0.11)),
        ));
        cubes.push(Cube::new(
            Vec3::new(0., -0.10, 0.),
            Vec3::new(15.4, 0.32, 4.2),
            metal(Vec3::new(0.08, 0.07, 0.10)),
        ));

        // Camino
        for row in 0..7 {
            for side in [-0.55_f32, 0.55_f32] {
                let z = 3.0 + row as f32 * 1.25;
                cubes.push(Cube::new(
                    Vec3::new(side, -0.08, z),
                    Vec3::new(0.95, 0.18, 0.9),
                    painted(Vec3::new(0.38, 0.37, 0.43)),
                ));
            }
        }
        for &(x, z) in &[
            (-2.2, 3.2),
            (2.2, 3.2),
            (-3.0, 5.0),
            (3.0, 5.0),
            (-3.6, 7.3),
            (3.6, 7.3),
        ] {
            cubes.push(Cube::new(
                Vec3::new(x, -0.09, z),
                Vec3::new(1.1, 0.16, 0.72),
                painted(Vec3::new(0.31, 0.30, 0.36)),
            ));
        }

        // Estanque
        cubes.push(Cube::new(
            Vec3::new(-7.1, -0.12, 6.2),
            Vec3::new(5.7, 0.16, 4.0),
            glass(Vec3::new(0.08, 0.48, 0.72)),
        ));
        for x in [-9.9, -8.5, -7.1, -5.7, -4.3] {
            for z in [4.05, 8.35] {
                cubes.push(Cube::new(
                    Vec3::new(x, -0.02, z),
                    Vec3::new(1.15, 0.42, 0.62),
                    painted(Vec3::new(0.34, 0.35, 0.39)),
                ));
            }
        }
        for z in [5.0, 6.2, 7.4] {
            for x in [-10.05, -4.15] {
                cubes.push(Cube::new(
                    Vec3::new(x, -0.02, z),
                    Vec3::new(0.62, 0.42, 0.95),
                    painted(Vec3::new(0.34, 0.35, 0.39)),
                ));
            }
        }
        for &(x, z) in &[(-8.4, 5.4), (-6.7, 6.5), (-8.0, 7.2), (-5.6, 5.2)] {
            cubes.push(Cube::new(
                Vec3::new(x, 0.02, z),
                Vec3::new(0.85, 0.08, 0.65),
                wallpaper(Vec3::new(0.20, 0.62, 0.16)),
            ));
        }

        // Jardin
        add_tree(&mut cubes, -10.0, 0.0, 3.8);
        add_tree(&mut cubes, 9.7, 1.1, 4.2);
        add_tree(&mut cubes, 9.4, 8.4, 3.5);
        for &(x, z) in &[
            (-9.5, 10.1),
            (-6.8, 10.4),
            (-3.8, 10.2),
            (3.8, 10.2),
            (6.8, 10.4),
            (9.5, 10.1),
            (-10.3, 3.0),
            (10.3, 4.4),
        ] {
            cubes.push(Cube::new(
                Vec3::new(x, 0.45, z),
                Vec3::new(1.7, 1.0, 1.25),
                wallpaper(Vec3::new(0.06, 0.32, 0.08)),
            ));
        }
        for i in 0..22 {
            let x = -10.0 + (i % 11) as f32 * 2.0;
            let z = 2.2 + (i / 11) as f32 * 7.6;
            let color = if i % 3 == 0 {
                Vec3::new(0.95, 0.18, 0.45)
            } else if i % 3 == 1 {
                Vec3::new(1.0, 0.72, 0.12)
            } else {
                Vec3::new(0.72, 0.45, 1.0)
            };
            cubes.push(Cube::new(
                Vec3::new(x, 0.18, z),
                Vec3::new(0.24, 0.52, 0.24),
                painted(color),
            ));
        }

        // Faroles
        add_lantern(&mut cubes, &mut lights, -2.3, 4.2);
        add_lantern(&mut cubes, &mut lights, 2.3, 4.2);
        add_lantern(&mut cubes, &mut lights, -2.3, 8.0);
        add_lantern(&mut cubes, &mut lights, 2.3, 8.0);

        // Estructura
        cubes.push(Cube::new(
            Vec3::new(0., 9.0, 0.75),
            Vec3::new(15.0, 0.35, 2.7),
            black,
        ));
        for y in [0.0, 3.0, 6.0, 9.0] {
            cubes.push(Cube::new(
                Vec3::new(0., y, 0.75),
                Vec3::new(15.0, 0.18, 2.65),
                black,
            ));
        }
        for x in [-7.5, -2.55, 2.55, 7.5] {
            cubes.push(Cube::new(
                Vec3::new(x, 4.5, 0.75),
                Vec3::new(0.18, 9.0, 2.65),
                black,
            ));
        }
        cubes.push(Cube::new(
            Vec3::new(-7.42, 4.5, -0.45),
            Vec3::new(0.18, 9.0, 2.5),
            black,
        ));
        cubes.push(Cube::new(
            Vec3::new(7.42, 4.5, -0.45),
            Vec3::new(0.18, 9.0, 2.5),
            black,
        ));
        cubes.push(Cube::new(
            Vec3::new(0.0, 4.5, -1.65),
            Vec3::new(15.0, 9.0, 0.16),
            wallpaper(Vec3::new(0.12, 0.09, 0.15)),
        ));

        // Fachada posterior
        for &(x, y, c) in &[
            (-5.0, 6.8, Vec3::new(0.18, 0.58, 0.86)),
            (0.0, 6.8, Vec3::new(0.78, 0.26, 0.62)),
            (5.0, 6.8, Vec3::new(0.88, 0.22, 0.32)),
            (-5.0, 3.4, Vec3::new(0.92, 0.58, 0.12)),
            (5.0, 3.4, Vec3::new(0.12, 0.34, 0.82)),
        ] {
            cubes.push(Cube::new(
                Vec3::new(x, y, -1.78),
                Vec3::new(1.6, 1.45, 0.08),
                glass(c),
            ));
            cubes.push(Cube::new(
                Vec3::new(x, y + 0.78, -1.84),
                Vec3::new(1.9, 0.12, 0.12),
                black,
            ));
            cubes.push(Cube::new(
                Vec3::new(x, y - 0.78, -1.84),
                Vec3::new(1.9, 0.12, 0.12),
                black,
            ));
            cubes.push(Cube::new(
                Vec3::new(x - 0.92, y, -1.84),
                Vec3::new(0.12, 1.68, 0.12),
                black,
            ));
            cubes.push(Cube::new(
                Vec3::new(x + 0.92, y, -1.84),
                Vec3::new(0.12, 1.68, 0.12),
                black,
            ));
        }
        cubes.push(Cube::new(
            Vec3::new(0.0, 1.25, -1.80),
            Vec3::new(1.55, 2.5, 0.12),
            wood(Vec3::new(0.58, 0.03, 0.05)),
        ));
        cubes.push(Cube::new(
            Vec3::new(0.0, 4.15, -2.25),
            Vec3::new(5.4, 0.18, 1.05),
            wood(Vec3::new(0.24, 0.13, 0.08)),
        ));
        for x in [-2.5, -1.25, 0.0, 1.25, 2.5] {
            cubes.push(Cube::new(
                Vec3::new(x, 4.8, -2.63),
                Vec3::new(0.10, 1.4, 0.10),
                metal(Vec3::new(0.12, 0.10, 0.14)),
            ));
        }
        for &(x, y) in &[(-6.7, 2.0), (-6.1, 4.1), (6.7, 5.1), (6.2, 7.2)] {
            cubes.push(Cube::new(
                Vec3::new(x, y, -1.82),
                Vec3::new(0.65, 1.7, 0.12),
                wallpaper(Vec3::new(0.05, 0.30, 0.08)),
            ));
        }
        // Habitaciones
        let colors = [
            Vec3::new(0.08, 0.55, 0.82),
            Vec3::new(0.18, 0.17, 0.22),
            Vec3::new(0.92, 0.16, 0.62),
            Vec3::new(0.95, 0.44, 0.06),
            Vec3::new(0.77, 0.78, 0.78),
            Vec3::new(0.73, 0.04, 0.10),
            Vec3::new(0.10, 0.48, 0.17),
            Vec3::new(0.33, 0.31, 0.34),
            Vec3::new(0.04, 0.20, 0.68),
        ];
        for row in 0..3 {
            for col in 0..3 {
                let i = row * 3 + col;
                let x = -5.0 + col as f32 * 5.0;
                let y = 7.5 - row as f32 * 3.0;
                let mat = wallpaper(colors[i]);
                cubes.push(Cube::new(
                    Vec3::new(x, y, -0.48),
                    Vec3::new(4.82, 2.82, 0.16),
                    mat,
                ));
                cubes.push(Cube::new(
                    Vec3::new(x, y - 1.38, 0.2),
                    Vec3::new(4.82, 0.12, 1.45),
                    wood(colors[i] * 0.72),
                ));
            }
        }
        // Entrada
        cubes.push(Cube::new(
            Vec3::new(0., 1.35, 1.05),
            Vec3::new(1.45, 2.55, 0.16),
            wood(Vec3::new(0.72, 0.025, 0.035)),
        ));
        cubes.push(Cube::new(
            Vec3::new(0.47, 1.3, 0.92),
            Vec3::new(0.10, 0.10, 0.12),
            metal(Vec3::new(0.92, 0.64, 0.10)),
        ));
        // Muebles
        let furniture = [
            (-5., 7.0, Vec3::new(0.90, 0.72, 0.48)),
            (5., 7.0, Vec3::new(0.95, 0.46, 0.73)),
            (-5., 4.0, Vec3::new(0.94, 0.75, 0.28)),
            (5., 4.0, Vec3::new(0.52, 0.04, 0.08)),
            (-5., 1.0, Vec3::new(0.05, 0.24, 0.10)),
            (5., 1.0, Vec3::new(0.10, 0.24, 0.66)),
        ];
        for (x, y, c) in furniture {
            cubes.push(Cube::new(
                Vec3::new(x, y - 0.45, 0.48),
                Vec3::new(2.65, 0.55, 1.05),
                fabric(c),
            ));
            cubes.push(Cube::new(
                Vec3::new(x, y - 0.05, 0.12),
                Vec3::new(2.65, 0.75, 0.18),
                fabric(c * 0.82),
            ));
            cubes.push(Cube::new(
                Vec3::new(x - 1.65, y - 0.72, 0.55),
                Vec3::new(0.65, 0.75, 0.85),
                wood(Vec3::new(0.42, 0.22, 0.10)),
            ));
        }

        // Cortinas y alfombras
        add_curtains(&mut cubes, -5.0, 7.45, Vec3::new(0.78, 0.90, 1.0));
        add_curtains(&mut cubes, 5.0, 7.45, Vec3::new(1.0, 0.73, 0.90));
        add_curtains(&mut cubes, -5.0, 4.45, Vec3::new(1.0, 0.78, 0.30));
        add_curtains(&mut cubes, 5.0, 4.45, Vec3::new(0.48, 0.02, 0.06));
        add_curtains(&mut cubes, -5.0, 1.45, Vec3::new(0.18, 0.58, 0.24));
        add_curtains(&mut cubes, 5.0, 1.45, Vec3::new(0.20, 0.35, 0.92));
        for &(x, y, c) in &[
            (-5.0, 6.25, Vec3::new(0.35, 0.72, 0.95)),
            (5.0, 6.25, Vec3::new(0.90, 0.30, 0.70)),
            (-5.0, 3.25, Vec3::new(0.95, 0.58, 0.12)),
            (5.0, 3.25, Vec3::new(0.55, 0.03, 0.08)),
            (-5.0, 0.25, Vec3::new(0.08, 0.34, 0.14)),
            (5.0, 0.25, Vec3::new(0.08, 0.22, 0.68)),
        ] {
            cubes.push(Cube::new(
                Vec3::new(x, y, 0.74),
                Vec3::new(2.8, 0.08, 1.05),
                fabric(c),
            ));
        }

        // Decoracion
        add_lamp(&mut cubes, -6.55, 7.0, Vec3::new(0.92, 0.92, 0.78));
        add_lamp(&mut cubes, 6.55, 7.0, Vec3::new(1.0, 0.82, 0.35));
        add_lamp(&mut cubes, -6.55, 4.0, Vec3::new(1.0, 0.82, 0.28));
        add_lamp(&mut cubes, 6.55, 4.0, Vec3::new(0.95, 0.80, 0.55));
        add_lamp(&mut cubes, -6.55, 1.0, Vec3::new(0.92, 0.86, 0.32));
        add_lamp(&mut cubes, 6.55, 1.0, Vec3::new(0.95, 0.76, 0.25));

        // Escalera
        for i in 0..7 {
            cubes.push(Cube::new(
                Vec3::new(-6.2 + i as f32 * 0.34, 0.35 + i as f32 * 0.27, 0.86),
                Vec3::new(0.38, 0.16, 0.7),
                wood(Vec3::new(0.17, 0.38, 0.13)),
            ));
        }
        cubes.push(Cube::new(
            Vec3::new(0.0, 4.5, 0.82),
            Vec3::new(0.13, 4.7, 0.13),
            metal(Vec3::new(0.68, 0.70, 0.72)),
        ));
        cubes.push(Cube::new(
            Vec3::new(0.7, 4.5, 0.82),
            Vec3::new(0.13, 4.7, 0.13),
            metal(Vec3::new(0.68, 0.70, 0.72)),
        ));
        for i in 0..9 {
            cubes.push(Cube::new(
                Vec3::new(0.35, 2.55 + i as f32 * 0.48, 0.82),
                Vec3::new(0.82, 0.10, 0.12),
                metal(Vec3::new(0.68, 0.70, 0.72)),
            ));
        }

        // Libros
        cubes.push(Cube::new(
            Vec3::new(0., 6.92, 0.05),
            Vec3::new(3.2, 0.12, 0.75),
            wood(Vec3::new(0.34, 0.18, 0.10)),
        ));
        for i in 0..7 {
            let c = [
                Vec3::new(0.8, 0.12, 0.18),
                Vec3::new(0.12, 0.45, 0.8),
                Vec3::new(0.92, 0.55, 0.08),
            ][i % 3];
            cubes.push(Cube::new(
                Vec3::new(-1.25 + i as f32 * 0.42, 7.25, 0.06),
                Vec3::new(0.25, 0.55, 0.55),
                painted(c),
            ));
        }
        // Ventanas
        for &(x, y) in &[
            (-5., 7.65),
            (5., 7.65),
            (-5., 4.65),
            (5., 4.65),
            (-5., 1.65),
            (5., 1.65),
        ] {
            cubes.push(Cube::new(
                Vec3::new(x, y, 0.63),
                Vec3::new(1.38, 1.05, 0.07),
                glass(Vec3::new(0.70, 0.88, 1.0)),
            ));
            for dx in [-0.76, 0.76] {
                cubes.push(Cube::new(
                    Vec3::new(x + dx, y, 0.69),
                    Vec3::new(0.08, 1.28, 0.08),
                    black,
                ));
            }
            for dy in [-0.62, 0.62] {
                cubes.push(Cube::new(
                    Vec3::new(x, y + dy, 0.69),
                    Vec3::new(1.6, 0.08, 0.08),
                    black,
                ));
            }
        }
        // Techo
        for i in 0..5 {
            let w = 15.0 - i as f32 * 2.55;
            cubes.push(Cube::new(
                Vec3::new(0., 9.15 + i as f32 * 0.42, 0.68),
                Vec3::new(w, 0.42, 2.75),
                black,
            ));
        }
        // Luces
        for &(x, y, c) in &[
            (-5., 7.6, Vec3::new(0.45, 0.75, 1.)),
            (5., 7.6, Vec3::new(1., 0.25, 0.62)),
            (-5., 4.6, Vec3::new(1., 0.55, 0.12)),
            (5., 4.6, Vec3::new(1., 0.08, 0.06)),
            (-5., 1.6, Vec3::new(0.2, 1., 0.35)),
            (5., 1.6, Vec3::new(0.18, 0.45, 1.)),
        ] {
            lights.push(Light {
                position: Vec3::new(x, y, 2.3),
                color: c,
                intensity: 10.,
                burning_only: false,
            });
        }
        lights.push(Light {
            position: Vec3::new(0., 11., 6.),
            color: Vec3::new(0.72, 0.78, 1.),
            intensity: 18.,
            burning_only: false,
        });
        // Incendio
        for &(x, y, s) in &[
            (-6.4, 9.8, 1.3),
            (-4.7, 10.1, 1.0),
            (-2.8, 9.55, 0.9),
            (0.2, 10.5, 1.45),
            (2.4, 9.7, 1.0),
            (4.7, 10.2, 1.35),
            (6.5, 9.55, 0.9),
            (-5.5, 5.6, 0.75),
            (5.7, 3.0, 0.8),
        ] {
            cubes.push(Cube::fire(
                Vec3::new(x, y, 0.9),
                Vec3::new(s * 0.75, s * 1.45, 0.6),
                fire(Vec3::new(1., 0.16, 0.015), Vec3::new(2.6, 0.15, 0.01)),
            ));
            cubes.push(Cube::fire(
                Vec3::new(x, y + s * 0.48, 1.0),
                Vec3::new(s * 0.48, s * 0.82, 0.52),
                fire(Vec3::new(1., 0.67, 0.03), Vec3::new(3.6, 1.1, 0.02)),
            ));
            lights.push(Light {
                position: Vec3::new(x, y, 2.6),
                color: Vec3::new(1., 0.20, 0.025),
                intensity: 25. * s,
                burning_only: true,
            });
        }
        for &(x, y) in &[(-5., 8.4), (0., 8.0), (5., 5.0), (-5., 2.2)] {
            cubes.push(Cube::fire(
                Vec3::new(x, y, -0.37),
                Vec3::new(4.0, 0.55, 0.08),
                charred(),
            ));
        }
        // Luz general
        lights.push(Light {
            position: Vec3::new(-10., 8., 10.),
            color: Vec3::new(0.55, 0.65, 1.),
            intensity: 28.,
            burning_only: false,
        });
        Self { cubes, lights }
    }
}
