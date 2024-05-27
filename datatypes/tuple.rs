fn main() {
    let tup: (i32, u8, f64) = (25349, 255, 99.9);

    // destructuring
    let (x, y, z) = tup;

    println!("tup: {} {} {}", x, y, z);

    // accessing a tuple element using index
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    println!("tup: [0]:{}, [1]:{}, [2]:{}", x, y, z);
}
