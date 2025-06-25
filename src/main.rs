fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1,2,3,4,5];  // 배열이 5개의 요소
    let a0 = a[0];
    println!("{a0}");
    
    let a = [3; 5];
    // [3, 3, 3, 3, 3]
    let a1 = a[1];
    println!("{a1}");
    
    // Runtime error
    // 명시한 인덱스가 배열 길이보다 작은지 검사
    // 인덱스가 배열 길이보다 크거나 같을 경우 -> 패닉 (panic)
}
