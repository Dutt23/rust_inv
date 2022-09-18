// use std::error::Error;

use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event};
use rusty_audio::Audio;
use std::{fs, error::Error, io, thread};
use std::sync::mpsc;
use std::thread::Thread;
use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use invaders::{frame, render};

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


	// Render loop in a seperate thread.
	// Use crossbeam channels much easier
	let (render_tx, render_rx) = mpsc::channel();
	let data = vec![1, 2, 3];
	let render_handle = thread::spawn(move || {
		let mut last_frame = frame::new_frame();
		let mut stdout = io::stdout();
		render::render(&mut stdout, &last_frame, &last_frame, true);
		// println!("captured {data:?} by value");
		loop {
			let curr_frame = match render_rx.recv() {
				Ok(x) => x,
				Err(_) => break
			};
			render::render(&mut stdout, &last_frame, &curr_frame, false);
			last_frame = curr_frame;
		}
	});
	// println!("captured {data:?} by value");
	// Game loop
	'gameloop: loop {
		let curr_frame = frame::new_frame();
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

	// 	Draw and render
		let _ = render_tx.send(curr_frame);
		thread::sleep(Duration::from_millis(1));
	}
	// cleanup
	drop(render_tx);
	render_handle.join().unwrap();
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
