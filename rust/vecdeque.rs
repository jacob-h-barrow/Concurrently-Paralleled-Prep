/*
VecDeque
- A double-ended queue implemented with a growable ring buffer

- Methods Used
    - pub fn get_mut(&mut self, index: usize) -> Option<&mut T>
    - pub fn get(&self, index: usize) -> Option<&T>
    - pub fn back(&self) -> Option<&T>
        - Has the same interface: front
    - pub fn back_mut(&mut self) -> Option<&mut T>
        - Has the same interface: front_mut
    - pub fn pop_back(&mut self) -> Option<T>
        - Has the same interface: pop_front  
    - pub fn push_back(&mut self, value: T)
        - Has the same interface: push_front  
*/

use std::collections::VecDeque;

fn main() {
    let mut buf = VecDeque::new();

    // Corrected loop syntax
    for i in 0..=10 {
        buf.push_back(i);
    }

    assert_eq!(buf[1], 1);

    if let Some(elem) = buf.get_mut(1) {
        *elem = 7;
    }

    if let Some(x) = buf.front_mut() {
        *x = 9;
    }

    if let Some(elem) = buf.back() {
        println!("Last value is {:?}", elem);
    }

    println!("FIFO style!");
    
    // Corrected pop_front loop
    while let Some(value) = buf.pop_front() {
        println!("Value popped: {}", value);
    }
}
