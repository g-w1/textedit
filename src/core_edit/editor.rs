use crate::buffer::Buffer;
use crate::cursor::cursor::Caret;
use crate::cursor::cursor::CursorType;
use crate::io;
use crate::io::key_getter::next_key;
use termion::event::Key;
// use crate::io::key_getter::next_key;
use std::io::stdout;
use termion::raw::IntoRawMode;

pub struct Editor {
    caret: Caret,
    buff: Buffer,
}

impl Editor {
    pub fn new(buff: Buffer) -> Editor {
        Editor {
            caret: Caret {
                mode: CursorType::Normal,
                pos: (0, 0),
            },
            buff: buff,
        }
    }
    pub fn start(&mut self) {
        self.buff.display_buff_clear_all();
        let thisedit = self.main_fn();
    }

    pub fn main_fn(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let break_bool: bool = false;
        // self.caret.goto();
        self.buff.display_buff_clear_all();

        while !break_bool {
            let key = match next_key().unwrap() {
                // Key::Char(c) => Some(c);,
                Key::Char(c) => {}
                // Key::Alt(c) => Some(c),
                // Key::Ctrl(c) => Some(c),
                // Key::Esc => println!("ESC"),
                // Key::Left => println!("Left"),
                // Key::Right => println!("Right"),
                // Key::Up => println!("Up"),
                // Key::Down => println!("Down"),
                // Key::Backspace => println!("Backspace"),
                _ => {
                    break;
                }
            };
            // self.caret.goto();
        }
    }
}
