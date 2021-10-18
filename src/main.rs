use std::collections::HashMap;

#[derive(Debug)]
enum Multi {
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {
    let mut v = Vec::new();

    for i in &v {
        println!("{}", i)
    }
    v.push(13);
    for i in &v {
        println!("{}", i)
    }

    println!("{:?}", v.pop());
    println!("v: {:?}, l: {}, cap: {}", &v, v.len(), v.capacity());
    println!("{:?}", v.pop());

    let m = vec![
        Multi::Float(1.1),
        Multi::Int(32),
        Multi::Text("Scuk it".to_string()),
    ];

    println!("{:?}", &m);

    let mut hm = HashMap::new();
    hm.insert(String::from("Dev"), 10);
    hm.insert(String::from("Nishant"), 6);
    hm.insert(String::from("Radhika"), 4);

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }
    hm.remove(&String::from("Dev"));

    match hm.get(&String::from("Dev")) {
        Some(&v) => println!("{}", v),
        _ => println!("No val found for Dev"),
    }

    let j = Some('x');

    if let Some(q) = j {
        println!("{}", q);
    } else {
        {}
    }

    let mut o = Some(0);

    // loop {
    //     match o {
    //         Some(b) => {
    //             if b > 19 {
    //                 println!("Quit");
    //                 o = None
    //             } else {
    //                 println!("{}", b);
    //                 o = Some(b + 2);
    //             }
    //         }
    //         _ => {
    //             break;
    //         }
    //     }
    // }

    while let Some(i) = o {
        if i > 19 {
            println!("Quit");
            o = None;
        } else {
            println!("{}", i);
            o = Some(i + 2);
        }
    }
}
