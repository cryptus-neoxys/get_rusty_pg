// Recursive Data Type -> Heap
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    End,
}

use List::Cons;
use List::End;

fn run<F>(f: F)
where
    F: Fn(),
{
    f()
}

fn add3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

struct A<F: Fn(i32) -> i32> {
    f: F,
}

fn prin() {
    println!("This is a function")
}

fn create() -> Box<Fn()> {
    Box::new(move || println!("this is a closure in a box"))
}

fn main() {
    // Box smart pointer
    let b = Box::new(10);
    println!("{}", b);

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
    println!("{:?}", l);

    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *z == *x {
        println!("*z[Box::new(y)] == *x[&y]")
    }

    let anon_fn = |a| a + 1;
    let x = 10;
    println!("{}", anon_fn(x));

    let pr = || println!("This is a closure");
    pr();

    let mut c = 0;
    let mut inc = || {
        c += 1;
        println!("Incremented by 1, c: {}", c);
    };

    inc();
    inc();
    inc();

    run(pr);
    run(prin);

    let x = |i| i * 10;
    println!("3 * 10 = {}", add3(x));

    let a = A { f: x };

    let boxed_closure = create();
    boxed_closure();
    run(boxed_closure);
}
