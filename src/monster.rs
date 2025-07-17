extern crate rand;

// use rand::Rng;
use colored::Colorize;
// use std::time::*;
// use std::thread;

// let mut rng = rand::thread_rng();

pub struct Monster {
    pub name: String,
    pub health: u32,
    pub damage: u32,
    pub defense: u32,
}


impl Monster {
    pub fn new(name: String, health: u32, damage: u32, defense: u32) -> Self {
        Monster {
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


/* 
*/