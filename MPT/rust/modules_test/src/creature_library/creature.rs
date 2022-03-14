// crate to generate random creature
use rand_derive2::RandGen;
// crate that implements method to get type of creature
use crate::get_type_creature::TypeToStr;

#[derive(RandGen)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Body {
    pub eyes: u8,
    pub nose: u8,
    pub hands: u8,
    pub legs: u8,
    pub mouth: u8,
}

#[derive(RandGen)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum Creature {
    Dog(Body),
    Cat(Body),
    Human(Body),
    MythicalBeast(Body),
    Donkey(Body),
    Monkey(Body),
    Snake(Body),
    Dragon(Body),
    Fish(Body),
    Bird(Body),
    Lizard(Body),
    Frog(Body),
    Bat(Body),
    Bear(Body),
    Elephant(Body),
    Rhino(Body),
    Giraffe(Body),
}

impl Creature {
    pub fn check_creature(&self) -> Option<bool> {
        match self {
            Creature::Dog(_)
            | Creature::Cat(_)
            | Creature::Human(_)
            | Creature::MythicalBeast(_)
            | Creature::Donkey(_)
            | Creature::Monkey(_)
            | Creature::Snake(_)
            | Creature::Dragon(_)
            | Creature::Fish(_)
            | Creature::Bird(_)
            | Creature::Lizard(_)
            | Creature::Frog(_)
            | Creature::Bat(_)
            | Creature::Bear(_)
            | Creature::Elephant(_)
            | Creature::Rhino(_)
            | Creature::Giraffe(_) => Some(true),
        }
    }

    pub fn new_creature(&self) -> &Self {
        self
    }

    pub fn copy_creature(&self) -> &Self {
        Self::new_creature(&self)
    }

    pub fn generate_random_creature() -> Self {
        Self::generate_random()
    }

    pub fn creature_exists(&self) {
        match self.check_creature() {
            Some(thing) => {
                if thing == true {
                    println!("{} exists!", self.get_type_creature())
                }
            }
            _ => {
                println!("{} Doesn't exist", self.get_type_creature())
            }
        }
    }
}
