fn main() {
    let x = 2.0; //f64 64 bits
    let y: f32 = 3.0; //f32 32 bits

    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //destructuring a tuple
    let (x, y, z) = tup;
    // more destructuring
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    //arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //in square brackets, the type and the number of elements

    let a = [3; 5]; // [3, 3, 3, 3, 3]
}
