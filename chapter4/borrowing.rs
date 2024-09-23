fn main() {
    let mut s = String::from("hello");

    println!("{s}");

    change(&mut s);

    println!("{s}");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1 = s1
    // println!("s1 is {s1}");
    // println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // println!("s1 is {s1}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // no ownership taken
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
