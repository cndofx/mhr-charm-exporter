use crate::constants;
use std::fmt::{Display, Formatter};

pub struct CharmSkill {
    pub skill_id: u8,
    pub skill_level: u32,
}

pub struct EquipBoxMetadata {
    pub equipment_location: usize,
    pub equipment_count: u32,
}

pub struct Charm {
    pub primary_skill: CharmSkill,
    pub secondary_skill: CharmSkill,

    pub slot_levels: [u32; 3],
}

impl Display for Charm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut remaining_slots = self.slot_levels.clone();

        let slot_types: Vec<u32> = (1..=3)
            .map(|_| {
                if remaining_slots[2] > 0 {
                    remaining_slots[2] -= 1;
                    3
                } else if remaining_slots[1] > 0 {
                    remaining_slots[1] -= 1;
                    2
                } else if remaining_slots[0] > 0 {
                    remaining_slots[0] -= 1;
                    1
                } else {
                    0
                }
            })
            .collect();

        let primary_name = constants::SKILL_NAMES[self.primary_skill.skill_id as usize];
        let secondary_name = constants::SKILL_NAMES[self.secondary_skill.skill_id as usize];

        write!(
            f,
            "{},{},{},{},{},{},{}",
            primary_name,
            self.primary_skill.skill_level,
            secondary_name,
            self.secondary_skill.skill_level,
            slot_types[0],
            slot_types[1],
            slot_types[2]
        )
    }
}
