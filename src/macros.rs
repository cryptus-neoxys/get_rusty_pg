macro_rules! a_macro {
    () => {
        println!("This is a macro")
    };
}

macro_rules! x_and_y {
    (x => $e:expr) => {
        println!("X: {}", $e)
    };
    (y => $e:expr) => {
        println!("Y: {}", $e)
    };
}

macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! print_ex {
    ($e:expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    };
}

macro_rules! exame {
    ($a:expr; and $b:expr) => {
        println!(
            "{:?} and {:?} = {:?}",
            stringify!($a),
            stringify!($b),
            $a && $b
        )
    };

    ($a:expr; or $b:expr) => {
        println!(
            "{:?} or {:?} = {:?}",
            stringify!($a),
            stringify!($b),
            $a || $b
        )
    };
}

macro_rules! compr {
    ($id1: ident | $id2: ident <- [$start: expr ; $end: expr], $cond: expr) => {{
        let mut v = Vec::new();

        for num in $start..=$end {
            if ($cond(num)) {
                v.push(num);
            }
        }
        v
    }};
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn is_odd(x: i32) -> bool {
    !is_even(x)
}

macro_rules! new_map {
    ($($key: expr => $val: expr),+) => {
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

macro_rules! calc {
    (eval $e: expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    };

    (eval $e: expr, $(eval $es: expr),+) => {
        {
            calc!{eval $e};
            calc!{$(eval $es),+}
        }
    }
}

fn main() {
    a_macro!();

    x_and_y!(x => 10);
    x_and_y!(y => 10 + 10);

    build_fn!(hello);
    hello();

    print_ex!({
        let a = 15;
        let b = 20;
        a + b * 12 + 300
    });

    exame!(1+1 == 2; and 3+1 ==2);
    exame!(1+1 == 2; or 3-1 ==2);

    let evens = compr!(x | x <- [1; 10], is_even);
    println!("{:?}", evens);
    let odds = compr!(x | x <- [1; 10], is_odd);
    println!("{:?}", odds);

    let m = new_map! {
        "a" => 10,
        "b" => 20,
        "c" => 30
    };
    println!("{:?}", m);

    calc!(
        eval 12 / 4,
        eval 12 + 4,
        eval (20 - 2) * 5
    )
}
