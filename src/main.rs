extern crate drawille;
extern crate rand;

use drawille::Canvas;
use rand::Rng;
use std::{thread, time};

fn main() {
    let width: u32 = 100;
    let height: u32 = 100;

    let mut canvas = generate_random_canvas(width, height);

    let mut generations: u64 = 0;
    loop {
        draw_frame(&canvas);
        println!("Generations: {}", generations);
        canvas = compute_tick(&canvas, width, height);
        generations += 1;
        thread::sleep(time::Duration::from_millis(75));
    }
}

fn compute_tick(canvas: &Canvas, width: u32, height: u32) -> Canvas {
    let mut new_canvas = Canvas::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let neighbors = count_neighbors(&canvas, x, y);
            if canvas.get(x, y) {
                if neighbors == 2 || neighbors == 3 {
                    new_canvas.set(x, y);
                }
            } else {
                if neighbors == 3 {
                    new_canvas.set(x, y);
                }
            }
        }
    }
    new_canvas
}

fn count_neighbors(canvas: &Canvas, x: u32, y: u32) -> u32 {
    let mut count = 0;
    for i in x - 1..x + 2 {
        for j in y - 1..y + 2 {
            if canvas.get(i, j) {
                count += 1;
            }
        }
    }
    if canvas.get(x, y) {
        count -= 1;
    }
    count
}

fn draw_frame(canvas: &Canvas) {
    clear_screen();
    println!("{}", canvas.frame());
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn generate_random_canvas(width: u32, height: u32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let mut rng = rand::thread_rng();
    for _i in 0..width * height / 2 {
        canvas.set(rng.gen_range(0..width), rng.gen_range(0..height));
    }
    canvas
}
