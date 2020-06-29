pub mod buffer;

pub mod core_edit;
pub mod cursor;
pub mod io;
use buffer::Buffer;
use core_edit::editor::Editor;
// use crate::buffer::Buffer;
// use std::io::{stdout, Write};
// use termion::clear;
// use termion::event::Key;
// use termion::raw::IntoRawMode;

fn main() {
    let mut edit = Editor::new(Buffer::new_from_str(
        "Hello\nWorld\nfoo bar baz  lorem ipsum idk the rest",
    ));
    edit.start();
}
