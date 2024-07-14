use std::fmt;
use crate::entity::Entity;

#[derive(Debug)]
pub struct Battle<'a> {
    attacker: &'a mut Entity,
    defender: &'a mut Entity,
}

impl<'a> Battle<'a> {
    pub fn new(attacker: &'a mut Entity, defender: &'a mut Entity) -> Self {
        Self { attacker, defender }
    }

    pub fn attack(&mut self) {
        let rounds = self.get_max_rounds();
        let attacker_damage = rounds.max_attacker_rounds * self.attacker.get_damage();
        let defender_damage = rounds.max_defender_rounds * self.defender.get_damage();

        //println!("{}", rounds);
        //println!("Attacker Total Damage: {}", attacker_damage);
        //println!("Defender Total Damage: {}", defender_damage);

        println!(
            "{} attacks {}",
            self.attacker.get_name(),
            self.defender.get_name()
        );

        if attacker_damage > self.defender.get_health() && attacker_damage > defender_damage {
            println!("{} wins", self.attacker.get_name());
            self.attacker.add_xp(self.defender.get_xp_drop());
            // Transfer items from defender's backpack to attacker's backpack
            self.transfer_items();
        } else if defender_damage > self.attacker.get_health() && defender_damage > attacker_damage {
            println!("{} wins", self.defender.get_name());
        } else {
            println!(
                "Attack was unsuccessful between {} and {}",
                self.attacker.get_name(),
                self.defender.get_name()
            );
        }
    }

    fn transfer_items(&mut self) {
        let defender = &mut self.defender;
        let attacker = &mut self.attacker;
    
        // Transfer all items from defender's backpack to attacker's backpack
    
    let items = defender.backpack.remove_items();
       for item in items {
        attacker.backpack.add_item(item.unwrap()); // Unwrap the Option to get the actual item
       }
    
        //println!("Items transferred to {}'s backpack:", attacker.get_name());
        //println!("{}", attacker_backpack); // Display attacker's updated backpack
    }

    fn get_max_rounds(&self) -> Round {
        let max_attacker_rounds = self.divide(self.attacker.get_health(), self.defender.get_damage()).unwrap_or(0);
        let max_defender_rounds = self.divide(self.defender.get_health(), self.attacker.get_damage()).unwrap_or(0);

        Round {
            max_attacker_rounds,
            max_defender_rounds,
        }
    }

    fn divide(&self, x: i128, y: i128) -> Result<i128, &'static str> {
        if y != 0 {
            Ok(x / y)
        } else {
            Err("Division by zero")
        }
    }
}

impl<'a> fmt::Display for Battle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Battle between {} and {}", self.attacker.name, self.defender.name)
    }
}

pub struct Round {
    max_attacker_rounds: i128,
    max_defender_rounds: i128,
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Attacker Max Rounds: {}, Defender Max Rounds: {}",
            self.max_attacker_rounds, self.max_defender_rounds
        )
    }
}
