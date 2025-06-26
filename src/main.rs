fn main() {
    // 슬라이스로써의 문자열 리터럴
    // String Literals as Slices
    let s = "Hello, world!";
    // s는 binary의 특정 지점을 가리키는 slice
    // data type: &'static str
    // &str 타입이므로 immutable reference. <- 문자열 리터럴 변경 불가의 이유
}

// String Slices as Parameters
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