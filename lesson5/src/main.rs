struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self)->u32{
        self.height*self.width
    }

    fn can_hold(&self, other: &Rectangle) ->bool{
        self.width>other.width && self.height> other.height
    }
}

fn main() {
    let mut user1 = User{
        email: String::from("sdfgfsdds"),
        username: String::from("dfrtrt"),
        sign_in_count: 0,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("gdrgergae");
    println!("{}", user1.username);

    let user2 = build_user(String::from("dsfgsdeeeeeee"), String::from("cvbnmklkkk"));

    struct Point(i32,i32,i32);
    struct Colour(i32,i32,i32);

    let rec1 = Rectangle{
        width: 5,
        height: 10
    };

    println!("{}", rec1.area());    
    println!("rec1: {:#?}", rec1);
}

fn build_user(email: String, username: String) -> User{
    User { username, email, sign_in_count: 1, active: true }
}

/* 
fn area(rec: &Rectangle) -> u32{
    rec.width*rec.height
}
*/