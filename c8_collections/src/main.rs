fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let v = vec![1, 2, 3];
    println!("{:#?}", v);
    let mut v = Vec::new();
    v.push(5);
    println!("{:?}", v);

    let v = vec![1,2,3];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum cell {
        Int(i32),
        Float(f32),
        Text(String),
        Mix((i32, f32)),
        AnonymousStruct{x: i32, y: f32},
    }

    let row = vec![
        cell::Int(3),
        cell::Float(0.5),
        cell::Text(String::from("new string")),
        cell::Mix((1, 0.1)),
        cell::AnonymousStruct{x:100, y:0.21},
    ];

    let item2 = &row[2];
    match item2 {
        cell::Text(text_value) => println!("I am {}", text_value),
        _ => println!("I am not text"),
    }

    if let cell::Text(text_value) = item2 {
        println!("I am {}", text_value);
    }

    let item3 = &row[3];
    if let cell::Mix((int_val, float_val)) = item3 {
        println!("I am {} and {}", int_val, float_val);
    }

    let item4 = &row[4];
    if let cell::AnonymousStruct{x: int_val, y: float_val} = item4 {
        println!("I am {} and {}", int_val, float_val);
    } 

    let s = "initial string".to_string();
    println!("{}", s);   

    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);


    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }

    for b in s.bytes() {
        print!("{} ", b);
    }

    for c in s.chars() {
        print!("{} ", c);
    }


    // hashmap
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // collect

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("yellow");
    let score = scores.get(&team_name);
    if let Some(value) = score {
        println!("{}", value);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // only insert a value if the key has no value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:b}", 256);

}
