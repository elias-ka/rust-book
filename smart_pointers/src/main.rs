use std::ops::Deref;
use std::rc::Rc;

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {
    //let m = MyBox::new(String::from("Rust"));
    //hello(&m);


    // let c = CustomSmartPointer {
    //     data: String::from("my stuff")
    // };
    // println!("CustomSmartPointer created.");
    // std::mem::drop(c);
    // println!("CustomSmartPointer dropped before the end of main.");
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff")
    // };
    // println!("CustomSmartPointer created.")

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}