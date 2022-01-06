pub struct Image<'a> {
    pub raw: &'a [u8; 256]
}

// error[E0106]: missing lifetime specifier
// pub struct Image {
//     pub raw: &[u8; 256]
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
