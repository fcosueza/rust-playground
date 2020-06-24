fn main() {

    //Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let a = 6;

    let a = a + 1;

    let a = a * 2;

    println!("The value of a is: {}", a);
}
