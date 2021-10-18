#![allow(dead_code)]

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Rectangle { width: u32, height: u32 },
    Square(u32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14159265 * (r * r),
            Shape::Rectangle { width, height } => (width * height) as f64,
        }
    }
}

fn main() {
    let u = Direction::Up(Point { x: 12, y: 24 });
    let k = u.match_direction();
    let x = k.destruct();

    println!("{}", x);

    let u = 10;
    let v = &u;
    let ref w = u;

    if v == w {
        println!("v == w");
    }

    let r = Shape::Rectangle {
        width: 12,
        height: 13,
    };
    let c = Shape::Circle(14.0);
    let s = Shape::Square(12);

    let rr = r.area();
    let cr = c.area();
    let sr = s.area();

    println!("Circle: {:.4}\nSquare: {}\nRect: {}", cr, sr, rr)
}
