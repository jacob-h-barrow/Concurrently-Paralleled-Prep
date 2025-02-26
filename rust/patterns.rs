/*
    - Matches are Exhaustive: compilation error is an arm possibility isn't covered

    - Control flow additions: if let, let else, while let
        - if let is syntax sugar for a match
        - If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern
            - instead of using let, we can use if let

    - A pattern consists of some combination of the following (Python's match statement)
        - Literals
        - Destructured arrays, enums, structs, or tuples
        - Variables
        - Wildcards
        - Placeholders

    - Patterns come in two forms: refutable (ones that can fail) and irrefutable
*/

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Patterns that bind to values
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn ex_one(coin: Coin, &mut count: int) {
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
        
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        count += 1;
    }
}

fn ex_two(&coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// Example Three
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);


// Using a catch-all
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    10 => (), // empty tuple type
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

// Using the Result<T, E> std::lib enum
use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

fn main2() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}


// ? can be used with the Option<T>
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// main can also return a Result<(), E>
use std::error::Error;
use std::fs::File;

// Box<dyn Error> is trait object, which can wrap any error
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}


// While Let - Example 1
let (tx, rx) = std::sync::mpsc::channel();

std::thread::spawn(move || {
    for val in [1, 2, 3] {
        tx.send(val).unwrap();
    }
});

while let Ok(value) = rx.recv() {
    println!("{value}");
}

// While Let - Example 2
let deck = vec![5, 6, 10, 3, 2, 4]; // Example card values.
let mut total = 0;
let max = 21;

let mut iter = deck.into_iter();

// The loop continues only if:
// 1. A card is drawn (Some(card)), AND
// 2. Adding the card does not exceed 21.
while let Some(card) = iter.next() && total + card <= max {
    total += card;
    println!("Drew card: {} (total: {})", card, total);
}

println!("Final total: {}", total);


// Pattern Matching
let x = Some(6);
let y = 10;
let boolean = true;

match x {
    1 | 2 => println!("one or two"),
    3..=5 => println!("3 through 5"),
    (6 | 7 | 8) if boolean => println!("Yes"), // Parentheses are needed
    Some(y) => println!("Matched, y = {y}"),
    _ => println!("Default case, x = {x:?}"),
}

/*
    Advanced Pattern Matching
        1. @ Bindings
        2. Ignoring Remining Parts of a Value with ..
        3. Ignoring Parts of a Value with a Nested _
        4. Destructuring (Nested) Structs and Enums
*/

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, z, .. } if (z > 10 && z < 15) => {
        println!("x is {x}, z is {z} (z in range 10-15)")
    }
    Point { x, z: 10, .. } => {
        println!("x is {x}, z is {z} (z is exactly 10)")
    }
    Point { x, .. } => println!("x is {x}"),
}

////////////////////////////////////

let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(old_value @ _), Some(_)) => {
        println!("Can't overwrite an existing customized value: {old_value}");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {setting_value:?}");

