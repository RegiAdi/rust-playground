fn main() {
    let mut x = 8;
    let mut y = 16; // unused mutable
    let mut z = 32; // unused mutable

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    x = 64;
    println!("x: {}", x);

    // mismatched types, we're not allowed to mutate a variable's type
    let mut spaces = "    ";
    spaces = spaces.len();
}
