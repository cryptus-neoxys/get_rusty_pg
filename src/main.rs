fn take(v: Vec<i32>) {
    println!("Took the v: {}", v[10] + v[100]);
}

fn cop(a: i32, b: i32) {
    println!("Copied the i: {} {}", a, b);
}

fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[100] + (*v)[150]);
}

fn borrow2(v: &Vec<i32>) {
    println!("{}", v[100] + v[150]);
}

fn main() {
    // scope
    // let a = 1;
    // {
    //     let b = 10;
    // }
    // a += b;

    // references
    let x: Vec<u32> = Vec::new();
    let y = &x;
    println!("{:?}", x);

    // moving
    let mut v = Vec::new();
    for i in 1..1000 {
        v.push(i);
    }

    // gets taken
    take(v);
    // println!("{}", v[1]);
    println!("Finished taking");

    // copying
    let a = 11;
    let b = 24;

    cop(a, b);

    println!("Finished copying, have a: {} b: {}", a, b);

    // borrowing
    let mut v2 = Vec::new();

    for i in 1..1000 {
        v2.push(i);
    }

    v2 = re(v2);

    println!("Still owns v: {} {}", v2[0], v2[10]);
    borrow1(&v2);
    println!("Still owns v: {} {}", v2[0], v2[10]);
    borrow2(&v2);
    println!("Still owns v: {} {}", v2[0], v2[10]);

    // a bit more complex
    let v3 = vec![2, 1, 4, 3, 5, 4, 6, 3, 5, 3, 2, 3, 7, 3, 6, 3, 4, 5];

    for &i in &v3 {
        let r = count(&v3, i);
        println!("{} is repeated {} times", i, r);
    }
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}
