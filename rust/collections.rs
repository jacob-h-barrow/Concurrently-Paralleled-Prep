/*
    Vectors
        - Two ways to create: vec! (macro) and Vec::new
        - Methods:
            - push(T)
            - Indexing: operator[T] or .get(T)
                - .get(T) -> Option<&T>

        - C++ users
            - vector_var.get(T) is the same at the vector_var.at(T) -> Gist: .get(T) is bounded while [] isnt
            - Pointer assignment in Rust can be done

        - (IMPORTANT) When a vector gets dropped, all of it's elements gets dropped
*/

// Creation
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];

//// Pointer assignment in Rust
let third_elem: &i32 = &v[2];
let second_elem: Option<&i32> = v.get(1);

match second_elem {
    Some(second_elem) => println!("The third element is {second_elem}"),
    None => println!("There is no third element."),
}

// Iteration
//// Mutable references require the use of the dereference operator (*)
for i in &mut v { *i += 50; } // use &v for a const reference

/*
    Hash Maps
        - HashMap<K, V>
        - Methods
            1. map_var.get(&T) -> Option<&V>
            2. map_var.entry(&T).or_insert(&T) -> &mut Key
                - returns a mutable reference to the corresponding Entry key if it exists, else a new Entry key is created and a mutable reference returned
*/

use std::collections::HashMap;

// Example One
let mut scores = HashMap::new();
let color_1 = String::from("Green");
let color_2 = String::from("Blue");

//// If the owned values are not a primitive data type (or has the Copy trait defined), the data is moved
//// Can no longer use color_1 or color_2
scores.insert(color_1, 10);
scores.insert(color_2, 20);
scores.insert(String::from("Yellow"), 50);

//// Add a team only if the key isnt present
scores.entry(String::from("Red")).or_insert(100);

let team_name = String::from("Blue");
//// get -> Option<&V>
let score = scores.get(&team_name).copied().unwrap_or(0);

println!("{scores:?}");
println!("The score of the {team_name:?} is {score:?}");


// Example Two
let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");