pub fn print_integers () -> () {
    let unsigned: u8 = 255;
    let signed: i8 = -128;

    println!("Unsigned: {}", unsigned);
    println!("Signed: {}", signed);

    // define biggest possible integer
    let a: u128 = 340282366920938463463374607431768211455;

    println!("Biggest possible integer: {}", a);
}

pub fn print_floats () -> () {
    let float: f32 = 3.14;
    let double: f64 = 3.14;

    println!("Float: {}", float);
    println!("Double: {}", double);
}

pub fn print_summation () -> () {
    let a: f32 = 10.0;
    let b: f32 = 3.0;
    let c = a + b;

    println!("The result of {} + {} is {}", a, b, c);
}