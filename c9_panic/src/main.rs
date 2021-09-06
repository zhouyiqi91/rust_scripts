use std::fs::File;
use std::io::Write;


fn main() {
    let f = File::open("hello.txt");
    let f1 = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => File::create("hello.txt").unwrap(),
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };

    let mut fh = File::create("hello.txt").unwrap();
    let s1: &str = "hello";
    fh.write_all(s1.as_bytes()).unwrap();

}
