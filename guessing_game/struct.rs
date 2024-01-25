#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    age: i32,
}

impl User {
    fn area(&mut self) -> i32 {
        self.age += 10;
        self.age
    }
    fn is_bigger(&self, other: &User) -> bool {
        self.age >= other.age
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        name: String::from("john"),
        age: 21,
    };
    user1.name = String::new();
    let mut user2 = new_user(String::from("foobar"), 32);

    println!("{}", user2.area());
    println!("{}f", user2.is_bigger(&user1));
    println!("user1 is {:#?}", user1);
}

fn new_user(name: String, age: i32) -> User {
    User {
        name,
        age,
        active: true,
    }
}
