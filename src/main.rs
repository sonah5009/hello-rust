fn main() {
    let x = 1;
    let x = x+1;
    println!("{x}");
    // let 없이 shadowing 불가능
    // x = 3;
    println!("{x}");
}
