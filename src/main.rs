fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // 소유권을 넘기지 않고 해당값을 참조할 수 있도록 함

    println!("The length of '{}' is {}.", s1, len);

}

fn change(some_string: &String){
    some_string.push_str(", world");
    // some_string은 borrow하는 참조자이므로 소유권이 없음. 즉, 수정 불가능
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

