fn main() {
    let mut x = 5; // 변수를 mutable로 만듦
    // let x = 5;   // immutable이므로 불가능
    println!("{x}");
    x=6;
    println!("{x}");
}
