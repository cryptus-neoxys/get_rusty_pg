fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let res = division(50.12, 0.0);
    match res {
        Some(x) => println!("{:.8}", x),
        None => println!("Cannot divide"),
    }
}
