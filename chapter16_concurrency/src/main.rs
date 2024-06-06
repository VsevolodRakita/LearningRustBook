use std::{thread, time::Duration};
use std::sync::mpsc::{self, Receiver, Sender};


fn main() {
    let handle = thread::spawn(||{
        for i in 0..10{
            println!("thread: {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 0..5{
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let (tx, rx):(Sender<String>,Receiver<String>)=mpsc::channel();

    thread::spawn(move ||{
        let msg=String::from("Hello!");
        println!("Sleeping...");
        thread::sleep(Duration::from_secs(3));
        tx.send(msg).unwrap();
    });

    loop {
        let recieved = rx.try_recv();
        match recieved {
            Err(e) => println!("Waiting..."),
            Ok(x) => {
                println!("Got it! {}",x);
                break;
            }
        }
    }

    let (tx, rx):(Sender<String>,Receiver<String>)=mpsc::channel();
    let tx2= tx.clone();
    thread::spawn(move ||{
        let vals = vec![String::from("1"),String::from("2"),String::from("3"),String::from("4")];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        let vals = vec![String::from("10"),String::from("20"),String::from("30"),String::from("40")];
        for val in vals{
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for recieved in rx{
        println!("{}",recieved);
    }
    foo();
}

use std::sync::{Mutex, Arc};

fn foo(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let j = i;
        let counter=counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("{} done! num is {}",j,*num);
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
