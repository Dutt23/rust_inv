use std::io::{stdout, Stdout, Write};
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool){

  if force {
    // unwrap to crash if there is any errors.
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(ClearType::All));
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
  }

  for(x, col) in current_frame.iter().enumerate() {
    for (y, s) in col.iter().enumerate(){
      if *s != last_frame[x][y] || force {
        stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
        print!("{}", *s);
      }
    }
  }
  stdout.flush().unwrap();
}