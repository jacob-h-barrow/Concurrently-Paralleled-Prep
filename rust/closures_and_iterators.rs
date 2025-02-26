/*
    Closures
        - A fxn-like construct you can store in a variable
        - Anonymous fxns without type annotations (usually)
        - (IMPORTANT) For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value. 
        - Closures can capture values from their environment in three ways
            1. Borrowing immutably
            2. Borrowing mutably
            3. Taking ownership
        - Closures will implement one or more of the following traits
            1. FnOnce: moves captures values out of its body; only called once
            2. FnMut: doesnt move anything out, but may mutate the captured values
            3. Fn: doesnt move anything out, nor does it mutate the captured values

    Iterators
        - Iterators are lazy in Rust
        - The Iterator trait only requires the next method (Python users), used the same in both Python and Rust
            - Collection.iter(); -> returns immutable references
            - Collection.iter_mut(); -> returns mutable references
            - Collection.into_iter(); -> takes ownership of the collection and returns owned values
*/

// Ways to annotate a closure
fn  add_one_v1   (x: u32) -> u32 { x + 1 } // fxnal definition
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

// Ways to Capture Values
use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];

    //// Option 1: Borrowing immutably
    let only_borrows = || println!("From closure: {list:?}");
    only_borrows();

    //// Option 2: Borrowing mutably
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();

    //// Option 3: Taking ownership
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

// Annotated Closure Example
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T // F must be able to be called once, take no arguments, and return a T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}


// Examples
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    //// Example 1
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    //// Example 2
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}


// Iterator example
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

//// Iterator is lazily consumed here
for val in v1_iter {
    println!("Got: {val}");
}


// All iterators implement a trait named Iterator
pub trait Iterator {
    type Item; // Defines an associated type with this trait

    fn next(&mut self) -> Option<Self::Item>; // the Item type will be the type returned from the iterator

    // methods with default implementations elided
}


/*
    - Collection.iter(); -> returns immutable references
    - Collection.iter_mut(); -> returns mutable references
    - Collection.into_iter(); -> takes ownership of the collection and returns owned values
*/
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // returns immutable references

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// Another Example
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // .collect() consumes the closure (since they are lazy by default)

assert_eq!(v2, vec![2, 3, 4]);


// Another Example
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


// Advanced Example
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}


// Iterator Example
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{value} is at index {index}");
}