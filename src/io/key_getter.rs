use crate::buffer::Buffer;
use std::io::{Read, stdout, Write};
use termion;
use termion::async_stdin;
use termion::event::Key;
use termion::raw::IntoRawMode;

pub fn next_io() -> Key {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();
    let b = stdin.next();
}
