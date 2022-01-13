fn sum(arr: [u8; 256], size: usize) -> i32 {

    let mut sum: i32 = 0;
    for i in 0..size {
        sum += i32::from(arr[i]);
    }

    sum
}

// fn sum(arr: &[u8], size: usize) -> i32 {

//     let mut sum: i32 = 0;
//     for i in 0..size {
//         sum += i32::from(arr[i]);
//     }

//     sum
// }

fn main() {

    let bytes = [1u8; 256];

    let s = sum(bytes, 256);
    // let s = sum(&bytes, 256);

    println!("bytes : {:?}", bytes);
    println!("sum = {}", s);

}
