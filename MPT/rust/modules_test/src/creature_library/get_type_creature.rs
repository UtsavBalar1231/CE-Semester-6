use crate::creature::Creature;

pub trait TypeToStr {
    fn get_type_creature(&self) -> &'static str;
}

impl TypeToStr for Creature {
    fn get_type_creature(&self) -> &'static str {
        match self {
            Creature::Dog(_) => "Dog",
            Creature::Cat(_) => "Cat",
            Creature::Human(_) => "Human",
            Creature::MythicalBeast(_) => "MythicalBeast",
            Creature::Donkey(_) => "Donkey",
            Creature::Monkey(_) => "Monkey",
            Creature::Snake(_) => "Snake",
            Creature::Dragon(_) => "Dragon",
            Creature::Fish(_) => "Fish",
            Creature::Bird(_) => "Bird",
            Creature::Lizard(_) => "Lizard",
            Creature::Frog(_) => "Frog",
            Creature::Bat(_) => "Bat",
            Creature::Bear(_) => "Bear",
            Creature::Elephant(_) => "Elephant",
            Creature::Rhino(_) => "Rhino",
            Creature::Giraffe(_) => "Giraffe",
        }
    }
}