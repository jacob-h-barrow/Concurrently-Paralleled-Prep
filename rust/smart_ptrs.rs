/*
    Smart Pointers
        - Exs: String, Vec<T>
        - Usually implemented using structs
        - Implements the following traits:
            1. Deref: allows an instance of the smart ptr struct to behave like a reference so your code could work with either references or smart ptrs
                - This allows customization of the dereference operator (*)
                - Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type
                - It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match 
                    the parameter type in the function or method definition.
                    A sequence of calls to the deref method converts the type we provided into the type the parameter needs.
                - Rust does deref coercion when it finds types and trait implementations in three cases:
                    1. From &T to &U when T: Deref<Target=U>
                    2. From &mut T to &mut U when T: DerefMut<Target=U>
                    3. From &mut T to &U when T: Deref<Target=U>
            2. Drop
                - Manual drop is only allowed with std::mem::drop
        - Here are a few smart pointers offered
            1. Box<T>: allocating values on the heap
            2. Rc<T>: reference counting type that enables multiple ownership
            3. Ref<T> and RefMut<T>, accessed through RefCell<T>: enforces the borrowing rules at runtime
            https://www.freecodecamp.org/news/smart-pointers-in-rust-with-code-examples/
            https://doc.rust-lang.org/std/rc/struct.Weak.html
            https://doc.rust-lang.org/std/sync/struct.Arc.html
            4. Weak
            5. Arc

        - Overview
            - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
            - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; 
            RefCell<T> allows immutable or mutable borrows checked at runtime.
            - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
*/

/*
    Box<T> Points to Data on the Heap
        - On the stack: the pointer
        - Reasons to use
            1. Unknown size at compile time, but want to use a context with exact size
            2. Want to transfer ownship without copying
            3. When you want to own a value with a particular trait, rather than type
*/

// Basics, Using Box<T> like a reference
//// Difference is that Box<T> points to a copied value of x, rather than a reference pointing to the value of x
fn main() {
    let x = 5;
    let y = Box::new(x);
    let z = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}

// Manual Drop
use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

/*
    Using Rc<T> to Share Data
        - Cloning an Rc<T> increases the Reference Count
        - Only one to allow multiple owners of the same data
        - When you call Rc::downgrade, you get a smart pointer of type Weak<T>. 
            - Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1. 
*/

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    
    // a = 1
    println!("count after creating a = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    
    // a = 2
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        
        // a = 3
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    // a = 2
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

/*
    RefCell<T> and the Interior Mutability Pattern
        - RefCell<T> represents single ownership over the data
        - Borrowing rules are enforced at runtime
*/
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}


// More Advanced Example
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}