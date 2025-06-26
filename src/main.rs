fn main() {
    // case 2: mutable, immutable reference 혼용 
    // -> (immutable reference) r1, r2의 사용 종료 후, 
    // (mutable reference) r3 생성
    // 즉, 서로의 scope가 겹치지 않게 함

    // let mut s = String::from("hello");

    // let r1 = &s; // 문제없음
    // let r2 = &s; // 문제없음
    // let r3 = &mut s; // 큰 문제

    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2);
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

    let r3 = &mut s; // 문제없음
    println!("{}", r3);


}
