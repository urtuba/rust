mod datatypes;
mod utils;

fn main(){
    // numbers
    datatypes::number::print_integers();
    datatypes::number::print_floats();
    datatypes::number::print_summation();
    println!("The result of {} / {} is {}", 10.0, 3.0, 10.0/3.0);
    utils::print_blank_line();

    // booleans
    datatypes::boolean::print_boolean();
    utils::print_blank_line();

    // characters
    datatypes::string::print_string();
    datatypes::string::print_string_slices();
    datatypes::string::print_string_ranges();
    utils::print_blank_line();

    // tuples
    datatypes::tuple::print_tuple();
    utils::print_blank_line();

    // arrays
    datatypes::array::print_array();
    utils::print_blank_line();

    // function calls
    let a = 10;
    let b = 11;

    println!("{} is even: {}", a, utils::number::is_even(a));
    println!("{} is odd: {}", a, utils::number::is_odd(a));
    println!("{} is even: {}", b, utils::number::is_even(b));
    println!("{} is odd: {}", b, utils::number::is_odd(b));
}