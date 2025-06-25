use std::io;    // prelude: 표준 라이브러리에 정의된 아이템 집합. 
// use 문 사용: 원하는 type이 prelude에 없는 경우

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // 변수 기본 속성: immutable
    // &guess -> &mut guess: 가변(mutable)로 변경
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}"); // {} : placeholder
}
// crate: rust code 파일들의 모음
// binary crate: 
// `rand` crate: Cargo.toml의 [dependencies] 에 rand 추가
// 예) rand = "0.8.5"
// Semantic Versioning: 0.8.5 이상, 0.9.0 아래의 모든 버전. ^0.8.5의 축약형

// cargo build 시,
// rand 가 동작하기 위한 다른 crate들도 가져옴을 알 수 있음