fn main() {
    let t = (1, 'a', false);
    let f = (2, (1, 'a', false));

    println!("{} {}", t.0, t.1);
    println!("{:#?}", f.1);

    // Arrays and Slices
    let xs: [i32; 5] = [5, 1, 4, 2, 3];
    println!("{} {}", xs[2], xs.len());

    let ys = &xs[2..5];
    println!("{:?} {:?}", ys, xs);

    // Strings and String slices
    let s = "Str";
    let ss = String::from("String");
    let fs = &s[1..3];
    println!("{} {} {}", s, fs, &ss[2..5]);
    // Concatenating Strings
    let h = "Hello, ".to_string();
    let w = "World!".to_string();
    let hw = h + &w;
    println!("{}", hw);
}
