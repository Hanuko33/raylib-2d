use raylib::camera::Camera2D;
use raylib::color::Color;
use raylib::ffi::KeyboardKey::*;
use raylib::math::Vector2;
use raylib::prelude::RaylibMode2DExt;
use raylib::{self, prelude::RaylibDraw};

#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Player {
    x: i32,
    y: i32,
}

struct Tile {
    x: i32,
    y: i32,
    color: Color,
}
fn generator(w: &mut Vec<Tile>, x: i32, y: i32, rng: &mut impl rand::Rng) {
    for _ in 0..10000 {
        w.push(Tile {
            x: rng.random_range(-1000 + x * 2000..1000 + x * 2000),
            y: rng.random_range(-1000 + y * 2000..1000 + y * 2000),
            color: Color {
                r: rng.random(),
                g: rng.random(),
                b: rng.random(),
                a: rng.random(),
            },
        });
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().vsync().size(1024, 512).title("game").build();
    let mut rng = rand::rng();

    rl.set_target_fps(60);

    let mut player = Player { x: 0, y: 0 };

    let mut cam = Camera2D {
        offset: Vector2 {
            x: (1024 / 2) as f32,
            y: (512 / 2) as f32,
        },
        target: Vector2 {
            x: player.x as f32,
            y: player.y as f32,
        },
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut world: Vec<Tile> = Vec::new();
    let mut generated: Vec<Point> = Vec::new();

    let mut speed;

    while !rl.window_should_close() {
        for i in -1..2 {
            for j in -1..1 {
                let current_chunk = Point {
                    x: player.x / 2000 + i,
                    y: player.y / 2000 + j,
                };
                if !generated.contains(&current_chunk) {
                    generator(&mut world, current_chunk.x, current_chunk.y, &mut rng);
                    generated.push(current_chunk);
                }
            }
        }
        if rl.is_key_down(KEY_LEFT_CONTROL) {
            speed = 10;
        }
        else if rl.is_key_down(KEY_LEFT_ALT) {
            speed = 50;
        }
        else {
            speed = 5;
        }
        if rl.is_key_down(KEY_A) {
            player.x -= speed;
        }
        if rl.is_key_down(KEY_D) {
            player.x += speed;
        }
        if rl.is_key_down(KEY_W) {
            player.y -= speed;
        }
        if rl.is_key_down(KEY_S) {
            player.y += speed;
        }

        cam.offset = Vector2 {
            x: rl.get_screen_width() as f32 / 2.0,
            y: rl.get_screen_height() as f32 / 2.0,
        };
        cam.target = Vector2 {
            x: player.x as f32,
            y: player.y as f32,
        };

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        });
        {
            let mut d2d = d.begin_mode2D(cam);

            for i in &world {
                if i.x + 1024 > player.x
                    && i.x < player.x + 1024
                    && i.y + 1024 > player.y
                    && i.y < player.y + 1024
                {
                    d2d.draw_rectangle(i.x, i.y, 10, 10, i.color);
                }
            }
            d2d.draw_rectangle(
                player.x,
                player.y,
                10,
                10,
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 125,
                },
            );
        }
        d.draw_fps(10, 10);
    }
}
