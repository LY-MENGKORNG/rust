pub fn data_types() {
    let x = 3;
    let y = 6;
    let z = 9;

    println!("Type inferring: {}{}{}", x, y, z);

    let x2: i32 = 3;
    let y2: f64 = 6.66;
    let z2: char = '9';
    let is_x: bool = true;
    let msg: &str = "Hi mom!";

    println!("Type explicit: {}, {}, {}, {}, {}", x2, y2, z2, is_x, msg);
}
