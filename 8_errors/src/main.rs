use std::io;
use std::io::Read;
use std::fs::File;
use std::error::Error; // see crate::failure

// main can return Result
fn main() -> Result<(), Box<dyn Error>> { // see trair object
    catch_and_panic();
    do_panic();
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // ? can match to Result

    Ok(s)
    // fs::read_to_string("hello.txt") is even simplier
}

fn with_unwrap() {
    let f = File::open("hello.txt").unwrap(); // get value or panic!
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // same but panic! with message
}

fn catch_and_panic() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}

fn do_panic() {
    let vec = vec![1, 2, 3];
    let x = vec[99];
}
