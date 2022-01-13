fn double(x: u8) -> u8 {
    // check overflow
    if x > u8::MAX / 2 {
        u8::MAX
    } else {
        x * 2
    }
}

fn main() {

    let start: u8;
    let end: u8;
    start = 0;
    end = u8::MAX;
    for i in start..end {
        let ii = double(i);
        println!("(i, 2i) = ({}, {})", i, ii);
    }
}
