/*  NOTE: Borrowing and References

    Sometimes you want to use a value without taking ownership of it.
    Rust lets you do this using a reference - this is called borrowing:

*/

pub fn borrowing() {
    // INFO: What is a Reference?
    // A reference lets you look at `a` value without owning it. You create `a` reference using the `&` symbol:
    // INFO: Since `b` is only borrowing the value, `a` still owns it.
    let a = String::from("Hello");
    let b = &a;
    println!("a = {}", a);
    println!("b = {}", b);

    // INFO: Mutable References
    // If you want to change `a` value through `a` reference, you need to make the reference `mut`:
    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");
    println!("{}", name_ref); // John Doe
    // WARN: Note: You can only have one mutable reference to a value at a time!

    /*
         NOTE: Why Borrowing is Important
        - Borrowing helps you reuse values safely, without giving them away.
        - It lets you use values without taking ownership
        - It avoids cloning, which can be slow for large data
        - It makes your programs safer and faster
    */
}
