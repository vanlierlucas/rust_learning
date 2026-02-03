fn first_word(s: &str) -> &str{
    let byte = s.as_bytes();
    for (i, &it) in byte.iter().enumerate() {
        if it == b' ' {
            return &s[0..i];
        }
    }
    &s[0..]

}

fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("{}", word);

    let literal = "hi there";
    let word2 = first_word(literal);
    println!("{}", word2);
}
