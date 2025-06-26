fn main() {
    // dangling pointer
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // String의 참조자를 반환하겠다고 함
    let s = String::from("hello");
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // 이 함수는 빌린 값을 반환하고 있으나, 빌린 실제 값이 존재하지 않는다.

    &s // String s의 참조자를 반환
} // 여기서 s는 스코프 밖으로 벗어나고 버려짐. 해당 메모리 해제.
// 위험