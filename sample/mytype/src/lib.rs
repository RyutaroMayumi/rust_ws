pub enum TypeWithoutValue {
    Int,
    Float,
    Boolean,
}
impl TypeWithoutValue {
    // pub fn print_type(t: TypeWithoutValue) {
    //     match t {
    //         TypeWithoutValue::Int => println!("type is integer"),
    //         TypeWithoutValue::Float => println!("type is floating point"),
    //         TypeWithoutValue::Boolean => println!("type is boolean"),
    //     }
    // }
    pub fn print_type(&self) {
        match self {
            TypeWithoutValue::Int => println!("type is integer"),
            TypeWithoutValue::Float => println!("type is floating point"),
            TypeWithoutValue::Boolean => println!("type is boolean"),
        }
    }
}

pub enum TypeWithValue {
    Int(i64),
    Float(f64),
    Boolean(bool),
}
impl TypeWithValue {
    // pub fn print_type(t: TypeWithValue) {
    //     match t {
    //         TypeWithValue::Int(i) => println!("integer value: {}", i),
    //         TypeWithValue::Float(f) => println!("floating point value: {}", f),
    //         TypeWithValue::Boolean(b) => println!("boolean value: {}", b),
    //     }
    // }
    pub fn print_type(&self) {
        match self {
            TypeWithValue::Int(i) => println!("integer value: {}", i),
            TypeWithValue::Float(f) => println!("floating point value: {}", f),
            TypeWithValue::Boolean(b) => println!("boolean value: {}", b),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
