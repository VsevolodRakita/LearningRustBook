fn main() {
    let s="dfsggses";
    let s2 = "xdcfvgbhnjmk";

    println!("{}",get_longest(s, s2));

    let novel=String::from("xcfvgbhjk. dfgghjk.");
    let sentence=novel.split('.').next().expect("oops");
    let i = Excerpt{
        part: sentence,
    };
}

fn get_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str{
    if s1.len()>s2.len(){
        return s1;
    }
    s2
}

struct Excerpt<'a>{
    part: &'a str,
}
