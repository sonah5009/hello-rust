fn main() {
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // index, 해당 요소의 reference
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}