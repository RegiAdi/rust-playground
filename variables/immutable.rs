fn main() {
    // immutable, value can't be changed
    let x = 8;
    println!("x: {}", x);

    let y = 128;
    println!("y: {}", y);

    // immutability error
    x = 44;
    println!("x: {}", x);

    y = 256;
    println!("y: {}", y)
}
