pub fn print_boolean() {
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("Boolean T: {}", t);
    println!("Boolean F: {}", f);
    println!("T or F: {}", t || f);
}