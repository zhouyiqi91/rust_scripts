#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("i am write"));
    m.call();

    let option1: Option<i32> = None;
    match option1 {
        Some(5) => println!("i am 5"),
        None => println!("i am none"),
        _ => println!("else"),
    }

    let option2: Option<i32>;

    option2 = Some(5);

    if let option1 = option2 {
        println!("i am none in if let");
    }

    if let None = option2 {
        println!("i am none in if let none=");
    }

    let a:i32;
    a = 123;
    println!("{}", a);
}
