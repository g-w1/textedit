pub mod buffer;
fn main() {
    let string = "tesaoiesntraoinstaoisrentoarient aorsiten aositn       oinoin   \ntest\ntea";
    let buff = buffer::Buffer::new_from_string(string);
    for i in buff.get_grid(10).iter() {
        println!("{}", i);
    }
}
