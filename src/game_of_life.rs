use drawille::Canvas;
use rand::Rng;

pub struct Game {
    grid: Vec<Vec<bool>>,
    width: u32,
    height: u32,
    generations: u64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            grid: Self::generate_random_grid(width, height),
            width,
            height,
            generations: 0,
        }
    }

    pub fn tick(&mut self) {
        let mut new_grid = vec![vec![false; self.height as usize]; self.width as usize];

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.count_neighbors(x, y);
                if self.grid[x as usize][y as usize] {
                    if neighbors == 2 || neighbors == 3 {
                        new_grid[x as usize][y as usize] = true;
                    }
                } else if neighbors == 3 {
                    new_grid[x as usize][y as usize] = true;
                }
            }
        }

        self.grid = new_grid;
        self.generations += 1;
    }

    pub fn draw(&self) -> String {
        let mut canvas = Canvas::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                if self.grid[x as usize][y as usize] {
                    canvas.set(x, y);
                }
            }
        }
        format!(
            "{}[2J{}\nGeneration: {}",
            27 as char,
            canvas.frame(),
            self.generations
        )
    }

    fn count_neighbors(&self, x: u32, y: u32) -> u32 {
        let mut count = 0;

        for i in x.saturating_sub(1)..x.min(self.width - 2) + 2 {
            for j in y.saturating_sub(1)..y.min(self.height - 2) + 2 {
                if self.grid[i as usize][j as usize] {
                    count += 1;
                }
            }
        }

        if self.grid[x as usize][y as usize] {
            count -= 1;
        }

        count
    }

    fn generate_random_grid(width: u32, height: u32) -> Vec<Vec<bool>> {
        let mut rng = rand::thread_rng();
        let mut grid = Vec::new();

        for _ in 0..width {
            let mut row = Vec::new();
            for _ in 0..height {
                row.push(rng.gen_bool(0.5));
            }
            grid.push(row);
        }

        grid
    }
}

#[test]
fn verify_game() {
    let mut game = Game::new(100, 100);
    game.tick();
    assert_eq!(game.generations, 1);
}
