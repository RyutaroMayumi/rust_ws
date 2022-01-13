struct Point2D {
    x: i32,
    y: i32,
}

fn double_p1(mut p: Point2D) -> Point2D {
    p.x = p.x * 2;
    p.y = p.y * 2;
    p
}

fn double_p2(p: &mut Point2D) {
    p.x = p.x * 2;
    p.y = p.y * 2;
}

fn main() {

    let v1 = 42;
    let p1 = Point2D { x: 4, y: 2 };

    let v2 = v1;    // copy
    let p2 = p1;    // move
    let p3 = &p2;   // borrow


    println!("(v1, v2) = ({}, {})", v1, v2);
    // println!("p1 = ({}, {})", p1.x, p1.y);
    println!("p2 = ({}, {})", p2.x, p2.y);
    println!("p3 = ({}, {})", p3.x, p3.y);

    let p4 = Point2D { x: 2, y: 1 };
    let p5 = double_p1(p4);
    // println!("p4 = ({}, {})", p4.x, p4.y);
    println!("p5 = ({}, {})", p5.x, p5.y);

    let mut p6 = Point2D { x: 2, y: 1 };
    double_p2(&mut p6);
    println!("p6 = ({}, {})", p6.x, p6.y);
}
