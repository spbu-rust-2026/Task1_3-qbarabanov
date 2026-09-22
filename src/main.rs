use std::fs;
use std::io;
use std::path::Path;
use std::println;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error!");

    let path = Path::new(input.trim());
    let file = fs::File::open(path);
    let result = file.is_ok();
    if result {
        let bytes = fs::read(path);
        match bytes {
            Ok(_) => println!("success"),
            Err(_) => println!("failure"),
        }
    } else {
        println!("failure");
    }
}
