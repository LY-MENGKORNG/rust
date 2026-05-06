pub fn matches() {
    let day = 3;

    match day {
        1 => println!("Mon"),
        2 => println!("Tue"),
        3 => println!("Wed"),
        4 => println!("Thur"),
        5 => println!("Fri"),
        6 => println!("Sat"),
        7 => println!("Sun"),
        _ => println!("Invalid day!"),
    }

    multi_matches();
    match_with_return();
}

fn multi_matches() {
    let day = 3;

    match day {
        1..=5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day!"),
    }
}

fn match_with_return() {
    let day = 3;

    let result = match day {
        1 => "Mon",
        2 => "Tue",
        3 => "Wed",
        4 => "Thur",
        5 => "Fri",
        6 => "Sat",
        7 => "Sun",
        _ => "Invalid day!",
    };

    println!("{} is: {}", day, result);
}
