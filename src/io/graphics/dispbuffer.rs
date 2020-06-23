use crate::buffer::Buffer;
use std::io::{stdout, Write};
use termion::clear;
use termion::raw::IntoRawMode;
use termion::terminal_size;

fn display_buff_clear_all(buff: Buffer, stdout: &mut Write) {
    write!(stdout, "{}", clear::All).unwrap();
    let lines = buff.get_grid(terminal_size().unwrap().0.into());
    write!(stdout, "{:?}", lines);
}
