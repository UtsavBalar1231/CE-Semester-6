mod roman_symbols;

use roman_symbols::RomanSymbols;

fn main() {
    use std::io;
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().parse().expect("Failed to parse input");

    let mut num = String::new();
    match RomanSymbols::int_to_roman(input) {
        Some(value) => {
            num = value;
            println!("{}", &num)
        }
        None => println!("Invalid value"),
    }
    match RomanSymbols::roman_to_int(&num) {
        Some(value) => println!("{}", value),
        None => println!("Invalid value"),
    }
}
