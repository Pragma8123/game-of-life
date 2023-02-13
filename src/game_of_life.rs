extern crate drawille;
extern crate rand;

use drawille::Canvas;
use rand::Rng;

pub struct Game {
  canvas: Canvas,
  width: u32,
  height: u32,
  generations: u64,
}

impl Game {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      canvas: Self::generate_random_canvas(width, height),
      width,
      height,
      generations: 0,
    }
  }

  pub fn tick(&mut self) {
    let mut new_canvas = Canvas::new(self.width, self.height);

    for x in 0..self.width {
      for y in 0..self.height {
        let neighbors = self.count_neighbors(x, y);
        if self.canvas.get(x, y) {
          if neighbors == 2 || neighbors == 3 {
            new_canvas.set(x, y);
          }
        } else if neighbors == 3 {
          new_canvas.set(x, y);
        }
      }
    }

    self.canvas = new_canvas;
    self.generations += 1;
  }

  pub fn draw(&self) -> String {
    format!("{}[2J{}\nGeneration: {}", 27 as char, self.canvas.frame(), self.generations)
  }

  fn count_neighbors(&self, x: u32, y: u32) -> u32 {
    let mut count = 0;

    for i in x.saturating_sub(1)..x.min(self.width - 2) + 2 {
        for j in y.saturating_sub(1)..y.min(self.width - 2) + 2 {
            if self.canvas.get(i, j) {
                count += 1;
            }
        }
    }

    if self.canvas.get(x, y) {
        count -= 1;
    }

    count
  }

  fn generate_random_canvas(width: u32, height: u32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let mut rng = rand::thread_rng();

    for _i in 0..width * height {
        canvas.set(rng.gen_range(0..width), rng.gen_range(0..height));
    }

    canvas
  }
}

#[test]
fn verify_game() {
  let mut game = Game::new(100, 100);
  game.tick();
  assert_eq!(game.generations, 1);
}
