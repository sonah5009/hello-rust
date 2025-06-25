use std::{cmp::Ordering, io};

use rand::Rng;    // prelude: 표준 라이브러리에 정의된 아이템 집합. 
// use 문 사용: 원하는 type이 prelude에 없는 경우

fn main() {
    println!("Guess the number!");

    let secrete_number = rand::thread_rng().gen_range(1..=100);

    println!("The secrete number is: {secrete_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // shadow: 값 덮어쓰기
    // trim 메서드: 처음과 끝부분의 공백문자들을 제거. `5\n` -> `5`
    // parse 메서드: 문자열 -> 숫자로 변경
    // expect 메서드: parse 실패 시 Result Err 반환 시
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed: {guess}"); // {} : placeholder

    match guess.cmp(&secrete_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
