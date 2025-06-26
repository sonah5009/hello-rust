fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
    println!("{}", r1);


}

fn change(some_string: &mut String){
    // mutable reference
    // 가변 참조자 생성
    some_string.push_str(", world");
    // 제약조건: 하나의 값에 대한 가변 참조자는 딱 하나만 생성 가능. 만약 2개면 작동 불가
}

