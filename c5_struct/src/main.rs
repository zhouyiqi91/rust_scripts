struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rec2: &Rectangle) -> bool {
        if self.height >= rec2.height && self.width >= rec2.width {
            return true;
        }
        return false;
    }

    fn change(&mut self, height: u32, width: u32) {
        self.height = height;
        self.width = width;
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}



fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.active = false;

    let email = String::from("someone@example.com");
    let username = String::from("someone@example.com");
    let user2 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user2.active);

    let user3 = User {
        active: false,
        ..user2
    };
    println!("{}", user3.email);

    // tuple structs
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("{}", black.0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    rect1.change(100, 100);
    println!("{}, {}", rect1.height, rect1.width);

    let s1 = Rectangle::square(10);
    println!("{}", s1.height);
}
