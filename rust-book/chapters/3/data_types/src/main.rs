use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Expected a number");

    let element = a[index];

    println!("the value of element at index {index} was {element}")
}
