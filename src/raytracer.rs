use crate::{
    geometry::{Intersect, Ray, RayIntersect},
    math::{hash2, Vec3},
    scene::Scene,
};

const MAX_DEPTH: u32 = 2;
const EPS: f32 = 0.003;

fn nearest(scene: &Scene, ray: Ray, burning: bool) -> Option<Intersect> {
    scene
        .cubes
        .iter()
        .filter(|c| burning || !c.burning_only)
        .filter_map(|c| c.ray_intersect(ray))
        .min_by(|a, b| a.distance.total_cmp(&b.distance))
}

fn sky(direction: Vec3, burning: bool) -> Vec3 {
    let t = (direction.y * 0.5 + 0.5).clamp(0., 1.);
    let base = if burning {
        Vec3::new(0.025, 0.008, 0.018).lerp(Vec3::new(0.22, 0.035, 0.015), t)
    } else {
        Vec3::new(0.035, 0.025, 0.11).lerp(Vec3::new(0.52, 0.19, 0.55), t)
    };
    let stars =
        if !burning && hash2((direction.x * 800.) as i32, (direction.y * 800.) as i32) > 0.994 {
            Vec3::splat(0.9)
        } else {
            Vec3::default()
        };
    let smoke = if burning {
        ((direction.x * 8. + direction.y * 13.).sin() * 0.5 + 0.5) * 0.055
    } else {
        0.
    };
    base + Vec3::splat(smoke) + stars
}

fn trace(scene: &Scene, ray: Ray, burning: bool, depth: u32) -> Vec3 {
    if depth > MAX_DEPTH {
        return sky(ray.direction, burning);
    }
    let Some(hit) = nearest(scene, ray, burning) else {
        return sky(ray.direction, burning);
    };
    let base = hit.material.sample(hit.point, hit.normal);
    let mut color = base * 0.075 + hit.material.emission;
    for light in scene.lights.iter().filter(|l| !l.burning_only || burning) {
        let to = light.position - hit.point;
        let dist = to.length();
        let ld = to / dist;
        if dist > 12.0 {
            continue;
        }
        let shadow_ray = Ray {
            origin: hit.point + hit.normal * EPS,
            direction: ld,
        };
        let blocked = nearest(scene, shadow_ray, burning).is_some_and(|h| h.distance < dist);
        if !blocked {
            let attenuation = light.intensity / (1. + dist * dist * 0.08);
            let diffuse = hit.normal.dot(ld).max(0.);
            let view = -ray.direction;
            let half = (ld + view).normalized();
            let spec =
                hit.normal.dot(half).max(0.).powf(hit.material.shininess) * hit.material.specular;
            color = color
                + base.component_mul(light.color) * (diffuse * attenuation)
                + light.color * (spec * attenuation);
        }
    }
    if hit.material.reflectivity > 0.001 {
        let rr = Ray {
            origin: hit.point + hit.normal * EPS,
            direction: ray.direction.reflect(hit.normal).normalized(),
        };
        color = color * (1. - hit.material.reflectivity)
            + trace(scene, rr, burning, depth + 1) * hit.material.reflectivity;
    }
    if hit.material.transparency > 0.001 {
        if let Some(rd) = ray.direction.refract(hit.normal, hit.material.ior) {
            let tr = Ray {
                origin: hit.point - hit.normal * EPS,
                direction: rd.normalized(),
            };
            color = color * (1. - hit.material.transparency)
                + trace(scene, tr, burning, depth + 1)
                    .component_mul(base.lerp(Vec3::splat(1.), 0.72))
                    * hit.material.transparency;
        }
    }
    let fog = (hit.distance / 38.).clamp(0., 0.68);
    color.lerp(sky(ray.direction, burning), fog)
}

pub fn render(
    scene: &Scene,
    camera: &crate::camera::Camera,
    w: usize,
    h: usize,
    burning: bool,
    pixels: &mut [u8],
    step: usize,
) {
    let threads = std::thread::available_parallelism()
        .map_or(1, |n| n.get())
        .min(h);
    let rows = (h + threads - 1) / threads;
    // Cada hilo recibe una porción independiente del buffer, sin compartir escrituras.
    std::thread::scope(|scope| {
        for (part, buffer) in pixels.chunks_mut(rows * w * 4).enumerate() {
            let start_y = part * rows;
            let local_h = buffer.len() / (w * 4);
            scope.spawn(move || {
                for local_y in (0..local_h).step_by(step) {
                    let y = start_y + local_y;
                    for x in (0..w).step_by(step) {
                        let mut c = trace(scene, camera.ray(x, y, w, h), burning, 0);
                        c = Vec3::new(c.x.powf(1. / 2.2), c.y.powf(1. / 2.2), c.z.powf(1. / 2.2))
                            .clamp01();
                        let rgba = [
                            (c.x * 255.) as u8,
                            (c.y * 255.) as u8,
                            (c.z * 255.) as u8,
                            255,
                        ];
                        // En modo rápido, el color calculado se copia al bloque que representa.
                        for by in local_y..(local_y + step).min(local_h) {
                            for bx in x..(x + step).min(w) {
                                let i = (by * w + bx) * 4;
                                buffer[i..i + 4].copy_from_slice(&rgba);
                            }
                        }
                    }
                }
            });
        }
    });
}
