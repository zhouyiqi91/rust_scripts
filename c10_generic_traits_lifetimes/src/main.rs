use c10_generic_traits_lifetimes::Tweet;
use c10_generic_traits_lifetimes::Summary;
use c10_generic_traits_lifetimes as c10;

use std::fmt::Display;

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    c10::notify(&tweet);

    let s1 = String::from("longlong");
    let s2 = String::from("short");
    let ann = String::from("ann");
    let longest_str = longest(&s1, &s2, &ann);
    println!("The longest string is {}", longest_str);
}

fn longest<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: &T,
) -> &'a str 
    where T: Display,
{   
    println!("this is ann: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}