fn main() {
    let reference_to_nothing = dangle();
    // this code does not run because the reference is dangling
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // s will actually be freed here, so this pointer points to nothing
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s // if you return the string, it will be moved and not freed
      //this means that it will no longe be a dangling pointer
}
