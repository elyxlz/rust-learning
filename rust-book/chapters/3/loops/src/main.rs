// fibonacci
// 1 1 2 3 5 8 13 21

fn main() {
    let mut previous_values: [i32; 2] = [0, 1];
    while previous_values[1] < 200 {
        let next_value: i32 = previous_values[0] + previous_values[1];
        previous_values[0] = previous_values[1];
        previous_values[1] = next_value;
        println!("{next_value}");
    }
}
