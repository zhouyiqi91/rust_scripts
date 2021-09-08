use std::{fmt, cmp};

pub struct Struct1{
    value1: i32,
}

impl fmt::Debug for Struct1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi: {}", self.value1)
    }
}

impl cmp::PartialEq for Struct1 {
    fn eq(&self, other: &Struct1) -> bool {
        self.value1 == other.value1
    }
}

#[cfg(test)]
mod tests {

    mod common;

    #[test]
    fn it_works() {
        common::setup();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test1() {
        assert!(true);
    }

    #[test]
    #[ignore]
    fn test_new() {
        let s1 = crate::Struct1{
            value1: 3
        };
        let s2 = crate::Struct1{
            value1: 3
        };
        assert_eq!(s1, s2);
    }

}
