use crate::buffer::Buffer;
use std::io::{stdin, stdout, Read, Write};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn next_key() -> Option<Key> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = stdin();
    for c in stdin.keys() {
        // write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::clear::CurrentLine).unwrap();
        return Some(c.unwrap());
    }
    None
}
