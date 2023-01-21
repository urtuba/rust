pub fn print_tuple() {
    // Tuples are fixed in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup(x,y,z) is: {:?}", tup);

    // Destructuring
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // Accessing tuple elements by index
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);
}