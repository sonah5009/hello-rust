use std::{cmp::Ordering, io};

use rand::Rng;    // prelude: 표준 라이브러리에 정의된 아이템 집합. 
// use 문 사용: 원하는 type이 prelude에 없는 경우

fn main() {
    println!("Guess the number!");

    let secrete_number = rand::thread_rng().gen_range(1..=100);

    println!("The secrete number is: {secrete_number}");

    loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // 잘못된 입력값 처리하기
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}"); // {} : placeholder
        
        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
