use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// Methods
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.height, self.width, self.area());
    }
}

// Related Functions
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}

// traits
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} x {})", self.width, self.height,)
    }
}

fn main() {
    let o = Object {
        width: 30,
        height: 20,
    };

    let obj = Object::new(51, 38);

    o.show();
    obj.show();

    println!("{}", o);
    println!("{}", obj);
}
