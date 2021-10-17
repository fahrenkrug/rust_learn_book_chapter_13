mod refcell_example;
mod reference_cycle_example;

use std::ops::Deref;
use std::alloc::handle_alloc_error;

fn main() {
    store_on_the_heap();
    example_with_recursive_type::run();
    dereference_examples();
    deref_coercion_example();
    example_drop_trait();
    reference_counted_smart_pointers_example::run();
    refcell_example::run();
    reference_cycle_example::run();
}

// The Box type let's us store data on the heap not on the stack
// This is a dumb example and won't be used that much.
fn store_on_the_heap() {
    let b = Box::new(5);
    println!("Hello, box {}", b);
}

// Recursive type definitions can't be allocated on the stack because the recursiveness could
// potentially go on and on. Therefore we need allocation on the heap here.
mod example_with_recursive_type {
    use crate::example_with_recursive_type::List::{Cons, Nil};
    use std::rc::Rc;

    pub fn run() {
        let list = Cons(1, Box::new(Cons (2, Box::new(Cons (3, Box::new(Nil))))));
        println!("list: {:?}", list);
    }

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil
    }
}

fn dereference_examples() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // This wouldn't compile: assert_eq!(5, y);
    // We need the * to dereference from the reference to the value behind it

    // Same for Box
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    // This can be written like that because Box is implementing a trait called Deref
    assert_eq!(5, *y);

    // Example with own Box implementation
    let x = 5;
    let y = MyBox(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // This works becaues we implement the Deref trait for MyBox
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// In this example the dereferencing happens "multiple times".
// At first: The &String is dereferenced out of &MyBox
// At second: The &str is dereferenced out of the &Str
// &str is the type of the parameter from the hello function
fn deref_coercion_example() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // What would we need to write here without the Deref trait?
    // This beauty: hello(&(*m)[..]);
}

// The Deref trait calls Deref:deref internally as many times as needed. This is done during
// Compile time - so it's a no cost abstraction.

// For mutable reference there is the DerefMut trait

fn hello(name: &str) {
    println!("Hello {}", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn example_drop_trait() {
    let c = CustomSmartPointer{
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer{
        data: String::from("other stuff")
    };
    drop(c);
    println!("CustomPointers are created")
}

mod reference_counted_smart_pointers_example {
    use std::rc::Rc;
    use crate::reference_counted_smart_pointers_example::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil
    }
    pub fn run() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("Reference count after creating a: {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("Reference count after creating b: {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("Reference count after creating c: {}", Rc::strong_count(&a));
        }
        println!("Reference count after c goes out of scope: {}", Rc::strong_count(&a));
        // Actually the Drop trait is used here to decrease the reference count internally
    }
}
