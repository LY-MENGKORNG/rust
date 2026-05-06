pub fn while_loop() {
    let mut count = 9;

    while count >= 1 {
        println!("Count: {}", count);
        count -= 1;
    }
    while_false_condition();
    while_stop();
    while_continue();
}

/**
* Rust's `while` won't be started if it `false` condition
*/
fn while_false_condition() {
    let count = 10;

    #[deny(clippy::while_immutable_condition)]
    while count <= 5 {
        println!("This won't be printed.");
    }
}

/**
* using `break` to stop while
*/
fn while_stop() {
    let mut count = 1;

    while count <= 9 {
        if count <= 3 {
            println!("I am done, Let break it!");
            break;
        }

        println!("count: {}", count);

        count += 1;
    }
}

/**
* using `continue` to skip lines execution
*/
fn while_continue() {
    let mut count = 1;

    while count <= 9 {
        if count == 3 {
            count += 1;
            println!("Good number!");
            continue;
        }

        println!("count: {}", count);

        count += 1;
    }
}
