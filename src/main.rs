extern crate colored;

use std::{thread, time};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use colored::*;

fn main() {
  let mut grid: [[bool; 30]; 30] = get_input();

  // main game loop
  loop {
    draw(grid);
    grid = tick(grid);
    thread::sleep(time::Duration::from_millis(100));
  }
}

fn get_input() -> [[bool; 30]; 30] {
  let path = Path::new("input.txt");
  let display = path.display();

  // Open read only
  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    Ok(_) => {},
  };

  let mut grid: [[bool; 30]; 30] = [[false; 30]; 30];
  let mut x: usize = 0;
  let mut y: usize = 0;
  for i in s.split('\n') {
    x = 0;
    for j in i.split("") {
      if j == "1" {
        grid[y % 30][x % 30] = true;
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

fn tick(grid: [[bool; 30]; 30]) -> [[bool; 30]; 30] {
  let mut next: [[bool; 30]; 30] = [[false; 30]; 30];
  for i in 0..30 {
    for j in 0..30 {
      // Check neighbors
      let mut neighbors: u8 = 0;
      let x: i64 = i as i64;
      let y: i64 = j as i64;
      let xm: usize = rem((x - 1), 30) as usize;
      let xp: usize = rem((x + 1), 30) as usize;
      let ym: usize = rem((y - 1), 30) as usize;
      let yp: usize = rem((y + 1), 30) as usize;

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

fn draw(grid: [[bool; 30]; 30]) {
  // Clear screen
  print!("{}[2J", 27 as char);

  println!(" -------------------------------------------------------------");
  for i in &grid {
    print!("| ");
    for j in i {
      if *j {
        print!("{} ", "*".blue().bold());
      } else {
        print!("  ");
      }
    }
    println!("|");
  }
  println!(" -------------------------------------------------------------");
}
