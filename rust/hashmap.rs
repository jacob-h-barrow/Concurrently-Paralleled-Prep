/*
HashMap
    -
    - Methods Used
        1. contains_key
        2. keys
        3. values, values_mut
        4. iter, iter_mut
        5. get
        6. get_mut
*/

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking { name: name.to_string(), country: country.to_string() }
    }
}

// OR let mut solar_distance = HashMap::new();
let vikings = HashMap::from([
    (Viking::new("Einar", "Norway"), 25),
    (Viking::new("Olaf", "Denmark"), 24),
    (Viking::new("Harald", "Iceland"), 12),
]);

// Use derived implementation to print the status of the vikings.
for (viking, health) in &vikings {
    println!("{viking:?} has {health} hp before lvl up");
}

for viking in &vikings.keys() {
    // modify an entry before an insert with in-place mutation
    let health = vikings.entry(viking).and_modify(|mana| *mana += 20).or_insert(10);
    println!("{viking:?} has {health} hp after lvl up");
}

// DO THIS FOR VIKINGS
// Look up the values associated with some keys.
let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
for &book in &to_find {
    match book_reviews.get(book) {
        Some(review) => println!("{book}: {review}"),
        None => println!("{book} is unreviewed.")
    }
}


use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn print_vikings(vikings: &HashMap<Viking, i32>, msg: &str) {
    for (viking, health) in vikings {
        println!("{viking:?} has {health} hp {msg} lvl up");
    }
}

fn main() {
    let mut vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    print_vikings(&vikings, "before");

    // Level up Viking by adding 20 to their health
    let viking = Viking::new("Einar", "Norway");
    vikings.entry(viking).and_modify(|health| *health += 20).or_insert(10);

    print_vikings(&vikings, "after");

    // List of Vikings to find
    let to_find = ["Einar", "Harald", "Leif"]; // "Leif" is not in the map

    for name in &to_find {
        let found = vikings.iter().find(|(viking, _)| viking.name == *name);
        match found {
            Some((viking, health)) => println!("{:?} has {} hp.", viking, health),
            None => println!("Viking named {} is not found.", name),
        }
    }

    // `get` Example
    let viking_name = "Olaf";
    let viking = Viking::new(viking_name, "Denmark");
    if let Some(health) = vikings.get(&viking) {
        println!("{:?} has {} hp.", viking, health);
    } else {
        println!("{:?} is not found.", viking);
    }

    // `get_mut` Example
    if let Some(health) = vikings.get_mut(&viking) {
        *health += 10; // Increase health
        println!("{:?} leveled up to {} hp.", viking, health);
    }

    // `values_mut` Example (modifies all health values)
    for health in vikings.values_mut() {
        *health += 5;
    }
    
    print_vikings(&vikings, "after all mutations");

    // `contains_key` Example
    let viking_to_check = Viking::new("Harald", "Iceland");
    if vikings.contains_key(&viking_to_check) {
        println!("{:?} exists in the map!", viking_to_check);
    } else {
        println!("{:?} is not in the map!", viking_to_check);
    }
}
