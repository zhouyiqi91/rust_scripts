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
}
