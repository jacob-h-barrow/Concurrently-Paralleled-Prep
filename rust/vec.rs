/*
std::vec
    - Vec will allocate if and only if mem::size_of::<T>() * capacity() > 0
    - Do not rely on removed data to be erased for security purposes
    - Covered Methods
        - pub fn with_capacity(capacity: usize) -> Vec<T>
            - Constructs a new, empty Vect<T> with at least the specified capacity
        - pub capacity(&self) -> usize
        - pub fn clear(&mut self)
        - pub fn is_empty(&self) -> bool
        - pub fn pop(&mut self) -> Option<T>
            - Removes the last element and returns it or None
        - pub fn remove(&mut self, index: usize) -> T
            - Removes the element at position index, panics if it doesn't exist!
        - pub fn insert(&mut self, index: usize, element: T)
            - Inserts an element at the position index, panics if it doesn't exist!
        - pub fn append(&mut self, other: &mut Vec<T, A>)
            - Moves all elements of other into self (no copying)
            - Panics if it exceeds the capacity, or the underlying container types are wrong
        - pub fn push(&mut self, value: T)
            - Appends to the back, panics if it exceeds the capacity
        - drain
        - pub fn retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool
            - Retains only the elements specified by the predicate
            - Similar to drain... use this instead of drain... opposites
            - If you need predicate-based, round scheduling then you are using the wrong DS

    - Nightly builds
        - extract_if, pop_if (not covered, small use case)

    - Unsafe, but may be needed: as_mut_ptr, as _ptr
*/

fn main() {
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.insert(1, 7); // vec = [1, 7, 2]
    vec[0] = 7;       // vec = [7, 7, 2]

    // Assert values at indexes 0, 1, and the last index
    assert_eq!(vec[0], 7);
    assert_eq!(vec[1], 7);
    assert_eq!(vec[vec.len() - 1], 2);

    let mut v2 = Vec::with_capacity(10);
    let mut v3 = vec![1, 2, 3];

    v2.append(&mut v3);  // v2 = [1, 2, 3], v3 is now empty
    v2.extend([1, 2, 3]); // v2 = [1, 2, 3, 1, 2, 3]

    // Assertions after moving elements
    assert_eq!(v3.len(), 0); // v3 is empty
    assert_eq!(v2.len(), 6); // v2 contains 6 elements
    assert!(v3.is_empty()); // v3 should be empty

    vec.clear();

    while let Some(top) = v2.pop() {
        println!("{top}");
    }

    // Assertions after popping all elements from v2
    assert_eq!(vec.len(), v2.len()); // Both should be empty
    assert!(v2.is_empty());          // v2 should be empty
}

////////////////////////////////////////////////////////////////
// Slices are RO objects
fn read_slice(slice: &[usize]) {
    println!("Reading slice:");
    for &item in slice {
        print!("{item} ");
    }
    println!("\nSlice length: {}", slice.len());
}

fn main() {
    let v = vec![0, 1];

    // Passing a slice to a function
    read_slice(&v);

    // Iterating over a slice reference
    println!("Iterating over &v:");
    for x in &v {
        println!("{x}");
    }

    // Explicit slice type
    let u: &[usize] = &v;
    println!("Explicit slice u: {:?}", u);

    // Using a slice with a more generic type annotation
    let u: &[_] = &v;
    println!("Generic slice u: {:?}", u);
}
///////////////////////////////////////////////////////////////
let mut vec = vec![1, 2, 3, 4];
vec.retain(|&x| x % 2 == 0);
assert_eq!(vec, [2, 4]);

// OR
let mut vec = vec![1, 2, 3, 4, 5];
let keep = [false, true, true, false, true];
let mut iter = keep.iter();
vec.retain(|_| *iter.next().unwrap());
assert_eq!(vec, [2, 3, 5]);
///////////////////////////////////////////////////////////////
/*
    Nightly Builds
        - extract_if: creates an iterator which uses a closure to determine if the element in the range should be removed
            - Elements will still have to be extracted... if the iterator is broken, the elements remain

pub fn extract_if<F, R>(
    &mut self,
    range: R,
    filter: F,
) -> ExtractIf<'_, T, F, A>
where
    F: FnMut(&mut T) -> bool,
    R: RangeBounds<usize>,
*/

#![feature(extract_if)]
let mut numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 11, 13, 14, 15];

let evens = numbers.extract_if(.., |x| *x % 2 == 0).collect::<Vec<_>>();
let odds = numbers.extract_if(..., |x| *x % 2 != 0);

for (i, val) in odds.enumerate() { if i == 3 { break; } }

assert_eq!(evens, vec![2, 4, 6, 8, 14]);
assert_eq!(odds, vec![9, 11, 13, 15]);