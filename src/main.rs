pub mod buffer;
pub mod io;
use termion::event::Key;
fn main() {
    let string = "tesaoiesntraoinstaoisrentoarient aorsiten aositn       oinoin   \ntest\ntea";
    let buff = buffer::Buffer::new_from_string(string);
    for i in buff.get_grid(10).iter() {
        println!("{}", i);
    }
    match io::key_getter::next_key().unwrap() {
        Key::Char(c) => println!("{}", c),
        Key::Alt(c) => println!("*{}", c),
        Key::Ctrl(c) => println!("^{}", c),
        Key::Esc => println!("ESC"),
        Key::Left => println!("Left"),
        Key::Right => println!("Right"),
        Key::Up => println!("Up"),
        Key::Down => println!("Down"),
        Key::Backspace => println!("Backspace"),
        _ => {}
    }
}
