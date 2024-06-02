fn main() {
    let arr=vec![1,4,7,10,2];
    println!("{}",get_largest(arr));
    struct Point<T,U>{
        x: T,
        y: U
    }
    let pt=Point{x:5,y:3.2};

    impl<T,U> Point<T,U>{
        fn x(&self)->&T{
            &self.x
        }
    }

    impl<T> Point<T,f64>{
        fn y(&self)-> &f64{
            &self.y
        }
    }

}

fn get_largest<T: PartialOrd + Copy>(items: Vec<T>)->T{
    let mut max=items[0];
    for x in items{
        if x>max{
            max=x;
        }
    }
    max
}
