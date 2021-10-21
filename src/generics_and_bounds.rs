use std::{fmt, future::Ready, ops::Mul};

#[derive(Debug)]
struct Square<T> {
    x: T,
}

fn p<T: fmt::Debug>(x: T) {
    println!("{:?}", x);
}

struct A<T> {
    x: T,
}

impl<T> A<T> {
    fn item(&self) -> &T {
        &self.x
    }
}

struct X<U, V> {
    x: U,
    y: V,
}

struct Y<V> {
    x: V,
    y: V,
}

trait Shape<T> {
    fn area(&self) -> T;
}

struct Rectangle<T: Mul> {
    w: T,
    h: T,
}

struct Circle<T: Mul> {
    r: T,
}

impl<T: Copy> Shape<T> for Rectangle<T>
where
    T: Mul<Output = T>,
{
    fn area(&self) -> T {
        self.h * self.w
    }
}

// !Doesn't work due to type incompatibility
impl<T: Copy> Shape<T> for Circle<T>
where
    T: Mul<Output = T>,
{
    fn area(&self) -> T {
        (self.r * self.r) * 3.14 // !return <T> not {float}
    }
} // !To Fix

fn main() {
    let s = Square { x: 1 };
    let s = Square { x: 1.0 };
    let s = Square { x: "1" };
    let s = Square { x: s };

    p(10);
    p(String::from("Dev"));
    p(s);

    let a = A { x: "Generics" };
    p(a.item());

    let r1 = Rectangle { h: 12, w: 10 };
    let r2 = Rectangle { h: 1.01, w: 12.45 };
    println!("int R: area={}, fl R: area={}", r1.area(), r2.area());
}
