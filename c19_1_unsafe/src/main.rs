fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    //unsafe
    //println!("r1 is: {}", *r1);

    /*
    hi
    */

    let string1: String = String::from("hello");
    let slice: &str = &string1;
    let s = "i am literal";
    //println!(s);
    println!("{}",slice);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}


