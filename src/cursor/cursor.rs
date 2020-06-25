use crate::io::key_getter::next_key;
pub enum CursorType {
    Insert,
    Normal,
    Cmd,
}
pub struct Caret {
    mode: CursorType,
    pos: (usize, usize),
}

impl Caret {
    pub fn change_shape(&mut self, type_to_change_to: CursorType) {
        self.mode = type_to_change_to;
    }
    pub fn handle_key() {}
    pub fn goto(&mut self, pos: (usize, usize)) {



    }
}
