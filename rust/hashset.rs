/*
HashSet
    - A hash set is implemented as a HashMap where the value is ()

    - Requires that the elements implement the Eq and Hash traits; can be achieved by using #[derive(PartialEq, Eq, Hash)]

    - Like HashMap, HashSet is randomly seeded: each HashSet instance uses a different seed, which means that HashSet::new cannot be used in const context.
        - Affected in concurrency-based sharing

    - Methods Used
        1. difference
        2. get, get_or_insert
        3. intersection
        4. union
        5. insert
        6. replace
*/
use std::collections::HashSet;
// Type inference lets us omit an explicit type signature (which
// would be `HashSet<String>` in this example).
let mut books = HashSet::new();

// Add some books.
books.insert("A Dance With Dragons".to_string());
books.insert("To Kill a Mockingbird".to_string());
books.insert("The Odyssey".to_string());
books.insert("The Great Gatsby".to_string());

// Check for a specific one.
if !books.contains("The Winds of Winter") {
    println!("We have {} books, but The Winds of Winter ain't one.",
             books.len());
}

// Remove a book.
books.remove("The Odyssey");

// Iterate over everything.
for book in &books {
    println!("{book}");
}
////////////////////////////////////////////////////////
use std::collections::HashSet;
#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    power: usize,
}

let mut vikings = HashSet::new();

vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });
vikings.insert(Viking { name: "Harald".to_string(), power: 8 });

// Use derived implementation to print the vikings.
for x in &vikings {
    println!("{x:?}");
}
//////////////////////////////////////////////////////////
use std::collections::HashSet;

fn main() {
    let northern_vikings: HashSet<String> = HashSet::from([
        "Einar".to_string(), 
        "Olaf".to_string(), 
        "Harald".to_string()
    ]);
    let southern_vikings: HashSet<String> = HashSet::from([
        "Olaf".to_string(), 
        "Harald".to_string(), 
        "Torstein".to_string(), 
        "Ragnar".to_string()
    ]);

    let vikings_intersection: HashSet<_> = southern_vikings
        .intersection(&northern_vikings)
        .collect(); 

    let southern_difference: HashSet<_> = southern_vikings
        .difference(&northern_vikings)
        .collect();

    let all_vikings: HashSet<_> = southern_vikings
        .union(&northern_vikings)
        .collect();

    println!("Intersection: {:?}", vikings_intersection);
    println!("Southern Difference: {:?}", southern_difference);
    println!("All Vikings: {:?}", all_vikings);
}
