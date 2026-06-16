mod game_of_life;

use crate::game_of_life::{Game, Renderer};
use clap::{value_parser, Parser};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::{
    io::{stdout, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use terminal_size::{terminal_size, Height, Width};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple implementation of Conway's Game of Life", long_about = None)]
struct Args {
    /// Width of the game board (Minimum: 2)
    #[arg(short = 'W', long, default_value = "100", value_parser = value_parser!(u32).range(2..))]
    width: u32,

    /// Height of the game board (Minimum: 2)
    #[arg(short = 'H', long, default_value = "100", value_parser = value_parser!(u32).range(2..))]
    height: u32,

    /// Simulation speed in generations per second (Minimum: 1)
    #[arg(short = 'S', long, default_value = "10", value_parser = value_parser!(u8).range(1..))]
    speed: u8,

    /// Full screen
    #[arg(short = 'F', long)]
    full: bool,
}

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> Self {
        crossterm::terminal::enable_raw_mode().ok();
        print!("\x1B[?25l"); // Hide cursor
        stdout().flush().ok();
        TerminalGuard
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        print!("\x1B[?25h"); // Show cursor
        stdout().flush().ok();
        crossterm::terminal::disable_raw_mode().ok();
    }
}

fn main() {
    let args = Args::parse();

    let width;
    let height;

    if args.full {
        if let Some((Width(w), Height(h))) = terminal_size() {
            width = (((w * 2) as i32) - 2).max(2) as u32;
            height = (((h * 4) as i32) - 2).max(2) as u32;
        } else {
            width = args.width;
            height = args.height;
        }
    } else {
        width = args.width;
        height = args.height;
    }

    let mut game = Game::new(width, height);
    let mut renderer = Renderer::new(width, height);

    let frame_duration = Duration::from_secs(1) / args.speed as u32;

    print!("\x1B[2J"); // Clear screen
    stdout().flush().unwrap();

    let _guard = TerminalGuard::new();

    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut paused = false;

    // Main game loop
    while running.load(Ordering::SeqCst) {
        let start = Instant::now();
        let mut step_requested = false;

        // Process all pending input events
        while event::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        running.store(false, Ordering::SeqCst);
                    }
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        running.store(false, Ordering::SeqCst);
                    }
                    KeyCode::Char(' ') => {
                        paused = !paused;
                    }
                    KeyCode::Char('s') | KeyCode::Char('n') => {
                        if paused {
                            step_requested = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !running.load(Ordering::SeqCst) {
            break;
        }

        if !paused || step_requested {
            game.tick();
            renderer.draw(&game).unwrap();
        }

        let elapsed = start.elapsed();

        if paused {
            if let Ok(true) = event::poll(Duration::from_millis(100)) {
                // Next iteration will read the event
            }
        } else if elapsed < frame_duration {
            let wait_time = frame_duration - elapsed;
            if let Ok(true) = event::poll(wait_time) {
                // Next iteration will read the event
            }
        }
    }
}

#[test]
fn verify_args() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
