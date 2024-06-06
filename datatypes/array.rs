fn main() {
    let _int_array = [1, 2, 3, 4];
    let _uint_array: [u32; 5] = [42, 77, 21, 7585, 2734];

    println!(
        "{}, {}, {}, {}",
        _int_array[0], _int_array[1], _int_array[2], _int_array[3],
    );

    println!(
        "{}, {}, {}, {}, {}",
        _uint_array[0], _uint_array[1], _uint_array[2], _uint_array[3], _uint_array[4],
    );
}
