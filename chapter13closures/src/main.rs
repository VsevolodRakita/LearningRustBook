use std::collections::HashMap;
use std::time::Duration;
use std::thread;

fn main() {
    let expensive_closure=|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut ca=Cacher::new(expensive_closure);
    println!("{}", ca.value(5));
    println!("{}", ca.value(3));
    println!("{}", ca.value(4));
    println!("{}", ca.value(5));
}

struct Cacher<T>
where
    T:Fn(u32)-> u32,
{
    calculation: T,
    map: HashMap<u32,u32>,
}

impl<T> Cacher<T>
where
    T:Fn(u32)-> u32,
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32{
        if self.map.contains_key(&arg){
            return self.map[&arg];
        }
        let v=(self.calculation)(arg);
        self.map.insert(arg, v);
        v
    }
}