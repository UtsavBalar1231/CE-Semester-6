fn main() {
    let name = String::from("Utsav Balar");

    let bytes = name.as_bytes();
    let mut idx = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            idx = i;
        }
    }
    let first_name = &name[..idx];
    let last_name = &name[idx + 1..];

    println!("First Name: {} \nLast Name: {}", first_name, last_name);

    println!("Full name: {}", combine_name(&first_name, &last_name));
}

fn combine_name(first_name: &str, last_name: &str) -> String {
    let mut full_name = String::new();
    full_name.push_str(first_name);
    full_name.push_str(" ");
    full_name.push_str(last_name);
    full_name
}
