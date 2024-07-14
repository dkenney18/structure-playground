extern crate rand;


use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd)]
pub enum Slot {
    Weapon,
    Ring,
    Helmet,
    Chestplate,
    Gauntlets,
    Greaves,
    Boots,
    Shield,
    Pauldrons,
    Bracers,
    Cloak,
    Belt,
    Amulet,
    Potion,
}

impl Ord for Slot {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let slot_str = match self {
            Slot::Weapon => "Weapon",
            Slot::Ring => "Ring",
            Slot::Boots => "Boots",
            Slot::Potion => "Potion",
            Slot::Helmet => "Helmet",
            Slot::Chestplate => "Chestplate",
            Slot::Gauntlets => "Gauntlets",
            Slot::Greaves => "Greaves",
            Slot::Shield => "Shield",
            Slot::Pauldrons => "Pauldrons",
            Slot::Bracers => "Bracers",
            Slot::Cloak => "Cloak",
            Slot::Belt =>  "Belt",
            Slot::Amulet => "Amulet",
        };
        write!(f, "{}", slot_str)
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub damage_boost: i128,
    pub slot: Option<Slot>,
}
impl Item {
    pub fn new_random() -> Self {
        let name = generate_random_item_name();
        let (description, damage_boost) = generate_random_item_description();
        let (_, suffix) = parse_item_name(&name);
        let slot = determine_item_slot(&suffix); // Assign slot based on suffix
        Item {
            name: name.clone(),
            description,
            damage_boost,
            slot,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} (+{} Damage), Slot: {:?}",
            self.name, self.description, self.damage_boost, self.slot
        )
    }
}

fn generate_random_item_name() -> String {
    let prefixes = [
        "Ancient",
        "Mystical",
        "Legendary",
        "Powerful",
        "Cursed",
        "Divine",
        "Epic",
        "Strange",
        "Exquisite",
        "Enchanted",
    ];
    let suffixes = [
        "Sword",
        "Staff",
        "Ring",
        "Amulet",
        "Potion",
        "Scroll",
        "Helmet",
        "Chestplate",
        "Gauntlets",
        "Greaves",
        "Boots",
        "Shield",
        "Pauldrons",
        "Bracers",
        "Cloak",
        "Belt",
        "Amulet",
    ];

    let mut rng = rand::thread_rng();
    let prefix_index = rng.gen_range(0..prefixes.len());
    let suffix_index = rng.gen_range(0..suffixes.len());

    format!("{} {}", prefixes[prefix_index], suffixes[suffix_index])
}

fn generate_random_item_description() -> (String, i128) {
    let descriptions = [
        ("A powerful artifact from ancient times.", 5),
        ("An enchanted item with mysterious properties.", 3),
        ("A legendary piece of equipment.", 8),
        ("A rare and valuable treasure.", 6),
        ("A cursed item that brings both power and danger.", 10),
        ("A divine relic imbued with extraordinary abilities.", 12),
        ("An epic gear forged by legendary craftsmen.", 7),
        ("A strange item with unpredictable effects.", 4),
        ("An exquisite piece of equipment adorned with gems.", 6),
        ("An enchanted scroll with arcane writings.", 2),
    ];

    let mut rng = rand::thread_rng();
    let description_index = rng.gen_range(0..descriptions.len());

    let (description, damage_boost) = descriptions[description_index];

    (description.to_owned(), damage_boost)
}

fn parse_item_name(name: &str) -> (&str, String) {
    let parts: Vec<&str> = name.split_whitespace().collect();
    if let [first, last] = parts.as_slice() {
        (*first, last.to_string())
    } else {
        ("", "".to_string())
    }
}

