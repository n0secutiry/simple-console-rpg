extern crate rand;

mod monster;
mod player;

use monster::Monster;
use player::Player;
use rand::Rng;
use colored::Colorize;
use std::time::*;
use std::thread;
use std::io;


fn calculate_attack_result(attack_power: u32, current_health: u32, current_defense: u32) -> (u32, u32) {
    let new_health: u32;
    let new_defense: u32;

    if current_defense >= attack_power {
        new_health = current_health;
        new_defense = current_defense - attack_power;
        println!("{}", "The armor withstood the blow! The protection was damaged.".green());

    } else {
        new_defense = 0;
        let damage_to_health = attack_power - current_defense;
        new_health = current_health.saturating_sub(damage_to_health);
        println!("{}", "Armor pierced! Health damage inflicted.".red());
    }

    (new_health, new_defense)
}

fn first_attack(player: &Player, enemy: &Monster) -> i8 {
    let mut rng = rand::thread_rng();
    let rand_attack = rng.gen_range(0..=1);
    rand_attack as i8
}

fn main() {
    let username: String;

    thread::sleep(Duration::from_secs(2));
    println!("{}", "Welcome to the battle!\n".purple().magenta().bold());

    thread::sleep(Duration::from_secs(2));
    loop {
        println!("{}", "Enter your username".blue());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        let trimmed_input = input.trim().to_string();

        if trimmed_input.is_empty() {
            thread::sleep(Duration::from_secs(1));
            println!("{}", "Username cannot be empty!".red());
        } else {
            username = trimmed_input;
            thread::sleep(Duration::from_secs(1));
            let s = format!("Hello, {}!", username);
            println!("{}", s.purple().magenta().bold());
            break;
        }
    }

    let user_welome = format!("{}, do you want to play a battle?", username);
    loop {
        println!("{}", user_welome.blue().bold());
        println!("{}", "Enter 'yes' to start or 'no' to exit".blue());
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line!");
        let answer = answer.trim().to_lowercase();

        let valid_answers = ["no", "n", "exit", "quit", "yes", "y", "start", "play"];


        if !valid_answers.contains(&answer.as_str()) {
            thread::sleep(Duration::from_secs(1));
            println!("{}", "Invalid input, please try again!".red().bold());
        }  else if (answer == "no") || (answer == "n") || (answer == "exit") || (answer == "quit") {
            println!("{}", "Okay. As you say..".blue());
            thread::sleep(Duration::from_secs(3));
            println!();
            println!();
            // println!("{}", "Thanks for coming. Good luck!".purple().magenta().bold());
            break;
        } else {
            println!("{}", "Great! Let's get started then = )".blue());
            thread::sleep(Duration::from_secs(1));
            println!("{}", "Choose which hero you want to play: (Enter num hero)".blue());
        
            println!();
            println!();
            thread::sleep(Duration::from_secs(1));
            let mut rng = rand::thread_rng();
            let player_dragon = Player::new(
                "Dragon".to_string(),
                rng.gen_range(50..=150),
                rng.gen_range(25..=80),
                rng.gen_range(5..=25),
            );
            let player_vurdalaka = Player::new(
                "Vurdalaka".to_string(),
                rng.gen_range(50..=150),
                rng.gen_range(25..=80),
                rng.gen_range(5..=25),
            );
            let player_robber = Player::new(
                    "Robber".to_string(),
                rng.gen_range(50..=150),
                rng.gen_range(25..=80),
                rng.gen_range(5..=25),
            );

            player_dragon.print_info();
            thread::sleep(Duration::from_secs(1));
            player_vurdalaka.print_info();
            thread::sleep(Duration::from_secs(1));
            player_robber.print_info();
            thread::sleep(Duration::from_secs(1));
                
            println!();
            // println!("{}", "Please, enter the ID of the hero you want to play:".blue());

            let mut choose_player;
            loop {
                let valid_heroes = ["1", "2", "3"];
                let mut hero_id = String::new();
                io::stdin().read_line(&mut hero_id).expect("Failed to read line!");
                let hero_id = hero_id.trim().to_lowercase();
                if !valid_heroes.contains(&hero_id.as_str()) {
                    // thread::sleep(Duration::from_secs(1));
                    println!("{}", "Invalid hero ID, please try again!".red().bold());
                } else {
                    if hero_id == "1" {
                        println!("{}", "You have chosen the Dragon!".blue());
                        choose_player = player_dragon;
                    } else if hero_id == "2" {
                        println!("{}", "You have chosen the Vurdalaka!".blue());
                        choose_player = player_vurdalaka;
                    } else {
                        println!("{}", "You have chosen the Robber!".blue());
                        choose_player = player_robber;
                    }
                    thread::sleep(Duration::from_secs(1));
                    break;
                }
            }

            println!();
            println!("{}", "The battle begins!".green().bold());
            thread::sleep(Duration::from_secs(1));

            println!("{}", "your opponent's name is Bowl...".blue());
            let mut rng = rand::thread_rng();
            let mut enemy = Monster::new(
                "Bowl".to_string(),
                rng.gen_range(50..=150),
                rng.gen_range(25..=80),
                rng.gen_range(5..=25),
            );
            enemy.print_info();
            thread::sleep(Duration::from_secs(1));


            println!("{}", "The game starts in..".blue());
            for i in 1..6 {
                println!(" {} ", i);
                thread::sleep(Duration::from_secs(1));
            }
            println!("{}", "START!".green().bold());
            println!();
            println!();

            let mut is_player_turt = first_attack(&choose_player, &enemy) == 0;

            loop {
                if enemy.health <= 0 {
                    println!("{}", "You have won the battle!".green().bold());
                    thread::sleep(Duration::from_secs(2));
                    break;
                }
                if choose_player.health <= 0 {
                    println!("{}", "You have lost the battle!".red().bold());
                    thread::sleep(Duration::from_secs(2));
                    break;
                }

                thread::sleep(Duration::from_secs(1));


                if is_player_turt {
                    println!("{}", "The enemy attacks!".truecolor(42, 129, 154));
                    let (update_hp, update_def) = calculate_attack_result(
                        enemy.damage,
                        choose_player.health,
                        choose_player.defense,
                    );

                    choose_player.health = update_hp;
                    choose_player.defense = update_def;

                    println!("Your health: {}, Your defense: {}", choose_player.health, choose_player.defense);
                    thread::sleep(Duration::from_secs(2));
                                            
                } else {
                    println!("{}", "You attack!".truecolor(42, 129, 154));
                    let (update_hp, update_def) = calculate_attack_result(
                        choose_player.damage,
                        enemy.health,
                        enemy.defense,
                    );
                    
                    enemy.health = update_hp;
                    enemy.defense = update_def;

                    println!("Enemy health: {}, Enemy defense: {}", enemy.health, enemy.defense);
                    thread::sleep(Duration::from_secs(2));
                }
                is_player_turt = !is_player_turt;
                println!("--- --- ---");
                println!();
            }
            thread::sleep(Duration::from_secs(1));

            }

    }
    println!();
    println!("{}", "Thank you for visiting us. Good luck!".purple().magenta().bold());
    thread::sleep(Duration::from_secs(5));
    println!();
    println!();
    println!();
    println!("To exit, press Enter...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
}
