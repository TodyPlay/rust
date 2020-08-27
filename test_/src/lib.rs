#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 5);
    }

    #[test]
    fn err() {
        panic!("this is a err test")
    }

    #[test]
    fn assert() {
        assert!(false);
    }

    #[test]
    fn result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("不相等".to_string())
        }
    }
}
