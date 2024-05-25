fn main() {
    let x = 5;

    let x = x + 7;

    {
        let x = x * 4;
        println!("x: {x}")
    }

    println!("x: {x}")
}
