#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.height * self.width
    }

    fn can_hold(&self, rect:&Rectangle) -> bool {
        self.area() > rect.area() 
    }

    fn square(i: u64) -> Rectangle {
        Rectangle {
            width: i,
            height: i,
        }
    }
}
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {

    // working with the dynamic string data structure in rust
    let mut s = String::from("Hello ");
    s.push_str(", world");
    let new_string = s.clone();
    println!("{}", new_string);
    println!("{}", s);

    say_something(s);
    
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];

    println!("{:?}", a_slice);

    let mut user = User {
        username: String::from("kosi"),
        email: String::from("keloasuzu@yahoo.com"),
        sign_in_count: 0,
        active: true
    };

    println!("user's email is {}", user.email);

    say_something(user.username);

    user.username = String::from("something");
    println!("{}", user.username);

    let email = String::from("random@person.com");
    let username = String::from("random.person");

    let user = build_user(username, email);
    println!("{:?}", user);

    // using the update syntax to create a new struct
    let user2 = User{
        username: String::from("updated.username"),
        ..user
    };

    println!("{:?}", user2);

    let rect = Rectangle {
        width: 10,
        height: 12,
    };

    let rect2 = Rectangle { width: 10, height: 40 };

    println!("Area of rectangle {}", rect.area());
    println!("Rectangle 1 can hold rectangle 2 {}", rect.can_hold(&rect2));
    println!("Rectangle 2 can hold rectangle 1 {}", rect2.can_hold(&rect));

    let my_square: Rectangle = Rectangle::square(5);
    println!("My square is {:?}", my_square);
}


fn say_something(message: String) {
    println!("The message is {}", message)
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // using the field init shorthand
        email, // using the field init shorthand
        active: true,
        sign_in_count: 1
    }
}
