fn main() {
    let x = five();
    another_function(x, 10);
}

/* Another function */

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

/* Function return a parameter */

fn five() -> i32 {
    5
}
