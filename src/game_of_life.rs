use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
};

use drawille::{Canvas, PixelColor};
use rand::*;

pub struct Game {
    grid: Vec<bool>,
    next_grid: Vec<bool>,
    width: u32,
    height: u32,
    generations: u64,
    last_tick: Duration,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let sim_width = width.saturating_sub(2);
        let sim_height = height.saturating_sub(2);

        let size = (sim_width as usize) * (sim_height as usize);
        let grid = Self::generate_random_grid(sim_width, sim_height);
        let next_grid = vec![false; size];

        Self {
            grid,
            next_grid,
            width: sim_width,
            height: sim_height,
            generations: 0,
            last_tick: Duration::new(0, 0),
        }
    }

    pub fn tick(&mut self) {
        if self.width == 0 || self.height == 0 {
            return;
        }
        let start = Instant::now();

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.count_neighbors(x, y);
                let idx = self.index(x, y);
                let alive = self.grid[idx];
                let next_alive = if alive {
                    neighbors == 2 || neighbors == 3
                } else {
                    neighbors == 3
                };
                self.next_grid[idx] = next_alive;
            }
        }

        std::mem::swap(&mut self.grid, &mut self.next_grid);
        self.last_tick = start.elapsed();
        self.generations += 1;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn generations(&self) -> u64 {
        self.generations
    }

    pub fn last_tick(&self) -> Duration {
        self.last_tick
    }

    pub fn get_cell(&self, x: u32, y: u32) -> bool {
        if x < self.width && y < self.height {
            self.grid[self.index(x, y)]
        } else {
            false
        }
    }

    #[inline]
    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn count_neighbors(&self, x: u32, y: u32) -> u32 {
        let mut count = 0;
        let x_start = x.saturating_sub(1);
        let x_end = (x + 1).min(self.width.saturating_sub(1));
        let y_start = y.saturating_sub(1);
        let y_end = (y + 1).min(self.height.saturating_sub(1));

        for i in x_start..=x_end {
            for j in y_start..=y_end {
                if (i != x || j != y) && self.grid[self.index(i, j)] {
                    count += 1;
                }
            }
        }
        count
    }

    fn generate_random_grid(width: u32, height: u32) -> Vec<bool> {
        let mut rng = rand::rng();
        let size = (width as usize) * (height as usize);
        let mut grid = Vec::with_capacity(size);
        for _ in 0..size {
            grid.push(rng.random_bool(0.5));
        }
        grid
    }
}

pub struct Renderer {
    canvas: Canvas,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            canvas: Canvas::new(width, height),
        }
    }

    pub fn draw(&mut self, game: &Game) -> io::Result<()> {
        self.canvas.clear();

        let width = game.width();
        let height = game.height();

        // Draw borders at 0 and width + 1, height + 1 (representing actual bounds)
        self.canvas
            .line_colored(0, 0, width + 1, 0, PixelColor::Green); // Top
        self.canvas
            .line_colored(0, 0, 0, height + 1, PixelColor::Green); // Left
        self.canvas.line_colored(
            width + 1,
            0,
            width + 1,
            height + 1,
            PixelColor::Green,
        ); // Right
        self.canvas.line_colored(
            0,
            height + 1,
            width + 1,
            height + 1,
            PixelColor::Green,
        ); // Bottom

        // Draw cells offset by 1
        for x in 0..width {
            for y in 0..height {
                if game.get_cell(x, y) {
                    self.canvas.set_colored(x + 1, y + 1, PixelColor::BrightGreen);
                }
            }
        }

        // Generations text
        if height >= 8 {
            self.canvas.text(
                0,
                height - 8,
                width,
                format!("Generations: {}", game.generations()).as_str(),
            );
        }

        // Tick time text
        if height >= 4 {
            self.canvas.text(
                0,
                height - 4,
                width,
                format!("Tick Time: {:?}", game.last_tick()).as_str(),
            );
        }

        // Set cursor to 0,0 and write
        print!("\x1B[H{}", self.canvas.frame());
        stdout().flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_game() {
        let mut game = Game::new(100, 100);
        game.tick();
        assert_eq!(game.generations(), 1);
    }

    #[test]
    fn verify_small_dimensions() {
        let mut game = Game::new(2, 2);
        assert_eq!(game.width(), 0);
        assert_eq!(game.height(), 0);
        game.tick();
        assert_eq!(game.generations(), 0);
    }
}
