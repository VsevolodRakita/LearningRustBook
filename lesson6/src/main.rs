
enum IPAddrKind{
    V4(i8,i8,i8,i8),
    V6(String),
}

struct IpAddr{
    kind: IPAddrKind,
    address: String,
}

enum Message{
    Quit,
    Move { x: i32, y:i32},
    Write(String),
    ChangeColour(i32,i32,i32),
}

impl Message{
    fn foo(){
        println!("Hello");
    }
}
fn main() {
    let six= IPAddrKind::V6;
    let four = IPAddrKind::V4;

    let local_host = IpAddr{
        kind: IPAddrKind::V4(2,3,4,5),
        address: String::from("127.0.0.1"),
    };
    
    let some_num= Some(5);
    let some_str = Some(String::from("hgfds"));
    let no_number: Option<i32> = None;

    let x=5;
    let y: Option<i32> =None;

    let sum = x+y.unwrap_or(0);
    let mes =Message::Write(String::from("zxcfghj"));
    match mes {
        Message::Quit => println!("Go away"),
        _ => println!("Something Else"),
    }

    if let Message::Quit = mes{
        println!("I said go!")
    }
    else{
        println!("Something Else!")
    }

}

fn plus_one(x: Option<i32>) ->Option<i32>{
    match x {
        None =>None,
        Some(i) => Some(i+1),
    }
}
