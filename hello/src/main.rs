fn main(){
    let a = 10;
    let b = 20;
    let c = a + b;

    println!("The sum of {} and {} is {}", a, b, c);

    // numbers
    let unsigned: u8 = 255;
    let signed: i8 = -128;
    let float: f32 = 3.14;
    let double: f64 = 3.14;
    
    println!("Unsigned: {}", unsigned);
    println!("Signed: {}", signed);
    println!("Float: {}", float);
    println!("Double: {}", double);

    // define biggest possible integer 
    let a: u128 = 340282366920938463463374607431768211455;

    println!("Biggest possible integer: {}", a);

    // chars
    let c = 'c';
    let heart_eyed_cat = '😻';

    println!("Char: {}", c);
    println!("Heart eyed cat: {}", heart_eyed_cat);

    // booleans
    let t = true;

    println!("Boolean: {}", t);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of tup is: {:?}", tup);
    println!("The value of tup is: ({}, {}, {})", tup.0, tup.1, tup.2);
    println!("The value of tup is: ({}, {}, {})", x, y, z);

    // define 2 floats explicitly with types and divide them then print
    let a: f32 = 10.0;
    let b: f32 = 3.0;
    let c = a / b;

    println!("The result of {} / {} is {}", a, b, c);

    // arrays
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    println!("Array a: {:?}", a);
    println!("Array b: {:?}", b);
    println!("Array c: {:?}", c);

    // access array elements
    let first = a[0];
    let second = a[1];

    println!("First element: {}", first);
    println!("Second element: {}", second);

    // string
    // let s = "Hello, world!";
    let s = String::from("Hello, world!");

    println!("String: {}", s);

    // string slices
    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    let world = &s[7..12];
    
    println!("Hello: {}", hello);
    println!("World: {}", world);

    // string slices with range syntax
    let hello = &s[..5];
    let world = &s[7..];

    println!("Hello: {}", hello);
    println!("World: {}", world);
}