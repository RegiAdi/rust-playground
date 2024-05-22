// This is a comment
/*
    This is a comment
*/

// This is the main function
fn main() {
    println!("1 Hello World!");
    println!("2 Hello World!");
    println!("3 Hello World!");

    println!("{} days", 365);
    println!("{} days, {} hours", 365, 24);
    println!("{} days, {} hours, {} minutes", 365, 24, 60);

    println!("{0}, this is {1}. {1}, this is {0}", "Foo", "Bar");
    println!("{0} {0} {1} {1} {0} {0}", "Foo", "Bar");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
}
