fn main() {
    let x=6;
    println!("{}", foo(5,x));
    let tupple=(32,"cat");
    let arr=[1,7];
    let arr2=["dog","sdfgd","sadf"];
    let (x1,x2)=tupple;
    let z2=arr2[1];

}

fn foo(x:i8, y:i8) ->i8{
    let a=1..6;
    for item in a {
        println!("{}", item)
    }
    x+y
}
