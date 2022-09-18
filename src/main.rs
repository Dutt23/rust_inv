// use std::error::Error;

use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event};
use rusty_audio::Audio;
use std::{fs, error::Error, io};
use std::time::Duration;
use crossterm::event::{Event, KeyCode};

fn main() -> Result<(), Box<dyn Error>> {
	let mut audio = Audio::new();

	audio.add("explode", "./sounds/explode.wav");
	audio.add("lose", "sounds/lose.wav");
	audio.add("move", "sounds/move.wav");
	audio.add("pew", "sounds/pew.wav");
	audio.add("startup", "sounds/startup.wav");
	audio.add("win", "sounds/win.wav");

	audio.play("startup");

	let mut stdout = io::stdout();
	// Keyboard input
	terminal::enable_raw_mode()?;
	// When use vim or emacs.
	// You enter another screen, and on exit you come back.
	// This is the same thing.
	stdout.execute(EnterAlternateScreen)?;
	// Hide Cursor
	stdout.execute(Hide)?;
	

	// Game loop

	'gameloop: loop {
		// Input
		while event::poll(Duration::default())? {
			if let Event::Key(key_event) = event::read()? {
				match key_event.code {
					KeyCode::Esc | KeyCode::Char('q') => {
						audio.play("lose");
						break 'gameloop
					}
					_ => {}
				}
			}
		}
	}
	// cleanup
	audio.wait();
	// SHow cursor
	stdout.execute(Show)?;
	stdout.execute(LeaveAlternateScreen)?;
	terminal::disable_raw_mode()?;

	Ok(())
}

fn currentPathReader(){
	pathReader(String::from("./"))
}

fn previousPathReader(){
  pathReader(String::from("../"))
}

fn  pathReader(path: String){
	let paths = fs::read_dir(path).unwrap();

	for path in paths {
			println!("Name: {}", path.unwrap().path().display())
	}
}
