fn main() {
    // dangling pointer
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();  // String을 비워서 ""으로 만듦
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