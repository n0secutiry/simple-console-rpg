extern crate rand;

use colored::Colorize;

pub struct Player {
    pub name: String,
    pub health: u32,
    pub damage: u32,
    pub defense: u32,
}


impl Player {
    pub fn new(name: String, health: u32, damage: u32, defense: u32) -> Self {
        Player {
            name,
            health,
            damage,
            defense,
        }
    }
    pub fn print_info(&self) {
        let print_info = format!(
            "Name = {}, Health = {}, Damage = {}, Defense = {}",
            self.name, self.health, self.damage, self.defense
        );
        println!("{}", print_info.yellow().bold());
    }
}

