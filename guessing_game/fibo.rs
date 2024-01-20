use std::io;

fn main() {
    println!("Input the nth number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");

    let input: i32 = input.trim().parse().expect("expected number");
    println!("{}", fibo(input));
}

fn fibo(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fibo(n - 1) + fibo(n - 2)
}
