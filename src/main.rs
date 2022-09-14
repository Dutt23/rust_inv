// use std::error::Error;

use rusty_audio::Audio;
use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
	let mut audio = Audio::new();

	audio.add("explode", "./sounds/explode.wav");
	audio.add("lose", "sounds/lose.wav");
	audio.add("move", "sounds/move.wav");
	audio.add("pew", "sounds/pew.wav");
	audio.add("startup", "sounds/startup.wav");
	audio.add("win", "sounds/win.wav");

	audio.play("startup");

	// cleanup
	audio.wait();

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
