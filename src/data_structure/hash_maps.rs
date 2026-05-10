use std::collections::HashMap;

/**
* INFO: A `HashMap` stores key-value pairs. It lets you look up a value using a key.
* To use HashMap, you must import it from the standard library.
*/
pub fn hash_maps() {
    let mut capital_cities = HashMap::new();

    update_values(&mut capital_cities);
    access_values(&mut capital_cities);
    remove_values(&mut capital_cities);
    loop_hash_map(&mut capital_cities);

    // NOTE: Why Use HashMaps?
    //
    // To store data by key
    // To quickly look up values
    // To group related data (like names and scores)
    //
    // Note: HashMaps require keys to be unique. Inserting the same key again will overwrite the old value.
}

/**
* INFO: If you insert a new value using a key that already exists, the old value is replaced with the new one:
*/
fn update_values(cities: &mut HashMap<&str, &str>) {
    cities.insert("Cambodia", "Phnom Penh");
    cities.insert("Japan", "Tokyo");

    println!("Capital cities: {:?} \n", cities);
}

/**
* INFO: You can use the `.get()` method to access a value in a HashMap by its key:
*/
fn access_values(cities: &mut HashMap<&str, &str>) {
    let country = "Cambodia";
    if let Some(&city) = cities.get(&country) {
        println!("Capital city of cambodia is: '{}'\n", city)
    } else {
        println!("Country '{}' is not found in the map!\n", country);
    }
}

/**
* INFO: To remove a key from a HashMap, use the `.remove()` method:
*/
fn remove_values(cities: &mut HashMap<&str, &str>) {
    cities.remove("Japan");

    println!("Capital cities after removed: {:?}\n", cities);

    // TEST: insert back the deleted value
    update_values(cities);
}

/**
* INFO: You can use a for `loop` to go through all key/value pairs:
*/
fn loop_hash_map(cities: &mut HashMap<&str, &str>) {
    for (&country, city) in cities {
        println!("The capital city of '{}' is '{}'", country, city);
    }
}
