fn main() {
    // dangling pointer
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    // 0 1 2 3 4 5 6 7 8 9 10
    // h e l l o   w o r l d
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