fn main() {
    another_function(245_326);
    position(345, 858, 5942);
    set_user(9383, "root", "123456");
}

fn another_function(x: u32) {
    println!("x: {x}")
}

fn position(x: i32, y: i32, z: i32) {
    println!("x: {x}, y: {y}, z: {z}");
}

fn set_user(id: u32, username: &str, password: &str) {
    println!("id: {id}, username: {username}, password: {password}");
}
