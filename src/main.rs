extern crate clap;

mod game_of_life;

use crate::game_of_life::Game;
use clap::{value_parser, Parser};
use std::{thread, time::Duration, time::Instant};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple implementation of Conway's Game of Life", long_about = None)]
struct Args {
    /// Width of the game board
    #[arg(short = 'W', long, default_value = "100")]
    width: u32,

    /// Height of the game board
    #[arg(short = 'H', long, default_value = "100")]
    height: u32,

    /// Simulation speed in generations per second
    #[arg(short = 'S', long, default_value = "10", value_parser = value_parser!(u8).range(1..))]
    speed: u8,
}

fn main() {
    let args = Args::parse();

    let frame_duration = Duration::from_secs(1) / args.speed as u32;
    let mut game = Game::new(args.width, args.height);
    loop {
        let start = Instant::now();
        game.tick();
        println!("{}", game.draw());
        let elapsed = start.elapsed();
        println!("Tick/Draw time: {:?}", elapsed);
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
}

#[test]
fn verify_args() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
