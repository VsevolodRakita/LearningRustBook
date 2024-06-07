use chapter17oop::{Draw, Post2};
use chapter17oop::Post;


struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //something...
    }
}

fn main() {
    let mut post= Post::new();
    post.add_text("sdfgh");
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("sdfgh", post.content());

    let mut post2=Post2::new();
    post2.add_text("sfgads");
    let post2=post2.request_review();
    let post2 = post2.approve();
    assert_eq!("sfgads",post2.content());
}


