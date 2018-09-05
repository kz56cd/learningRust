fn main() {
    let x: i32 = 2;
    {
        let x = 10;
        println!("{}", x);
    }
    println!("{}", x);
    let x = 399;
    println!("{}", x);
}
