fn main() {
    let mut stack = vec![1,2,3];
    while let Some(top) = stack.pop(){
        println!("{}",top);
    }

    let x = Some(11);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y= {}",y),
        _ =>println!("Don't know.")
    }

    let x = 4;
    match x {
        1 | 2 |3 => println!("Something"),
        4..=7 => println!("Something else"),
        _ => println!("Default")
    }

    let p=Point{x:2, y:5};
    let Point{x:a,y}=p;
    assert_eq!(2,a);
    assert_eq!(5,y);

    match p {
        Point{x,y: 0} => println!("Point is on x axis"),
        Point{x:0, y} => println!("Point is on y axis"),
        Point{x,y} => println!("Point is somewhere else, {:?},{:?}",x,y)
    }

    foo(2,3);

    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n==y => println!("x is equal to y"),
        _ => println!("x isn't equal to y")
    }
}


struct Point{
    x: i32,
    y: i32,
}

fn foo(_: i32, y:i32){
    println!("Do something with y={}",y);
}

