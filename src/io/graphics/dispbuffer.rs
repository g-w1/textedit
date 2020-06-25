use crate::buffer::Buffer;
use crate::core_edit::editor::Editor;
use std::io::{stdout, Write};
use termion::clear;
use termion::cursor::Goto;
use termion::terminal_size;

impl Buffer {
    pub fn display_buff_clear_all(&self) {
        print!("{}", clear::All);
        let lines = &self.content;

        for (num, line) in lines.iter().enumerate() {
            print!("{}", Goto(1, num as u16 + 1));
            println!("{}", line);
        }
    }
}
