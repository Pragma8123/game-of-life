extern crate clap;

mod game_of_life;

use crate::game_of_life::Game;
use std::{thread, time};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple implementation of Conway's Game of Life", long_about = None)]
struct Args {
    #[arg(short = 'W', long, default_value = "100")]
    width: u32,

    #[arg(short = 'H', long, default_value = "100")]
    height: u32,
}

fn main() {
    let args = Args::parse();

    let mut game = Game::new(args.width, args.height);
    game_loop(&mut game);
}

fn game_loop(game: &mut Game) {
    loop {
        println!("{}", game.draw());
        game.tick();
        thread::sleep(time::Duration::from_millis(33));
    }
}
