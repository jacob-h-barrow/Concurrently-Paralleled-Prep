/*
    - Generics
        - Template specialization isn't supported yet, but could be in the future with #![feature(specialization)]
        - Can declare generic parameters in three places: in the struct, in the impl head, and in the fn definitions 
        - A trait defines the fxnality a particular type has and can share with other types
            - Trait bounds specifies how a generic type can be any type with a certain behavior 
            - Traits may be called interfaces in other languages
            - A type's behavior consists of the methods we can call on that type
            - One restriction to note is that we can implement a trait on a type (struct) only if either the trait or the type (struct), 
                or both, are local to our crate.
            - Python Users: trait overriding is common in MRO defined inheritance
                - Optional override for default implementations
            - Python Users: trait bounds is actually 3.12+ typing for fxns/methods. Both can be omitted
                - When is a trait bounds necessary? Good question, generally, if you need to specify bounded trait inputs
                    - Isnt that redundantly inherent..? Yes, yes it is.
                    - For Rust, the trait bounds should be preferred over parameter bounding when multiple traits per parameter are used,
                        or redundant traits are used
            - Trait bounds is the same as the where clause. Use the where clause when appropriate
            - Two types of generic implementations for a trait bound: blanket implementation and conditional implementation
*/

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditional Implementation
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Conditional Implementation: Where Clause
//// (IMPORTANT) Another way to use is the with the for keyword, but this only works for traits... Pair is a struct
impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    fn display_both(&self) {
        println!("x = {}, y = {}", self.x, self.y);
    }
}

impl<K,V> HashMap<K, V>
    where K : Hash + Eq
{
    ..
}
///////////////////////////////////////////////////////////////////////////////////
// Type (or Struct)
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Trait - Default implementation
pub trait Summary {
    // Virtual void like... or abstract_method like
    fn summarize_author(&self) -> String;

    // Optional implementation, but provides a default implementation
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Blanket Implementation
impl Summary for NewsArticle {
    // Trait Method Overriding - similar to subclass method overshadowing
    fn summarize(&self) -> String { format!("{}, by {} ({})", self.headline, self.author, self.location) }
}

/*
    Options
        1. Parameter-Defined Typing
        2. Trait Bounded: Basic
        3. Trait Bounded: Multiple Traits
        4. Trait Bounded: Where Clause
        5. Returning Types That Implement Traits
*/

// Option 1: Parameter-Defined Typing
pub fn notify(item: &impl Summary) { println!("Breaking news! {}", item.summarize()); }

// Option 2: Trait Bounded: Basic
pub fn notify<T: Summary>(item: &T) { ... }

// Option 3: Trait Bounded: Multiple Traits
pub fn notify<T: Summary + Display>(item: &T) { ... }

// Option 4: Trait Bounded: Where Clause
pub fn notify<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    ...
}

// Option 5: Returning Types That Implement Traits
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("horse_ebooks"),
        location: String::from("USA"),
        author: String::from("Aurthor"),
        content: String::from("of course, as you probably already know, people"),
    }
}
