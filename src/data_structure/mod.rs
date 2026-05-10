mod array;
mod enums;
mod hash_maps;
mod structs;
mod tuple;
mod vectors;

pub fn data_structure() {
    println!("\n--- [ARRAY] ---\n");
    array::arrays();
    println!("\n--- [VECTOR] ---\n");
    vectors::vectors();
    println!("\n--- [TUPLE] ---\n");
    tuple::tuples();
    println!("\n--- [HASH_MAPS] ---\n");
    hash_maps::hash_maps();
    println!("\n--- [STRUCT] ---\n");
    structs::structs();
    println!("\n--- [ENUM] ---\n");
    enums::enums();
}