fn determine_item_slot(suffix: &str) -> Option<Slot> {
    match suffix {
        "Sword" | "Staff" => Some(Slot::Weapon),
        "Helmet" => Some(Slot::Helmet),
        "Chestplate" => Some(Slot::Chestplate),
        "Gauntlets" => Some(Slot::Gauntlets),
        "Greaves" => Some(Slot::Greaves),
        "Boots" => Some(Slot::Boots),
        "Shield" => Some(Slot::Shield),
        "Pauldrons" => Some(Slot::Pauldrons),
        "Bracers" => Some(Slot::Bracers),
        "Cloak" => Some(Slot::Cloak),
        "Belt" => Some(Slot::Belt),
        "Amulet" => Some(Slot::Amulet),
        "Ring" => Some(Slot::Ring),
        "Potion" | "Scroll" => Some(Slot::Potion),
        _ => None,
    }
}


    #[derive(Debug, Clone)]
pub struct Backpack {
    pub items: HashMap<String, Item>,
    pub equipped_items: HashMap<Slot, Item>,
}

impl Backpack {
    pub fn new() -> Self {
        Backpack {
            items: HashMap::new(),
            equipped_items: HashMap::new(),
        }
    }
    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.name.clone(), item);
    }

    pub fn remove_items(&mut self) -> Vec<Option<Item>> {
        let items: Vec<Option<Item>> = self.items.drain().map(|(_, v)| Some(v.clone())).collect();
        items
    }

   pub fn list_items(&self) {
    if self.items.is_empty() {
        println!("No items in the Inventory.");
    } else {
        // Collect inventory items and sort by slot, then by damage boost
        let mut inventory_items: Vec<&Item> = self.items.values().collect();
        inventory_items.sort_by_key(|item| (item.slot.clone(), item.damage_boost));

        // Print inventory items grouped by slot
        println!("{} items in the Inventory:", self.items.len());
        self.pretty_print_items(&inventory_items);
    }

    if self.equipped_items.is_empty() {
        println!("No items equipped.");
    } else {
        // Collect equipped items and sort by slot, then by damage boost
        let mut equipped_items: Vec<&Item> = self.equipped_items.values().collect();
        equipped_items.sort_by_key(|item| (item.slot.clone(), item.damage_boost));

        // Print equipped items grouped by slot
        println!("{} items equipped:", self.equipped_items.len());
        self.pretty_print_items(&equipped_items);
    }

    println!("\n");
}

fn pretty_print_items(&self, items: &[&Item]) {
    let mut current_slot: Option<&Slot> = None;

    for item in items {
        if current_slot.is_none() || current_slot != item.slot.as_ref() {
            current_slot = item.slot.as_ref();
            println!("Slot: {:?}", item.slot.unwrap());
        }
        println!("    {}", item);
    }
}

    pub fn calculate_total_damage(&self) -> i128 {
        self.equipped_items
            .values()
            .map(|item| item.damage_boost)
            .sum()
    }

    pub fn equip_item(&mut self, item_name: &str) -> Result<(), String> {
        println!("Equipping item: {}", item_name);
        if let Some(item) = self.items.remove(item_name) {
            if let Some(slot) = item.slot {
                // Check if there's already an item in the slot
                if let Some(prev_item) = self.equipped_items.insert(slot.clone(), item.clone()) {
                    // Put back the previously equipped item into the items map
                    self.items.insert(prev_item.name.clone(), prev_item.clone());
                }
                // Equip the new item
                self.equipped_items.insert(slot, item.clone());
                println!("Equipped item: {}", item.name);
                Ok(())
            } else {
                self.items.insert(item.name.clone(), item.clone());
                Err("Item cannot be equipped because it doesn't have a valid slot.".to_string())
            }
        } else {
            Err("Item not found in the backpack.".to_string())
        }
    }

    pub fn unequip_item(&mut self, slot: &Slot) -> Option<Item> {
        if let Some(item) = self.equipped_items.remove(slot) {
            self.items.insert(item.name.clone(), item.clone());
            Some(item)
        } else {
            None
        }
    }

    pub fn generate_item(&self) -> Item {
        Item::new_random()
    }
}

