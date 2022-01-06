use std::mem::size_of;

fn main() {

    let tup : (i32, bool, f64) = (39, false, 1.28);
    let size = size_of::<(i32, bool, f64)>();

    println!("size of tup = {}", size);
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);
}
