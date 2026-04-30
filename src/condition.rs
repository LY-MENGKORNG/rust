pub fn if_else() {
    let x = 3;
    let y = 9;

    if x > y {
        println!("[x] is greater than [y]");
    } else if x == y {
        println!("[x] is greater than [y]");
    } else {
        println!("[x] is less than [y]");
    }

    // If as Expression
    let age = 18;
    let greeting = if age > 18 { "Hi man!" } else { "Hi boy!" };

    println!("{}", greeting);
}
