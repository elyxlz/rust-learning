// fn main() {
//     let number: i32 = 1;
//
//     if number % 4 == 0 {
//         println!("num div 4");
//     } else if number % 3 == 0 {
//         println!("num div 3")
//     } else if number % 2 == 0 {
//         println!("num div 2")
//     } else {
//         println!("num not div")
//     }
// }

fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" };

    println!("the number is {number}")
}
