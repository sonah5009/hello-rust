fn main() {
    let x = 1;
    {
        let x = 2;
        println!("scope: {x}");
    }
    println!("밖: {x}");
    // let 없이 shadowing 불가능
    // x = 3;
}
