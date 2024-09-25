use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("enter an array of integers");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //use vectors because size is unkown
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("invalid integer"))
        .collect();

    numbers.sort();
    println!("median: {}", &numbers[(numbers.len() - 1) / 2]); // borrow not transfer ownership

    // let mut integers = HashMap::new();

    // // i just want to read so no mut no transfer
    // for number in &numbers {
    //     if (integers.contains_key(number)) {
    //         integers.insert(number, integers.get(number).unwrap() + 1);
    //     } else {
    //         integers.insert(number, 1);
    //     }
    // }

    // let mut count = 0;
    // let mut mode = 0;

    // for (&key, &value) in &integers {
    //     if (value > count) {
    //         count = value;
    //         mode = *key;
    //     }
    // }

    // Build a frequency table using HashMap
    for &number in &numbers {
        // Dereference number when inserting into the HashMap
        let count = integers.entry(number).or_insert(0); // Efficiently update or insert
        *count += 1;
    }

    // Finding the mode
    let mut count = 0;
    let mut mode = 0;

    for (&key, &value) in &integers {
        if value > count {
            count = value;
            mode = key;
        }
    }

    println!("mode: {}", &mode);
}
