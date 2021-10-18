fn main() {
    let n = 6;

    let x = if n < 6 { "Test" } else { "Spec" };
    println!("n: {}, x: {}", n, x);

    let mut c = 4;
    loop {
        println!("{}", c);
        if c <= 0 {
            break;
        }
        c -= 1;
    }

    'a: loop {
        println!("loop a");
        'b: loop {
            println!("loop b");

            'c: loop {
                println!("loop c");

                break 'b;
            }
        }
        break 'a;
    }

    // break assignment
    let f = loop {
        break 10;
    };

    println!("f: {}", f);

    // while
    let mut d = 0;

    while d < 2 {
        println!("d: {}", d);
        d += 1;
    }

    // for in

    // each
    let v = vec![10, 100, 1000];
    for i in v {
        println!("i: {}", i);
    }
    // range
    for i in 1..=5 {
        println!("{}", i * 10);
    }

    // match (switch on STEROIDS)
    let m = 10;
    // other
    match m {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("unrecognized"),
    }
    // rust
    match m {
        1 => println!("The one!"),
        2 | 3 | 5 | 7 => println!("Prime Time"),
        12..=20 => println!("Teen"),
        _ => println!("meh"),
    }
    // more rust
    let p = (1, -9);
    match p {
        (0, y) => println!("y: {}", y),
        (x, -9) => println!("x: {}", x),
        _ => println!("matched none"),
    }
    let p2 = (2, -2);
    match p2 {
        (x, y) if x == -y => println!("{} + {} = 0", x, y),
        (x, y) if x == y => println!("x = y = {}", x),
        (x, _) if x % 2 == 0 => println!("x is even: {}", x),
        _ => println!("no match found"),
    }
    let n = match m {
        n @ 1..=12 => n,
        n @ 13..=19 => n,
        _ => 0,
    };
    println!("n binds: {}", n)
}
