use crate::buffer::Buffer;
use crate::cursor::cursor::Caret;
use crate::cursor::cursor::CursorType;
use crate::io;
// use crate::io::key_getter::next_key;
use std::io::stdout;
use termion::raw::IntoRawMode;

pub struct Editor {
    caret: Caret,
    buff: Buffer,
}

impl Editor {
    fn new(cursor: Caret, buff: Buffer) {
        Editor {
            caret: Caret {
                mode: CursorType::Normal,
                pos: (0,0)
            },
            buff: cursor,
        }
    }
    pub fn start(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let buff = Buffer::new_from_str(string);
        buff.display_buff_clear_all();
        let thisedit = Editor::self.loop_fn();
    }

    fn loop_fn(&mut self) {
        let break: bool = false;
        // while !break {

        // }
    }
}
