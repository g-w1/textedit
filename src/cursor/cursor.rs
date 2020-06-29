use crate::io::key_getter::next_key;
use termion::cursor::Goto;
pub enum CursorType {
    Insert,
    Normal,
    Cmd,
}
pub struct Caret {
    pub mode: CursorType,
    pub pos: (usize, usize),
}

impl Caret {
    pub fn change_shape(&mut self, type_to_change_to: CursorType) {
        self.mode = type_to_change_to;
    }
    pub fn handle_key() {}
    pub fn goto(&mut self) {
        print!("{}", Goto(self.pos.0 as u16 + 1, self.pos.1 as u16 + 1));
    }
}
