// print char fn

pub fn print_char() {
    let c = 'c';
    let heart_eyed_cat = '😻';

    println!("Char: {}", c);
    println!("Heart eyed cat: {}", heart_eyed_cat);
}

pub fn print_string() {
    let s = "Hello, world!";
    let s2 = String::from("Hello, world!");

    println!("String: {}", s);
    println!("String: {}", s2);
    println!("Length of string: {}", s.len());
}

pub fn print_string_slices() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[7..12];

    println!("Hello, world! [0..5]: {}", hello);
    println!("World, world! [7..12]: {}", world);
}

pub fn print_string_ranges() {
    let s = String::from("Hello, world!");
    let hello = &s[..5];
    let world = &s[7..];

    println!("Hello, world! [..5]: {}", hello);
    println!("World, world! [7..]: {}", world);
}

