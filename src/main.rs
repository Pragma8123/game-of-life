use std::thread::sleep;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  let mut grid: [[bool; 50]; 50] = get_input();

  let tick_time = Duration::from_millis(100); // 10 TPS

  let mut ticks: u64 = 0;

  // main game loop
  loop {
    let now = Instant::now();

    draw(&grid);
    println!("Ticks: {}", ticks);
    grid = tick(grid);
    ticks += 1;

    let mut sleep_time = tick_time.checked_sub(now.elapsed());
    if sleep_time == None {
      sleep_time = Some(Duration::from_millis(0));
    }
    println!(
      "Frame Time: {:.3}ms",
      tick_time.checked_sub(sleep_time.unwrap()).unwrap().subsec_nanos() as f64 * 1e-6);
    sleep(sleep_time.unwrap());
  }
}

fn get_input() -> [[bool; 50]; 50] {
  let path = Path::new("input.txt");
  let display = path.display();

  // Open read only
  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
    Ok(_) => {},
  };

  let mut grid: [[bool; 50]; 50] = [[false; 50]; 50];
  let mut x: usize;
  let mut y: usize = 0;
  for i in s.split('\n') {
    x = 0;
    for j in i.split("") {
      if j == "1" {
        grid[y % 50][x % 50] = true;
      }
      x += 1;
    }
    y += 1;
  }

  grid
}

// % (modulo) is not the same as getting the remainder in Rust
fn rem(a: i64, b: i64) -> i64 {
  ((a % b) + b) % b
}

fn tick(grid: [[bool; 50]; 50]) -> [[bool; 50]; 50] {
  let mut next: [[bool; 50]; 50] = [[false; 50]; 50];
  for i in 0..50 {
    for j in 0..50 {
      // Check neighbors
      // TODO: Find a better way to do this
      let mut neighbors: u8 = 0;
      let x: i64 = i as i64;
      let y: i64 = j as i64;
      let xm: usize = rem(x - 1, 50) as usize;
      let xp: usize = rem(x + 1, 50) as usize;
      let ym: usize = rem(y - 1, 50) as usize;
      let yp: usize = rem(y + 1, 50) as usize;

      if grid[i][ym] { neighbors += 1; }
      if grid[i][yp] { neighbors += 1; }
      if grid[xm][j] { neighbors += 1; }
      if grid[xp][j] { neighbors += 1; }
      if grid[xm][ym] { neighbors += 1; }
      if grid[xp][yp] { neighbors += 1; }
      if grid[xm][yp] { neighbors += 1; }
      if grid[xp][ym] { neighbors += 1; }

      if grid[i][j] { // Alive
        if neighbors == 2 || neighbors == 3 {
          next[i][j] = true;
        }
      } else { // Dead
        if neighbors == 3 {
          next[i][j] = true;
        }
      }
    }
  }
  next
}

fn draw(grid: &[[bool; 50]; 50]) {
  // Clear screen
  print!("{}[2J", 27 as char);

  println!(" -----------------------------------------------------------------------------------------------------");
  for x in 0..50 {
    print!("| ");
    for y in 0..50 {
      if grid[x][y] {
        print!("* ");
      } else {
        print!("  ");
      }
    }
    println!("|");
  }
  println!(" -----------------------------------------------------------------------------------------------------");
}
