trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (std::f64::consts::PI * self.radius * self.radius) as u32
    }
}

fn fib(n: i32) -> i32 {
    match n {
        0..=1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    let c = Circle { radius: 2.11 };
    let r = Rectangle { x: 2, y: 30 };

    println!("{} {}", c.area(), r.area());

    println!("{}", fib(10));
}
