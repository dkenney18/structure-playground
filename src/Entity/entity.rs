use rand::Rng;
use rnglib::{Language, RNG};
use std::fmt;
use crate::backpack::backpack::{Backpack, Slot, Item};

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub age: i32,
    pub health: i128,
    pub damage: i128,
    pub base_damage: i128,
    pub level: i128,
    pub xp_drop: i128,
    pub xp_stored: i128,
    pub level_up_multiplier: f64,
    pub xp_threshold: i128,
    pub multiplier: f64,
    pub backpack: Backpack,
}

impl Entity {
    pub fn new() -> Self {
        let rng = RNG::try_from(&Language::Elven).unwrap();
        let mut rnd_num = rand::thread_rng();
        let damage = rnd_num.gen_range(1..10).into();
        Self {
            name: rng.generate_name(),
            age: rnd_num.gen_range(18..100),
            health: rnd_num.gen_range(10..100).into(),
            damage: damage,
            base_damage: damage,
            level: 1,
            xp_drop: rnd_num.gen_range(1..50).into(),
            xp_stored: 0,
            level_up_multiplier: 1.01,
            xp_threshold: 100,
            multiplier: 1.1,
            backpack: Backpack::new(),
        }
    }

    pub fn get_name(&mut self) -> &str {
        &self.name
    }

    pub fn get_health(&self) -> i128 {
        self.health
    }

    pub fn get_damage(&self) -> i128 {
        self.damage
    }

    pub fn get_xp_stored(&self) -> i128 {
        self.xp_stored
    }

    pub fn add_xp(&mut self, xp: i128) {
        self.xp_stored += xp;
    }

    pub fn get_xp_drop(&self) -> i128 {
        self.xp_drop
    }

    pub fn recalculate_damage(&mut self) {
        // Calculate total damage boost from equipped items
        let total_item_damage: i128 = self.backpack.calculate_total_damage();

        // Update the current damage with the total item damage boost
        self.damage = self.base_damage + total_item_damage;
    
        // Print the new damage calculation
        println!("Your new damage is Base Damage {} + Item Boost {} = {}", self.base_damage, total_item_damage, self.damage);
    }

    pub fn level_up(&mut self) {
        let levels = self.xp_stored / self.xp_threshold;
        if levels > 0 {
            self.level += levels;
            self.health = (self.health as f64 * self.multiplier).floor() as i128 + levels;
            self.xp_drop = (self.xp_drop as f64 * self.multiplier).floor() as i128 + levels;
            self.damage = (self.damage as f64 * self.multiplier).floor() as i128 + levels;
            self.xp_stored %= self.xp_threshold;
            self.level_up_multiplier *= 1.0 + (self.level / 100) as f64;
            self.xp_threshold = (self.xp_threshold as f64 * self.level_up_multiplier).ceil() as i128;

            println!(
                "You have gained {} levels\n\
                 Your new level is {}\n\
                 Your new damage is {}\n\
                 Your new level multiplier is {}\n\
                 Your new level threshold is {}\n\
                 Your new health is {}\n\
                 Your new xp_drop is {}",
                levels, self.level, self.damage, self.level_up_multiplier, self.xp_threshold, self.health, self.xp_drop
            );
        }
    }

    pub fn equip_item(&mut self, item_name: &str) -> Result<(), String> {
        return self.backpack.equip_item(item_name);
    }

    pub fn unequip_item(&mut self, slot: &Slot) -> Option<Item> {
        self.backpack.unequip_item(slot)
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}, Age: {}, Health: {}, Damage: {}, Level: {}, XP Stored: {}, XP Dropped: {}",
            self.name, self.age, self.health, self.damage, self.level, self.xp_stored, self.xp_drop
        )
    }
}
