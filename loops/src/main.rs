fn main() {
    // Loop
    let mut counter: u32 = 0;

    let result: u32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Labelled Loop

    let mut count: u32 = 0;

    'counting_up: loop {
        println!("counter = {count}");
        let mut remaining: u32 = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            } else if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End Counter = {count}");

    // While
    let mut number: u32 = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // For
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // Using Range in for
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
