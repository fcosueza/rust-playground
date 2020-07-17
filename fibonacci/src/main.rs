use std::io;

fn main() {
    let mut nth = String::new();

    println!("Introduce the nth fibonacci number position (max. 185): ");

    io::stdin().read_line(&mut nth).expect("Failed to read");

    let nth: u32 = match nth.trim().parse() {
        Ok(nth) => {
            if nth > 185 {
                185
            } else {
                nth
            }
        }
        Err(_) => 1,
    };

    let mut n: u128 = 1;

    if nth != 1 && nth != 2 && nth > 0 && nth <= 185 {
        let mut n_1: u128 = 1;
        let mut n_2: u128 = 1;

        for _ in 1..(nth - 1) {
            n = n_2 + n_1;
            n_2 = n_1;
            n_1 = n;
        }
    }

    println!("The {} term of fibonacci serie is {}.", nth, n);
}
