fn main() {
    let x = five();
    let y = four();
    let z = plus_nine(4);
    let get_true = get_bool(1);
    let get_false = get_bool(2);

    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");
    println!("get_true: {get_true}");
    println!("get_false: {get_false}");
}

fn five() -> i32 {
    5
}

fn four() -> u32 {
    4
}

fn plus_nine(x: i32) -> i32 {
    x + 9
}

fn get_bool(x: i32) -> bool {
    if x == 1 {
        return true;
    }

    false
}
