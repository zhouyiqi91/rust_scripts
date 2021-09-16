fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    //let age: Result<u8, _> = "34".parse();
    let age1: Result<u8, _> = Err("this is err");

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age1 {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        // without this else, nothing output
        println!("final else");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    let value = match stack.pop() {
        Some(top) => top,
        None => {
            println!("I am none");
            0
        },
    };
    println!("{}", value);

    // ignore multiple values
    let (x, y, ..) = (1,2,3,4,5);
    println!("{} {}", x, y);

    let val = 5;
    let y = match val {
        x => x,
    };
    println!("{}", y);

}
