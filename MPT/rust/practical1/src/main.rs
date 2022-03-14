use std::io;

fn main() {
    let mut num = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut num).unwrap();
    let num: u32 = num.trim().parse().unwrap();

    match is_prime(num) {
        true => {
            println!("{} is prime", num);
        }
        false => {
            println!("{} is not prime", num);
        }
    }
}

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let mut counter = 0;
    for y in 1..num + 1 {
        if num % y == 0 {
            counter += 1;
            //println!("{} is divisible by {}", num, y);
        }
    }
    //println!("{}", counter);
    if counter == 2 {
        true
    } else {
        false
    }
}
