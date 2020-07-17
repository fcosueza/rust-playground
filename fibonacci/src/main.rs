use std::io;
use std::mem::replace;

fn main() {
    let mut nth = String::new();

    println!("Introduce the nth fibonacci number position (max. 180): ");

    io::stdin().read_line(&mut nth).expect("Failed to read");

    let nth: u8 = match nth.trim().parse() {
        Ok(nth) if nth > 180 => 180,
        Ok(nth) => nth,
        Err(_) => 1,
    };

    println!("{}nth term of fibonacci serie is {}.", nth, fibo_nth(nth));
}

/* fibo_nth returns the nth term in fibonacci serie  */

fn fibo_nth(nth: u8) -> i128 {
    let mut n: i128 = 1;
    let mut n1: i128 = 1;
    let mut n2: i128 = 1;

    for _ in 1..(nth - 1) {
        n = n2 + &n1;
        n2 = replace(&mut n1, n);
    }
    n
}
