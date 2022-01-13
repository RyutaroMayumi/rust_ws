// #[derive(Debug, Copy, Clone)]
struct Point2D {
    x: i32,
    y: i32,
}

// impl Point2D {
//     fn new() -> Point2D {
//         Point2D { x:1, y:1 }
//     }
// }

fn sum(arr: [Point2D; 10], size: usize) -> f64 {

    let mut sum: f64 = 0.0;
    for i in 0..size {
        let xx = f64::from(arr[i].x.pow(2));
        let yy = f64::from(arr[i].y.pow(2));
        sum += (xx + yy).sqrt();
    }

    sum
}

fn main() {

    // let points = [ Point2D{x:1,y:1}; 10 ];
    // let points: [Point2D; 10] = [ Point2D::new(); 10 ];
    let points: [Point2D; 10] = [ 
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
        Point2D{x:1,y:1},
    ];
    // uninitialized error occured
    // let points: [Point2D; 10];
    // for i in 0..10 {
    //     let point = Point2D { x: 1, y: 1 };
    //     points[i] = point;
    // }

    let s = sum(points, 10);

    // println!("points[0] = ({}, {})", points[0].x, points[0].y);
    println!("sum = {}", s);

}
