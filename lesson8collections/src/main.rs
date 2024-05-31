fn main() {
    let a=[1,2,3];
    let mut v:Vec<i32>=Vec::new();
    v.push(1);
    v.push(2);
    let mut v2=vec![1,2,3,4,5];
    let mut third=&v2[2];
    println!("third is {}",third);
    third=&7;
    println!("v2 is {}",v2[2]);
    println!("third is {}",third);
    match v2.get(3) {
        Some(x) => println!("Found value {}",x),
        None => println!("No such index!"),
    }

    for i in &mut v2 {

        *i+=50;
        println!("{}",i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row=vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("hgfds")),
        SpreadsheetCell::Float(3.2),
    ];
    for c in &row{
        match c {
            SpreadsheetCell::Int(i)=>println!("Found int {}",i),
            _ => println!("Not an Int!")
        }
    }

    let mut s=String::from("dsfg");
    s.push_str("bar");
    println!("{}",s);
    s+="dfhsfds";
    println!("{}",s);
    hashmaps();
}

use std::collections::HashMap;
fn hashmaps() {
    let blue = String::from("vbnjkl");
    let green = String::from("dsg");

    let mut map=HashMap::new();
    map.insert(blue, 1);
    map.insert(green, 5);

    let team=map.get(&String::from("mohh"));
    match team {
        Some(i) => println!("The value is {}",i),
        None => println!("No value!")
    }
    for (key,val) in &map {
        println!("Key is {}, val is {}",key, val);
    }
    println!("{}",map.entry(String::from("nnnnn")).or_insert(30));
    println!("{}",map.entry(String::from("nnnnn")).or_insert(40));

    let mut map2 = HashMap::new();
    let text=String::from("Hello world wonderful world");
    for word in text.split_whitespace() {
        let count =map2.entry(word).or_insert(0);
        *count+=1
    }
    println!("{:?}",map2);
}