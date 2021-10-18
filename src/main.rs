use std::fs::File;

fn main() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("There was an error opening file: {:?}", err)
        }
    };
}
