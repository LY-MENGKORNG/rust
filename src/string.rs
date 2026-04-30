// NOTE: There are two main types of strings in Rust:
// * `&str` - is called "string slices", and is used for fixed text like "Hello"
// * `String` - used when you need a string that can change

pub fn strings() {
    let greeting = "Hello world";

    // Creating a String
    let _text1 = greeting.to_string();
    let _text2 = String::from(greeting);

    // Changing a String
    let mut text3 = String::from("Hello");
    text3.push_str(" World");
    text3.push('!'); // => Hello World!

    // Concatenate Strings
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = String::from("What a beautiful day!");
    let _result = format!("{} {}. {}", s1, s2, s3);

    // let s1 = String::from("Hello");
    // let s2 = String::from("World!");
    // let s3 = String::from("What a beautiful day!");
    // let result = s1 + " " + &s2 + " " + &s3;
    // println!("{}", result);

    // String Length
    let name = String::from("John");
    let _str_length = name.len(); // 4
}
