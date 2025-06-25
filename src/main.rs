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
