fn main() {
    // Variable declaration (inmutable by deafult)
    let a = 0;
    println!("The value of a is: {}", a);

    // Declare mutable variable (using mut)
    let mut x = 5;
    let y = 6;

    x = x + y;
    println!("The value of x: {}", x);

    // Variable shadowing (not the same behavior as mut)
    let y = y * 2;
    let y = y / 3;

    println!("The value of y: {}", y);
}
