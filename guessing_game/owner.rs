fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world");

    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{}", s1);

    // let mut s = String::from("Hello");
    // take_ownership(s);
    // s = String::from("world");
    // println!("{}", s);

    let mut s = String::from("hello");

    let s1 = &mut s;
    s1.push_str("world");

    // cannot use s1 after this line
    let s2 = &mut s;
    s2.push_str("fo");

    println!("{}", s);
    // println!("{}, {}", s1, s2);
    // same thing with using immutable borrow and mutable borrow
}
fn mutate_string(foo: &mut String) {
    foo.push_str(" world!");
}

fn take_ownership(foo: String) {
    println!("{}", foo);
}
fn example1() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
fn dangled() {
    fn dangle() -> &String {
        // dangle returns a reference to a String

        let s = String::from("hello"); // s is a new String

        &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
      // Danger!
}
// string slice hello = &s[0..5];
