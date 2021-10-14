use std::iter::Iterator;
use std::ops::Deref;

struct myvec {
    data: Vec<i32>,
}

impl Deref for myvec{
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/*
impl Iterator for myvec {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        if i <= self.data.len() {
            Some(self.data[i])
        }
    }

}
*/


fn main() {
    let v = myvec{
        data: vec![1,2,3],
    };
    for item in (*v).iter() {
        println!("{}", item);
    }
}
