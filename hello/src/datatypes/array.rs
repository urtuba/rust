pub fn print_array() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    println!("Array a: {:?}", a);
    println!("Length of array a: {}", a.len());
    println!("Array b: {:?}", b);
    println!("Length of array b: {}", b.len());
    println!("Array c: {:?}", c);
    println!("Length of array c: {}", c.len());

    // access array elements
    let first = a[0];
    let second = a[1];
    
    println!("First element(a): {}", first);
    println!("Second element(a): {}", second);
}