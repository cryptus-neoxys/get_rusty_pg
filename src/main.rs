fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn main() {
    let v = vec![1, 2, 3];

    println!("v: {}", v.into_iter().any(|i| i != 2));

    // Imperative approach
    let top = 10000;
    let mut c = 0;

    for n in 0.. {
        let x = n * n;

        if x >= top {
            break;
        } else if is_even(x) {
            c += x;
        }
    }

    println!("c = {}", c);
    // c = 0;
    // Functional approach
    c = (0..)
        .into_iter()
        .map(|n| n * n)
        .take_while(|&n| n < 10000)
        .filter(|&n| is_even(n))
        .fold(0, |s, i| s + i);

    println!("c = {}", c)
}
