use std::fmt::{format, Display};

fn main() {
    
}

pub struct Article{
    pub author: String,
    pub headline: String,
    pub contents: String, 
}

impl Summary for Article {
    fn summerize(&self) ->String {
        format!("{}, by {}", self.headline, self.author)
    }
    
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub retweet: String, 
}

impl Summary for Tweet { }

pub trait Summary {
    fn summerize(&self) ->String{
        String::from("Read more...")
    }
    
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summerize());
}

pub fn foo<T,U> (t: &T, u: &U) -> i32
    where T: Display + Clone,
        U: Clone + Summary
{
    //...
    5
}
