fn main() {
    // Mutability
    let mut x = 5;

    println!("The value of x: {}", x);

    x = 6;
    println!("The value of x: {}", x);

    // Shadowing
    let y = 7;
    let y = y * 2;
    let y = y / 3;

    println!("The value of y: {}", y);
}
