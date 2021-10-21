struct Fib {
    c: u32,
    n: u32,
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;
        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib { c: 0, n: 1 }
}

fn main() {
    // prints the first 10 fibs
    for f in fib().take(10) {
        println!("{}", f);
    }

    // prints the next 10 fibs after first 10
    for f in fib().skip(10).take(10) {
        println!("{}", f);
    }
}
