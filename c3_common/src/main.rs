fn main() {
    let tup = (500, 6.4, 'a');

    let (_x, y, _z) = tup;

    let t0 = tup.0;
    let t2:char = tup.2;

    println!("The value of y is: {}", y);
    println!("The value of t0 is: {}", t0);
    println!("The value of t2 is: {}", t2);

    let x:[i32;5] = [1,2,3,4,5];
    let array1 = [3;5];
    println!("The value is: {}", x[1]);
    println!("The value is: {}", array1[4]);

    if true {
        println!("this is true.");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}",result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("number is {}", number);

    // test for
    let array: [u8; 4] = [1, 3, 5, 7];

    for item in array.iter() {
        println!("{}", item);
    }

    for item in 1..4 {
        println!("{}", item);
    }

    for item in (1..4).rev() {
        println!("{}", item);
    }

    println!("fib 7 is {}", fib(7));

}

fn fib(n: i32) -> i32 {
    if n == 1 || n==2 {
        return 1
    }
    return fib(n-1) + fib(n-2)
}
