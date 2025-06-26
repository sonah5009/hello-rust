fn main() {
    // dangling pointer
    let reference_to_nothing = dangle();
}

fn dangle() -> String { // String 직접 반환
// fn dangle() -> &String { // String의 참조자를 반환하겠다고 함
    let s = String::from("hello");
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // 이 함수는 빌린 값을 반환하고 있으나, 빌린 실제 값이 존재하지 않는다.

    // &s // String s의 참조자를 반환
    // s는 dangle 함수 내에서 생성 됐으므로 함수가 끝나면 free 됨.
    // 따라서 유효하지 않은 String을 가리키는 참조자를 반환하므로 에러가 난 것.

    s // String의 직접 반환해야함
}