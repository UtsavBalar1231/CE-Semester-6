fn main() {
    let owner = String::from("Utsav Balar");
    let new_owner = give_me_ownership(owner);

    // println!("Owner: {}", owner); <- Error: owner has been moved
    println!("{}", new_owner);

    let owner = String::from("Utsav Balar");
    let new_owner = give_me_ownership_without_move(&owner); // <- Pass owner by reference

    println!("{}", owner); // <- Works fine
    println!("{}", new_owner);

    let name = &owner; // <- borrow owner
    println!("{:#?}", split_name(&name)); // <- pass name by reference
}

fn split_name(name: &str) -> (&str, &str) {
    let bytes = name.as_bytes();
    let mut idx = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            idx = i;
        }
    }
    let first_name = &name[..idx];
    let last_name = &name[idx + 1..];

    (first_name, last_name)
}

fn give_me_ownership_without_move(owner: &String) -> String {
    owner.to_string()
}

fn give_me_ownership(owner: String) -> String {
    owner
}
