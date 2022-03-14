#[allow(unused_imports)]
use creature_library::creature::{Creature, Body};
use creature_library::get_type_creature::TypeToStr;

fn main() {
    // let creature1 = Creature::Dog(Body {
    //     eyes: 2,
    //     nose: 1,
    //     hands: 0,
    //     legs: 4,
    //     mouth: 1,
    // });

    // let creature2 = Creature::Cat(Body {
    //     eyes: 2,
    //     nose: 1,
    //     hands: 0,
    //     legs: 4,
    //     mouth: 1,
    // });

    // println!("Creature 1 => {}", creature1.get_type_creature());
    // println!("Creature 2 => {}", creature2.get_type_creature());

    // creature1.creature_exists();
    // creature2.creature_exists();

    // let creature3 = Creature::new_creature(&Creature::Human(Body {
    //     hands: 2,
    //     legs: 2,
    //     mouth: 1,
    //     nose: 1,
    //     eyes: 2,
    // }));

    // println!("Creature 3 => {}", creature3.get_type_creature());

    // let creature4 = Creature::copy_creature(&creature3);
    // println!("Creature 4 => {}", creature4.get_type_creature());

    // creature3.creature_exists();
    // creature4.creature_exists();

    // let creature5 = Creature::new_creature(&Creature::MythicalBeast(Body {
    //     hands: 8,
    //     legs: 8,
    //     mouth: 4,
    //     nose: 4,
    //     eyes: 8,
    // }));

    // creature5.creature_exists();

    use std::io;
    let your_creature: Creature = Creature::generate_random_creature();
    let mut input = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{} is the {}", input.trim(), your_creature.get_type_creature());
}