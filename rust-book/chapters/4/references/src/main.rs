fn second_word_index(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut first_word = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_word > 0 {
                return (first_word + 1, i);
            }
            first_word = i;
        }
    }
    (0, 0)
}
fn main() {
    let a: String = String::from("bananaasasdsd hello yo");
    let i = second_word_index(&a);
    println!("{}", &a[i.0..i.1]);
}
