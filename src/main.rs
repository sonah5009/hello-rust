fn main() {
    let s = String::from("hello word");

    let word = first_word(&s);

    // s.clear(); // &mut String -- mutable reference 요구
    // immutable reference 인 word가 있으므로, word 가 살아 있는 동안 mutable reference 생성 불가
    // 컴파일 에러

    println!("first word: {}", word);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // index, 해당 요소의 reference
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}