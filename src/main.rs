extern crate sdl2;

use rand::Rng;
use rand_distr::{Distribution, Normal};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
// use std::collections::HashSet;
// use std::time::Duration;

const VSYNC: bool = true;
const NUM_RECTS: i32 = 27;
const RECT_W: u32 = 40;
const W: u32 = RECT_W * NUM_RECTS as u32;
const H: u32 = 720;
const TITLE: &str = "Distributions Demo";
// const BG_COLOR: Color = Color::RGB(255, 253, 208);
const BG_COLOR: Color = Color::RGB(238, 236, 194);

const STD_DEV: f32 = 0.15;
const MEAN: f32 = 0.5;

const ROUNDS: i32 = 5000;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsys = sdl_ctx.video().unwrap();

    let mut window = video_subsys
        .window(TITLE, W, H)
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    window.set_position(
        sdl2::video::WindowPos::from((1920 / 2) - (W / 2) as i32),
        sdl2::video::WindowPos::from((1080 / 2) - (H / 2) as i32),
    );

    let wincan = window.into_canvas().accelerated();

    let wincan = if VSYNC {
        wincan.present_vsync()
    } else {
        wincan
    };

    let mut wincan = wincan.build().map_err(|e| e.to_string()).unwrap();

    wincan.set_blend_mode(BlendMode::Blend);
    wincan.set_draw_color(BG_COLOR);
    wincan.clear();
    wincan.present();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut normal: bool = false;
    let mut running: bool = false;
    let mut is_done: bool = false;

    let mut rect_vec: Vec<Rect> = Vec::new();

    // Add rectangles to rect_vec
    for _x in 0..NUM_RECTS {
        rect_vec.push(Rect::new(_x * RECT_W as i32, H as i32 - 1, RECT_W, 0));
    }

    // Have standard distribution ready
    let norm = Normal::new(MEAN, STD_DEV).unwrap();

    let mut count: i32 = 0;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'mainloop;
                }
                Event::KeyDown {
                    keycode: Some(k), ..
                } => match k {
                    Keycode::U => {
                        if !running && !is_done {
                            println!("Running uniform...");
                            running = true;
                            normal = false;
                        }
                    }
                    Keycode::N => {
                        if !running && !is_done {
                            println!("Running normal...");
                            running = true;
                            normal = true;
                        }
                    }
                    Keycode::R => {
                        println!("Reset!");
                        running = false;
                        is_done = false;
                        count = 0;
                        reset_rects(&mut rect_vec);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if count >= ROUNDS {
            println!("Done!");
            is_done = true;
            running = false;
            count = 0;
        }

        if running {
            // sample random number
            let num: f32 = if !normal {
                rand::thread_rng().gen::<f32>().clamp(0.0, 1.0)
            } else {
                norm.sample(&mut rand::thread_rng()).clamp(0.0, 1.0)
            };

            // Add it to the proper rectangle
            for r in &mut rect_vec {
                if (num * W as f32) as i32 >= r.x()
                    && (num * W as f32) as i32 <= r.x() + r.width() as i32
                {
                    r.set_y((r.y() - 1).clamp(0, H as i32));
                    r.set_height((r.height() + 1).clamp(0, H));
                }
            }
            count += 1;
        }
        // Clear screen
        wincan.set_draw_color(BG_COLOR);
        wincan.clear();

        // Draw rectangles
        for r in &rect_vec {
            wincan.set_draw_color(pick_color((r.x() / RECT_W as i32) as u32));
            wincan.fill_rect(r.clone()).unwrap();
        }

        wincan.present();
        // for i in 0..NUM_RECTS {
        //     wincan.set_draw_color(pick_color(i as u32));
        //     wincan.fill_rect(rect_vec.get(i as usize).as_deref()).unwrap();
        // }

        // let mut keystate: HashSet<Keycode> = event_pump
        //     .keyboard_state()
        //     .pressed_scancodes()
        //     .filter_map(Keycode::from_scancode)
        //     .collect();

        // if keystate.contains(&Keycode::U) {
        //     if !running {
        //         println!("Running uniform...");
        //         running = true;
        //         normal = false;
        //     }
        // }
        // if keystate.contains(&Keycode::N) {
        // if !running {
        //     println!("Running normal...");
        //     running = true;
        //     normal = true;
        // }
        // }
        // if keystate.contains(&Keycode::R) {
        // println!("Reset!");
        // running = false;
        // keystate.remove(&Keycode::R);
        // }
    }
}

fn pick_color(num: u32) -> Color {
    match num % 3 {
        0 => Color::RGBA(200, 50, 100, 200),
        1 => Color::RGBA(47, 70, 200, 200),
        2 => Color::RGBA(30, 220, 150, 200),
        _ => Color::MAGENTA,
    }
}

fn reset_rects(v: &mut Vec<Rect>) {
    for r in v {
        r.set_y((H - 1) as i32);
        r.set_height(0);
    }
}
