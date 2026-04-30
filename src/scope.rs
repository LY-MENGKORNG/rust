pub fn scopes() {
    let x = 5;

    {
        let x = 10;
        println!("Inside block: {}", x);
    }

    println!("Outside block: {}", x);
}
