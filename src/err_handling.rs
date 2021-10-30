use core::panic;
use std::{
    fmt::Result,
    fs::File,
    io::{self, ErrorKind, Read},
    num::IntErrorKind,
};

fn exit(code: Option<i32>) {
    match code {
        Some(0) => panic!("got 0, panicked"),
        Some(x) => println!("Non-0: {}, no-exit", x),
        None => println!("received nothing"),
    }
}

fn read_file() -> io::Result<String> {
    // Java-Like
    // let f = File::open("text.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => return Err(e),
    // }

    // Rustik
    // let mut f = File::open("text.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // Optional Chaining: The Origin
    let mut s = String::new();
    File::open("text.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    exit(Some(1));
    exit(Some(10));
    exit(None);
    // exit(Some(0));

    // // Feels like Java
    // let f = File::open("text.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref err) if err.kind() == ErrorKind::NotFound => match File::create("text.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             panic!("could not create file: {:?}", e)
    //         }
    //     },
    //     Err(err) => {
    //         panic!("could not open file: {:?}", err)
    //     }
    // };

    // // More like rust
    // let f = File::open("text.txt").expect("Unable to open file");
    read_file().unwrap();
}
