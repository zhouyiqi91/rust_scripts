struct Point{x:i32, y:i32}

fn main() {
    let x = Some(5);
    let y = 6;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let char1 = 'x';
    let _c = match char1 {
        'a'..='z' => println!("In it"),
        _ => println!("Not In it"),
    };

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    println!("{}", char1);

    // if let takes ownership?No. Some() takes onwership
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    //println!("{:?}", s);

    let string1 = String::from("string 1");
    let string2 = match string1 {
        _ => println!("In it"),
    };

    println!("{}",string1);
}
