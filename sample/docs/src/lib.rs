// refer to https://ytyaru.hatenablog.com/entry/2020/09/18/000000
// refer to https://dackdive.hateblo.jp/entry/2020/12/09/103000

/// Doubles the number given.
/// 
/// # Examples
/// 
/// ```
/// let result = docs::double(5);
/// println!("result = {}", result);
/// ```
pub fn double(number: u32) -> u32 {
    number * 2
}

#[derive(Debug)]
pub enum Error {
    Overflow,
    Zero,
}

fn double_with_error_sub(number: u32) -> Result<u32, Error> {
    if number == 0 {
        Err(Error::Zero)
    } else if number > (u32::MAX / 2) {
        Err(Error::Overflow)
    } else {
        Ok(number * 2)
    }
}

/// Doubles the number given.
/// 
/// # Examples
/// 
/// ```
/// if let Ok(i) = docs::double_with_error(5) {
///     println!("5 x 2 = {}", i);
/// }
/// if let Err(e) = docs::double_with_error(0) {
///     println!("func failed with {:?}", e);
/// }
/// ```
pub fn  double_with_error(number: u32) -> Result<u32, Error> {
    let doubled = double_with_error_sub(number)?;
    Ok(doubled)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let source: u32 = 2;
        let result = double(source);
        assert_eq!(result, source * 2);
    }
    #[test]
    fn test_err_overflow() {
        let source: u32 = u32::MAX;
        let result = double_with_error(source);
        assert!(result.is_err());
    }
    #[test]
    fn test_err_zero() {
        let source: u32 = 0;
        let result = double_with_error(source);
        assert!(result.is_err());
    }
    #[test]
    fn test_normal() {
        let source: u32 = 2;
        let result = double_with_error(source);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), source * 2);
    }
}
