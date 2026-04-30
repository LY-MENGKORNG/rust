pub fn variables() {
    let name = "Jonh";
    let age = 23;

    println!("{} is {} years old", name, age);

    muteability();
}

fn muteability() {
    let mut x = 3;
    println!("Before: {}", x);

    x = 9;
    println!("After: {}", x);
}
