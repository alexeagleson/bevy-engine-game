#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum CreatureType {
    Human,
    Goblin,
    Orc,
}

pub struct CombatStats {
    pub armour_class: i32,
    pub attack_bonus: i32,
}

impl CreatureType {
    pub fn get_stats(&self) -> &CombatStats {
        match self {
            CreatureType::Human => &CombatStats {
                armour_class: 10,
                attack_bonus: 0,
            },
            CreatureType::Goblin => &CombatStats {
                armour_class: 10,
                attack_bonus: 0,
            },
            CreatureType::Orc => &CombatStats {
                armour_class: 18,
                attack_bonus: 0,
            },
        }
    }
}
