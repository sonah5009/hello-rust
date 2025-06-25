use std::io;

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

    println!("You guessed: {guess}"); // {} : placeholder
}
