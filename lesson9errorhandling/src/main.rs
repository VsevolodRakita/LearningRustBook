use std::{env,fs::File,io::ErrorKind};



fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    //a()
    //open_a_file()
    open_a_file_2()

}

fn open_a_file_2(){
    let f = File::open("hello.txt").expect("Failed!");
    println!("Done!")
}

fn open_a_file(){
    let f = File::open("hello.txt");
    let f=match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(file2) => file2,
                Err(error2) => panic!("Failed to create file: {:?}",error2)
            },
            _ => panic!("There was some other error: {:?}", error),
        }
    };
    println!("Done!")
}

fn a(){
    b()
}

fn b(){
    c(22)
}

fn c(x:i32){
    if x==22{
        panic!("Don't pass 22")
    }
}