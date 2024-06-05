enum List{
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons,Nil};
use std::ops::Deref;

use std::rc::Rc;
use List2::{Cons2,Nil2};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x=5;
    let y = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("dfghjk"));
    hello(&m);
    hello( &(*m)[..]);

    let a = Rc::new(Cons2(5,Rc::new(Cons2(10,Rc::new(Nil2)))));
    println!("Count after creating a: {}", Rc::strong_count(&a));
    let b = Cons2(3,Rc::clone(&a));
    println!("Count after creating b: {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("Count after creating c: {}", Rc::strong_count(&a));
    }
    println!("Count after c went out: {}", Rc::strong_count(&a));
}

enum List2{
    Cons2(i32, Rc<List2>),
    Nil2,
}




fn hello(s: &str){
    println!("Hello, {}", s);
}

struct MyBox<T>(T);

impl<T> MyBox<T>  {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target=T;

    fn deref(&self) -> &T {
        &self.0
    }
}
