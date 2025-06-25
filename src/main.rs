fn main() {
    let condition = true;
    // 앞에가 5라서 뒤에도 같은 타입이여야 함
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
    // 5 출력
}