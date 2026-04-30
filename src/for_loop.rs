pub fn for_loop() {
    for i in 1..3 {
        println!("[i] is: {}", i);
    }
    inclusive_range();
    break_and_continue();
}

fn inclusive_range() {
    for i in 1..=3 {
        println!("i is: {}", i);
    }
}

fn break_and_continue() {
    for i in 1..=10 {
        if i == 3 {
            continue; // skip 3
        }
        if i == 5 {
            break; // stop before printing 5
        }
        println!("i is: {}", i);
    }
}
