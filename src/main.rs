fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // 소유권을 넘기지 않고 해당값을 참조할 수 있도록 함

    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

