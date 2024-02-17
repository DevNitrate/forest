use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct User {
    pseudo: String,
    miner: u32,
    farmer: u32,
    hunter: u32,
    alchemist: u32,
}

impl User {
    fn add(self: &Self) {
        let json = serde_json::to_string(self).expect("Failed to parse json");
        let mut path: String = String::from("./Users/");
        path.push_str(&self.pseudo);
        path.push_str(".json");
        fs::write(path, json).expect("Failed to save data");
    }
}

fn get(pseudo: String) {
    let mut path: String = String::from("./Users/");
    path.push_str(&pseudo);
    path.push_str(".json");

    if Path::new(path.as_str()).exists() {
        let content: String = fs::read_to_string(path).expect("Could not read file");
        let file: Value = serde_json::from_str(&content.as_str()).expect("Failed to parse json");
        println!("pseudo: {}\nmineur: {}\nfarmeur: {}\nhunter: {}\nalchimiste: {}", file["pseudo"].as_str().expect("Is not a string"), file["miner"], file["farmer"], file["hunter"], file["alchemist"]);
    } else {
        println!("Ce membre n'existe pas");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("Vous n'avez pas assez d'argument faites 'forest help' pour voir les commandes");
    } else if args[1] == "new" {
        if args.len() < 7 || args.len() > 7 {
            if args.len() == 3 {
                if args[2] == "help" {
                    println!("Syntaxe de la commande: 'forest new <pseudo> <miner> <farmer> <hunter> <alchimiste>'");
                }
            } else {
                println!("Votre commande n'a pas le bon nombre d'arguments faites 'forest new help' pour plus");
            }
        } else {
            let pseudo: String = String::from(&args[2]);
            let miner: u32 = args[3].parse::<u32>().expect("Failed to parse a number");
            let farmer: u32 = args[4].parse::<u32>().expect("Failed to parse a number");
            let hunter: u32 = args[5].parse::<u32>().expect("Failed to parse a number");
            let alchemist: u32 = args[6].parse::<u32>().expect("Failed to parse a number");

            let user: User = User {
                pseudo,
                miner,
                farmer,
                hunter,
                alchemist,
            };
            user.add();
        }
    } else if args[1] == "update" {
        if args.len() < 7 || args.len() > 7 {
            if args.len() == 3 {
                if args[2] == "help" {
                    println!("Syntaxe de la commande: 'forest update <pseudo> <miner> <farmer> <hunter> <alchimiste>'");
                }
            } else {
                println!("Votre commande n'a pas le bon nombre d'arguments faites 'forest new help' pour plus");
            }
        } else {
            let pseudo: String = String::from(&args[2]);
            let miner: u32 = args[3].parse::<u32>().expect("Failed to parse a number");
            let farmer: u32 = args[4].parse::<u32>().expect("Failed to parse a number");
            let hunter: u32 = args[5].parse::<u32>().expect("Failed to parse a number");
            let alchemist: u32 = args[6].parse::<u32>().expect("Failed to parse a number");

            let user: User = User {
                pseudo,
                miner,
                farmer,
                hunter,
                alchemist,
            };
            user.add();
        }
    } else if args[1] == "get" {
        if args.len() < 3 || args.len() > 3 {
            println!("Votre commande n'a pas le bon nombre d'arguments faites 'forest new help' pour plus");
        } else if args[2] == "help" {
            println!("Syntaxe de la commande: 'forest get <pseudo>'");
        } else {
            let pseudo: String = String::from(&args[2]);
            get(pseudo);
        }
    }
}