/*
BinaryHeap
    - Max Heap by default
    
    - If a method is documented as iterating in sorted order, that’s guaranteed to work as long as elements in the heap have not changed order

    - Methods Used
        1. new
        2. peek, peek_mut
        3. push
        4. pop
*/
use std::collections::BinaryHeap;

// OR let heap = BinaryHeap::new();
let mut heap = BinaryHeap::from([1, 5, 2]);

assert_eq!(heap.peek(), Some(&5));

// Let's add some scores...
heap.push(3);
heap.push(6);
heap.push(7);

// Now peek shows the most important item in the heap.
assert_eq!(heap.peek(), Some(&7));

// Mutably peek and modify the top element
if let Some(top) = heap.peek_mut() { *top += 1; }

assert_eq!(heap.peek(), Some(&8));

// We can check the length of a heap.
assert_eq!(heap.len(), 6);

// We can iterate over the items in the heap, although they are returned in
// a random order.
for x in &heap {
    println!("{x}");
}

// If we instead pop these scores, they should come back in order.
while let heap.pop() = value {
    println!("{value}");

    if value == 2 { break; }
}

assert_eq!(heap.pop(), Some(1));
assert_eq!(heap.pop(), None);

// We can clear the heap of any remaining items.
heap.clear();

// The heap should now be empty.
assert!(heap.is_empty())