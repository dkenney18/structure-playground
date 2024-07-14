use std::io::{stdout, Write};
use termion::clear;

use backpack::backpack::Slot;
// main.rs
use text_io::read;
use Battle::battle;
use Entity::entity;
mod backpack;
mod Battle;
mod Entity;


fn main() {
    let mut is_first_time: bool = true;
    let mut attacker: entity::Entity = entity::Entity::new();
    let mut defender: entity::Entity;

    loop {
        if is_first_time {
            println!("Welcome to Dungeon Fighter. Let's get your player set up for the game");
            println!("{:#?}", attacker);
            is_first_time = false;
        }

        let ans = read_input();

        match ans.as_str() {
            "A" | "a" => {
                defender = entity::Entity::new();
                defender
                    .backpack
                    .add_item(defender.backpack.generate_item());
                println!("{}", attacker);
                println!("{}", defender);
                let mut battle = battle::Battle::new(&mut attacker, &mut defender);
                battle.attack();
            }
            "L" | "l" => {
                attacker.level_up();
            }
            "V" | "v" => {
                println!("{}", attacker);
                attacker.backpack.list_items();
            }
            "I" | "i" => {
                // View items and equip/unequip
                handle_inventory(&mut attacker);
            }
            "Q" | "q" => break,
            _ => {
                println!("Invalid Input");
            }
        }
    }
}

fn handle_inventory(entity: &mut entity::Entity) {
    clear_screen();
    println!("Inventory Items:");
    for (count, (item_name, item)) in entity.backpack.items.iter().enumerate() {
        println!(
            "{}: {} gives +{} damage  Slot: {:?}",
            count, item_name, item.damage_boost, item.slot
        );
    }

    println!("Equipped Items:");
    for (count, (item_name, item)) in entity.backpack.equipped_items.iter().enumerate() {
        println!(
            "{}: {} gives +{} damage  Slot: {:?}",
            count, item_name, item.damage_boost, item.slot
        );
    }

    println!("Enter A:item_number to equip (add) item or R:item_number to unequip item: ");
    let ans: String = read!();
    let parts: Vec<&str> = ans.split(':').collect();

    if parts.len() != 2 {
        println!("Invalid input format.");
        return;
    }

    match parts[0] {
        "A" | "a" => {
            let item_num: Result<usize, _> = parts[1].parse();
            match item_num {
                Ok(num) => {
                    if num >= entity.backpack.items.len() {
                        println!("Invalid item number");
                        return;
                    }

                    let item_name = entity.backpack.items.keys().nth(num).unwrap();
                    if let Err(err) = entity.equip_item(item_name.clone().as_str()) {
                        println!("Failed to equip item: {}", err);
                    } else {
                        entity.recalculate_damage();
                    }
                }
                Err(_) => println!("Invalid item number"),
            }
        }
        "R" | "r" => {
            let item_num: Result<usize, _> = parts[1].parse();
            match item_num {
                Ok(num) => {
                    if num >= entity.backpack.items.len() {
                        println!("Invalid item number");
                        return;
                    }

                    let slot_str = entity.backpack.equipped_items.keys().nth(item_num.unwrap()).unwrap();
                    if let Ok(slot) = parse_slot(slot_str.to_string().as_str()) {
                        if let Some(item) = entity.unequip_item(&slot) {
                            println!("Unequipped item: {}", item.name);
                            entity.recalculate_damage();
                        } else {
                            println!("No item found in slot {:?}", slot);
                        }
                    } else {
                        println!("Invalid slot specified.");
                    }
                }
                Err(_) => println!("Invalid item number"),
        }
        }
        _ => println!("Invalid input format."),
    }
}

fn parse_slot(slot_str: &str) -> Result<Slot, ()> {
    match slot_str.trim().to_lowercase().as_str() {
        "weapon" => Ok(Slot::Weapon),
        "helmet" => Ok(Slot::Helmet),
        "chestplate" => Ok(Slot::Chestplate),
        "gauntlets" => Ok(Slot::Gauntlets),
        "greaves" => Ok(Slot::Greaves),
        "boots" => Ok(Slot::Boots),
        "shield" => Ok(Slot::Shield),
        "pauldrons" => Ok(Slot::Pauldrons),
        "bracers" => Ok(Slot::Bracers),
        "cloak" => Ok(Slot::Cloak),
        "belt" => Ok(Slot::Belt),
        "amulet" => Ok(Slot::Amulet),
        "ring" => Ok(Slot::Ring),
        "potion" | "scroll" => Ok(Slot::Potion),
        _ => Err(()),
    }
}

// Function to read user input
fn read_input() -> String {
    println!("Enter A to Attack\nEnter Q to Quit\nEnter L to level up\nEnter V to view inventory\nEnter I to manage inventory");
    let input: String = read!();
    input
}

fn clear_screen() {
    print!("{}", clear::All);
    stdout().flush().unwrap();
}
