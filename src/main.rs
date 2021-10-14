struct Object {
    width: i32,
    height: i32,
}

fn area(obj: &Object) -> i32 {
    obj.width * obj.height
}

fn main() {
    let o = Object {
        width: 30,
        height: 20,
    };

    println!("{}x{} with area: {}", o.height, o.width, area(&o));
}
