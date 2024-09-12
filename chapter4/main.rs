fn main() {
    let s1 = String::from("hello");
    // let s2 = s1; // here rust frees s1
    let s2 = s1.clone(); // now the code below works because deep copy
                         //s1 is not freed anymore but this is expensive

    println!("{s1}, world!"); // error: s1 is free and invalid alr
}
