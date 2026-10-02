mod camera;
mod framebuffer;
mod geometry;
mod material;
mod math;
mod raytracer;
mod scene;

use camera::Camera;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use scene::Scene;

const RENDER_W: i32 = 480;
const RENDER_H: i32 = 270;
const WINDOW_W: i32 = 1280;
const WINDOW_H: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_W, WINDOW_H)
        .title("Lover House: Before the Fire / In Flames")
        .resizable()
        .build();
    rl.set_target_fps(60);
    let image = Image::gen_image_color(RENDER_W, RENDER_H, Color::BLACK);
    let mut texture = rl
        .load_texture_from_image(&thread, &image)
        .expect("No se pudo crear el framebuffer");
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    let scene = Scene::lover_house();
    let mut camera = Camera::new();
    let mut framebuffer = Framebuffer::new(RENDER_W as usize, RENDER_H as usize);
    let mut burning = false;
    let mut auto_rotate = false;
    let mut dirty = true;
    let mut was_moving = false;
    let mut render_ms = 0.;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.05);
        let mut moved = false;
        let rotate = 1.15 * dt;
        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.yaw -= rotate;
            moved = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.yaw += rotate;
            moved = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
            camera.distance = (camera.distance - 12. * dt).max(9.);
            moved = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.distance = (camera.distance + 12. * dt).min(42.);
            moved = true;
        }
        let wheel = rl.get_mouse_wheel_move();
        if wheel.abs() > 0.01 {
            camera.distance = (camera.distance - wheel * 2.0).clamp(9., 42.);
            moved = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            burning = false;
            dirty = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            burning = true;
            dirty = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            burning = !burning;
            dirty = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            auto_rotate = !auto_rotate;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            camera = Camera::new();
            dirty = true;
        }
        if auto_rotate {
            camera.yaw += 0.22 * dt;
            moved = true;
        }
        let refine = was_moving && !moved;
        dirty |= moved || refine;
        if dirty {
            let start = std::time::Instant::now();
            // Durante el movimiento se renderizan bloques grandes; al detenerse se dibuja cada píxel.
            let step = if moved { 3 } else { 1 };
            raytracer::render(
                &scene,
                &camera,
                framebuffer.width,
                framebuffer.height,
                burning,
                &mut framebuffer.pixels,
                step,
            );
            render_ms = start.elapsed().as_secs_f32() * 1000.;
            texture
                .update_texture(&framebuffer.pixels)
                .expect("No se pudo actualizar el framebuffer");
            dirty = false;
        }
        was_moving = moved;
        let sw = rl.get_screen_width() as f32;
        let sh = rl.get_screen_height() as f32;
        let scale = (sw / RENDER_W as f32).min(sh / RENDER_H as f32);
        let dw = RENDER_W as f32 * scale;
        let dh = RENDER_H as f32 * scale;
        let dx = (sw - dw) * 0.5;
        let dy = (sh - dh) * 0.5;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(7, 5, 12, 255));
        d.draw_texture_pro(
            &texture,
            Rectangle::new(0., 0., RENDER_W as f32, RENDER_H as f32),
            Rectangle::new(dx, dy, dw, dh),
            Vector2::zero(),
            0.,
            Color::WHITE,
        );
        let screen_height = d.get_screen_height();
        let screen_width = d.get_screen_width();
        d.draw_rectangle(18, 18, 510, 92, Color::new(8, 6, 15, 205));
        d.draw_rectangle_lines(
            18,
            18,
            510,
            92,
            if burning { Color::ORANGE } else { Color::PINK },
        );
        d.draw_text("LOVER HOUSE", 34, 30, 28, Color::WHITE);
        d.draw_text(
            if burning {
                "IN FLAMES  [2]"
            } else {
                "BEFORE THE FIRE  [1]"
            },
            34,
            62,
            22,
            if burning { Color::ORANGE } else { Color::PINK },
        );
        d.draw_rectangle(18, screen_height - 88, 770, 70, Color::new(8, 6, 15, 205));
        d.draw_text(
            "A/D: rotar   W/S o rueda: zoom   F: cambiar estado",
            32,
            screen_height - 75,
            18,
            Color::RAYWHITE,
        );
        d.draw_text(
            "ESPACIO: rotacion automatica   R: reiniciar camara",
            32,
            screen_height - 48,
            18,
            Color::LIGHTGRAY,
        );
        let fps = d.get_fps();
        d.draw_text(
            &format!(
                "FPS: {}  |  Render: {:.0} ms  |  5 materiales",
                fps, render_ms
            ),
            screen_width - 390,
            24,
            18,
            Color::RAYWHITE,
        );
    }
}
