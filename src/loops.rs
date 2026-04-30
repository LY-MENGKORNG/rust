pub fn loops() {
    let mut count = 1;

    loop {
        println!("Loop {}", count);

        if count >= 3 {
            break;
        }

        count += 1;
    }

    println!("Looo with return");
    loop_return();
}

pub fn loop_return() {
    let mut count = 1;

    let result = loop {
        println!("Loop {}", count);
        if count >= 3 {
            break count;
        }
        count += 1;
    };

    println!("The loop stopped at: {}", result)
}
