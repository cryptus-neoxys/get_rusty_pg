use std::ops;

#[derive(Debug, Clone, Copy)]
struct A(i32);

struct B(i32);

struct X;
struct Y;
#[derive(Debug)]
struct XY;

#[derive(Debug)]
struct YX;

impl ops::Add<Y> for X {
    type Output = XY;

    fn add(self, _rhs: Y) -> XY {
        XY
    }
}

impl ops::Add<X> for Y {
    type Output = YX;

    fn add(self, _rhs: X) -> YX {
        YX
    }
}

struct D {
    s: String,
}

impl Drop for D {
    fn drop(&mut self) {
        println!("dropped: {}", self.s);
    }
}

fn main() {
    let a = A(32);
    let b = B(1);

    let c = a;
    println!("{:?}", a);

    println!("{:?}", X + Y);
    println!("{:?}", Y + X);

    let d = D { s: "D".to_string() };
    {
        let e = D { s: "E".to_string() };
        {
            let f = D { s: "F".to_string() };
        }
    }
    drop(d);
    println!("finished")
}
