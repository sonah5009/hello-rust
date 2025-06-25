fn main() {
    let number = 3;

    // 불가능 expected `bool`
    // bool이 아니라서
    // if number {
    //     println!("number was something other than zero");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }
}