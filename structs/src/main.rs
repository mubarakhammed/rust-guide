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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let mut user1 = User {
    //     email: String::from("mubarak@rust.com"),
    //     username: String::from("mubawesco"),
    //     active: true,
    //     sign_in_count: 54,
    // };
    // let name = user1.username;
    // user1.username = String::from("muba123");

    // let user2 = build_user(
    //     String::from("mubarakhammed00@rust.com"),
    //     String::from("mubawesco123"),
    // );

    // let user3 = User {
    //     email: String::from("johndoe@rust.com"),
    //     username: String::from("johndoe123"),
    //     ..user2
    // };

    // //creating structs without named fields
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 20,
        height: 40,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("rect is {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect hold rec1: {} ", rect.can_hold(&rect1));
    println!("Can rect hold rect2: {} ", rect.can_hold(&rect2));
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
