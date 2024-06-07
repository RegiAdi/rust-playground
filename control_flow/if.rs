fn main() {
    let number = 4;

    if number <= 15 && number >= 10 {
        println!("less than 15");
    } else if number < 10 && number >= 5 {
        println!("less than 10");
    } else {
        println!("less than 5");
    }

    let get_number = if number > 0 { 4 } else { 0 };

    println!("get_number: {get_number}");
}
