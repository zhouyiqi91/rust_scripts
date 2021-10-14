// function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}


fn main() {
    let result = do_twice(add_one, 2);
    println!("The answer is: {}", result);

    let list_of_statuses: Vec<Status> =
    (0u32..20)
    .map(|value| Status::Value(value))
    .collect();

    println!("{:?}", list_of_statuses);

    let mut x = String::from("I am x");
    let mut_and_return_x = || {
        x.push_str("hello");
        String::from("I am x")
    };
    consume_with_relish(mut_and_return_x);

    let x = String::from("I am x");
    let inmut_and_return_x = || {
        println!("{}", x);
        String::from("return value")
    };
    consume_with_relish(inmut_and_return_x);


    let mut v = vec![1,2,3];
    let v1: Vec<i32> = v.iter().map(|x| x+1).collect();
    let v2 = v.iter().map(|x| x+2).collect::<Vec<i32>>();
    println!("{:?} {:?}", v1, v2);

}

fn consume_with_relish<F>(mut func: F)
    where F: FnMut() -> String
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());

    println!("Delicious!");

    println!("Consumed: {}", func());

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}