// use crate::buffer::Buffer;
use std::io::{stdin, stdout}; //, Read, Write};
use termion;
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn next_key() -> Result<Key, std::io::Error> {
    // let mut stdin = stdin();
    // for c in stdin.keys() {
    //     // write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::clear::CurrentLine).unwrap();
    //     return Some(c.unwrap());
    // }
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key;
        }
    }
}

// match io::key_getter::next_key().unwrap() {
//     Key::Char(c) => println!("{}", c),
//     Key::Alt(c) => println!("*{}", c),
//     Key::Ctrl(c) => println!("^{}", c),
//     Key::Esc => println!("ESC"),
//     Key::Left => println!("Left"),
//     Key::Right => println!("Right"),
//     Key::Up => println!("Up"),
//     Key::Down => println!("Down"),
//     Key::Backspace => println!("Backspace"),
//     _ => {}
// }
