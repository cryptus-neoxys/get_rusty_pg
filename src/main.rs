fn pr<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() == y.len() {
        x
    } else {
        y
    }
}

struct A<'a, 'b> {
    x: &'a str,
    y: &'b str,
}

fn main() {
    let x;
    // {
    let y = 10;

    x = &y;
    // }
    println!("x: {}", x);

    let a = "a string";
    let b = "b string";

    let c = pr(a, b);
    println!("c: {}", c);

    let ast = A {
        x: "Hello",
        y: "There",
    };
    println!("{} {}!", ast.x, ast.y)
}
