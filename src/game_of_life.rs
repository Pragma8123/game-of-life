use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
};

use drawille::{Canvas, PixelColor};
use rand::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub alive: bool,
    pub age: u32,
}

pub struct Game {
    grid: Vec<Cell>,
    next_grid: Vec<Cell>,
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
        let next_grid = vec![Cell { alive: false, age: 0 }; size];

        Self {
            grid,
            next_grid,
            width: sim_width,
            height: sim_height,
            generations: 0,
            last_tick: Duration::new(0, 0),
        }
    }

    pub fn tick(&mut self, wrap_enabled: bool) {
        if self.width == 0 || self.height == 0 {
            return;
        }
        let start = Instant::now();

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.count_neighbors(x, y, wrap_enabled);
                let idx = self.index(x, y);
                let current_cell = self.grid[idx];
                let next_alive = if current_cell.alive {
                    neighbors == 2 || neighbors == 3
                } else {
                    neighbors == 3
                };

                self.next_grid[idx] = if next_alive {
                    Cell {
                        alive: true,
                        age: if current_cell.alive { current_cell.age + 1 } else { 1 },
                    }
                } else {
                    Cell {
                        alive: false,
                        age: 0,
                    }
                };
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

    pub fn get_cell(&self, x: u32, y: u32) -> Cell {
        if x < self.width && y < self.height {
            self.grid[self.index(x, y)]
        } else {
            Cell { alive: false, age: 0 }
        }
    }

    pub fn stamp_pattern(&mut self, pattern_id: u32) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        let cx = self.width / 2;
        let cy = self.height / 2;

        let pattern: &[(i32, i32)] = match pattern_id {
            1 => &[ // Glider
                (0, -1), (1, 0), (-1, 1), (0, 1), (1, 1)
            ],
            2 => &[ // Gosper Glider Gun
                (-17, 0), (-17, 1), (-16, 0), (-16, 1),
                (-7, 0), (-7, 1), (-7, 2), (-6, -1), (-6, 3), (-5, -2), (-5, 4), (-4, -2), (-4, 4),
                (-3, 1), (-2, -1), (-2, 3), (-1, 0), (-1, 1), (-1, 2), (0, 1),
                (3, -2), (3, -1), (3, 0), (4, -2), (4, -1), (4, 0), (5, -3), (5, 1),
                (7, -4), (7, -3), (7, 1), (7, 2),
                (17, -2), (17, -1), (18, -2), (18, -1)
            ],
            3 => &[ // Pulsar
                (-2, -1), (-3, -1), (-4, -1), (-6, -2), (-6, -3), (-6, -4),
                (-2, -6), (-3, -6), (-4, -6), (-1, -2), (-1, -3), (-1, -4),
                (2, -1), (3, -1), (4, -1), (6, -2), (6, -3), (6, -4),
                (2, -6), (3, -6), (4, -6), (1, -2), (1, -3), (1, -4),
                (-2, 1), (-3, 1), (-4, 1), (-6, 2), (-6, 3), (-6, 4),
                (-2, 6), (-3, 6), (-4, 6), (-1, 2), (-1, 3), (-1, 4),
                (2, 1), (3, 1), (4, 1), (6, 2), (6, 3), (6, 4),
                (2, 6), (3, 6), (4, 6), (1, 2), (1, 3), (1, 4)
            ],
            4 => &[ // LWSS
                (-1, -1), (2, -1), (-2, 0), (-2, 1), (2, 1), (-2, 2), (-1, 2), (0, 2), (1, 2)
            ],
            _ => &[],
        };

        // Determine bounding box to clear
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for &(dx, dy) in pattern {
            min_x = min_x.min(dx);
            max_x = max_x.max(dx);
            min_y = min_y.min(dy);
            max_y = max_y.max(dy);
        }

        // Add padding
        min_x -= 2;
        max_x += 2;
        min_y -= 2;
        max_y += 2;

        // Clear area
        for dx in min_x..=max_x {
            for dy in min_y..=max_y {
                let px = cx as i32 + dx;
                let py = cy as i32 + dy;
                if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                    let idx = self.index(px as u32, py as u32);
                    self.grid[idx] = Cell { alive: false, age: 0 };
                }
            }
        }

        // Draw pattern
        for &(dx, dy) in pattern {
            let px = cx as i32 + dx;
            let py = cy as i32 + dy;
            if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                let idx = self.index(px as u32, py as u32);
                self.grid[idx] = Cell { alive: true, age: 1 };
            }
        }
    }

    #[inline]
    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn count_neighbors(&self, x: u32, y: u32, wrap_enabled: bool) -> u32 {
        let mut count = 0;

        if wrap_enabled {
            let w = self.width as i32;
            let h = self.height as i32;
            let xi = x as i32;
            let yi = y as i32;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = ((xi + dx + w) % w) as u32;
                    let ny = ((yi + dy + h) % h) as u32;
                    if self.grid[self.index(nx, ny)].alive {
                        count += 1;
                    }
                }
            }
        } else {
            let x_start = x.saturating_sub(1);
            let x_end = (x + 1).min(self.width.saturating_sub(1));
            let y_start = y.saturating_sub(1);
            let y_end = (y + 1).min(self.height.saturating_sub(1));

            for i in x_start..=x_end {
                for j in y_start..=y_end {
                    if (i != x || j != y) && self.grid[self.index(i, j)].alive {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn generate_random_grid(width: u32, height: u32) -> Vec<Cell> {
        let mut rng = rand::rng();
        let size = (width as usize) * (height as usize);
        let mut grid = Vec::with_capacity(size);
        for _ in 0..size {
            let alive = rng.random_bool(0.5);
            grid.push(Cell {
                alive,
                age: if alive { 1 } else { 0 },
            });
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

    fn age_color(age: u32) -> PixelColor {
        match age {
            0 => PixelColor::Black,
            1 => PixelColor::BrightCyan,
            2..=5 => PixelColor::BrightGreen,
            6..=15 => PixelColor::Green,
            16..=30 => PixelColor::Yellow,
            31..=100 => PixelColor::BrightRed,
            _ => PixelColor::BrightMagenta,
        }
    }

    pub fn draw(&mut self, game: &Game, speed: u32, wrap_enabled: bool, paused: bool) -> io::Result<()> {
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
                let cell = game.get_cell(x, y);
                if cell.alive {
                    self.canvas.set_colored(x + 1, y + 1, Self::age_color(cell.age));
                }
            }
        }

        // HUD info at the bottom-left
        if height >= 4 {
            self.canvas.text(
                2,
                height - 4,
                width,
                format!("Tick Time: {:?}", game.last_tick()).as_str(),
            );
        }
        if height >= 8 {
            self.canvas.text(
                2,
                height - 8,
                width,
                format!("Generations: {}", game.generations()).as_str(),
            );
        }
        if height >= 12 {
            self.canvas.text(
                2,
                height - 12,
                width,
                format!("Speed: {} gen/s (+/- to adjust)", speed).as_str(),
            );
        }
        if height >= 16 {
            let wrap_str = if wrap_enabled { "Toroidal (Wrap)" } else { "Closed (Clip)" };
            self.canvas.text(
                2,
                height - 16,
                width,
                format!("Boundary: {} ('w' to toggle)", wrap_str).as_str(),
            );
        }
        if height >= 20 {
            let status_str = if paused { "PAUSED ('s' to step, '1-4' to stamp)" } else { "RUNNING (Space to pause)" };
            self.canvas.text(
                2,
                height - 20,
                width,
                format!("Status: {}", status_str).as_str(),
            );
        }

        // Set cursor to 0,0 and write (replacing \n with \r\n for raw terminal mode compatibility)
        let frame = self.canvas.frame().replace('\n', "\r\n");
        print!("\x1B[H{}", frame);
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
        game.tick(false);
        assert_eq!(game.generations(), 1);
    }

    #[test]
    fn verify_small_dimensions() {
        let mut game = Game::new(2, 2);
        assert_eq!(game.width(), 0);
        assert_eq!(game.height(), 0);
        game.tick(false);
        assert_eq!(game.generations(), 0);
    }
}
