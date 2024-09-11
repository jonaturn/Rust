fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let z = 3;
        z + 1 // adding a semicolon here breaks the code cause z immutable
    };

    let a = five();
    println!("The value of y is: {y}");
    println!("The value of y is: {a}"); // cant do {five()}
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
