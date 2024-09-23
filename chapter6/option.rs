fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);
    println!("{:?}", six); //debug print
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // if missing rust will throw error
        Some(i) => Some(i + 1),
    }
}
