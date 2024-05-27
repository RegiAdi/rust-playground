fn main() {
    // signed integers
    let _8bit_int: i8 = 127;
    let _16bit_int: i16 = 32_767;
    let _32bit_int: i32 = 2_147_483_647;
    let _64bit_int: i64 = 9_223_372_036_854_775_807;
    let _128bit_int: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;

    println!("signed integer");
    println!("i8: {} to {}", i8::MIN, i8::MAX);
    println!("i16: {} to {}", i16::MIN, i16::MAX);
    println!("i32: {} to {}", i32::MIN, i32::MAX);
    println!("i64: {} to {}", i64::MIN, i64::MAX);
    println!("i128: {} to {}", i128::MIN, i128::MAX);
    println!(
        "isize: {} bit, {} to {}",
        isize::BITS,
        isize::MIN,
        i128::MAX
    );

    // unsigned integers
    let _8bit_uint: u8 = 255;
    let _16bit_uint: u16 = 65535;
    let _32bit_uint: u32 = 4_294_967_295;
    let _64bit_uint: u64 = 18_446_744_073_709_551_615;
    let _128bit_uint: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

    println!("\nunsigned integer");
    println!("u8: {} to {}", u8::MIN, u8::MAX);
    println!("u16: {} to {}", u16::MIN, u16::MAX);
    println!("u32: {} to {}", u32::MIN, u32::MAX);
    println!("u64: {} to {}", u64::MIN, u64::MAX);
    println!("u128: {} to {}", u128::MIN, u128::MAX);
    println!(
        "usize: {} bit, {} to {}",
        usize::BITS,
        usize::MIN,
        usize::MAX
    );

    // floating-points (IEEE 754)

    println!("\nfloating-points (IEEE 754)");
    println!("f32 layout");
    println!("     1       8                   23                   bit");
    println!("     . | .... .... | .... .... .... .... .... ... |");
    println!("  sign | exponent  |        fraction              |");
    let _float32: f32 = 127.481_029_348;

    println!("f32: {}", _float32);

    println!("\nf64 layout");
    println!("   1       11         52        bit");
    println!(" sign | exponent | fraction |");
    let _float64: f64 = 4_747_374.234_839_283_477_34;

    println!("f64: {}", _float64);

    // numeric operations
    // addition
    let sum = 9 + 20;

    // subtraction
    let diff = 99.9 - 4.3;

    // multiplication
    let product = 20 * 2;

    // division
    let quotient = 99.9 / 66.6;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    println!("\nnumeric operations");
    println!("addition");
    println!("9 + 20 = {}", sum);

    println!("subtraction");
    println!("99.9 - 4.3 = {}", diff);

    println!("multiplication");
    println!("20 * 2 = {}", product);

    println!("division");
    println!("99.9 / 66.6 = {}", quotient);
    println!("-5 / 3 = {}", truncated);

    println!("remainder");
    println!("43 % 5 = {}", remainder);

    // booleans
    let _true = true;
    let _false: bool = false; // with explicit type annotation

    println!("\nbooleans");
    println!("true: {}", _true);
    println!("false: {}", _false);

    // characters
    let _char = 'z'; // char use single quote
    let _char_with_annotation: char = 'Z'; // with explicit type annotation
    let _char_emoji = '😻';

    println!("\ncharacter (4 bytes)");
    println!("char: {}", _char);
    println!("emoji: {}", _char_emoji);
}
