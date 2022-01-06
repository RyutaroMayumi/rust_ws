pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // generic associated function
    pub fn new(x: T, y: T) -> Point<T> {
        Point{ x, y }
    }

    // generic method
    pub fn get_x(&self) -> &T {
        &self.x
    }
    pub fn get_y(&self) -> &T {
        &self.y
    }
}

// generic function
pub fn make_point<T>(x: T, y: T) -> Point<T> {
    Point{ x, y }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point() {
        let x = 300;
        let y = 400;
        let point = Point::<i32>::new(x, y);
        assert_eq!(*point.get_x(), x);  // '*' means the access to data in reference
        assert_eq!(*point.get_y(), y);  // '*' means the access to data in reference
    }
    #[test]
    fn test_make_point() {
        let x = 300;
        let y = 400;
        let point = make_point::<i32>(x, y);
        assert_eq!(*point.get_x(), x);  // '*' means the access to data in reference
        assert_eq!(*point.get_y(), y);  // '*' means the access to data in reference
    }
}
