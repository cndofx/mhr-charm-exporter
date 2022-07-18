use process_memory::{DataMember, Memory, ProcessHandle};

use crate::{constants, data};

pub struct DataManager {
    process: ProcessHandle,
    manager_location: usize,
}

impl DataManager {
    pub fn new(process: ProcessHandle) -> DataManager {
        let manager_location = process.read_value_at(constants::offsets::DATA_MANAGER);

        dbg!(manager_location);

        return DataManager {
            process,
            manager_location,
        };
    }

    pub fn get_all_charms(&self) -> Vec<data::Charm> {
        let box_ptr = self
            .process
            .read_value_at(self.manager_location + constants::offsets::EQUIPMENT_BOX);

        dbg!(box_ptr);

        let box_meta = self.get_box_metadata(box_ptr);

        //if someone is using this thing, it's probably safe to assume
        //they have too many charms. Probably around 50-75% inventory?
        let mut charms = Vec::with_capacity((box_meta.equipment_count / 2) as usize);

        for i in 0..box_meta.equipment_count {
            let offset = (i * 0x8) as usize;

            let charm_location = self
                .process
                .read_value_at(box_meta.equipment_location + offset);

            if let Some(charm) = self.get_charm_at(charm_location) {
                charms.push(charm)
            }
        }

        charms
    }

    pub fn get_box_metadata(&self, box_location: usize) -> data::EquipBoxMetadata {
        let meta_ptr: usize = self
            .process
            .read_value_at(box_location + constants::offsets::EQUIPMENT_LIST);

        dbg!(meta_ptr);

        let equipment_count = self
            .process
            .read_value_at(meta_ptr + constants::offsets::EQUIPMENT_SIZE);

        //for some reason offsets again here
        //copied from
        //https://github.com/Fexty12573/mhr-charm-item-editor/blob/8566ad0928c5689e0312a34775ab4201887ab340/RisePCItemEditor/Window.cpp#L989
        //and seems to work
        let equipment_location: usize = self
            .process
            .read_value_at::<usize>(meta_ptr + constants::offsets::EQUIPMENT_ITEMS)
            + constants::offsets::EQUIPMENT_LIST;

        data::EquipBoxMetadata {
            equipment_count,
            equipment_location,
        }
    }

    pub fn get_charm_at(&self, charm_location: usize) -> Option<data::Charm> {
        let equip_type: u32 = self
            .process
            .read_value_at(charm_location + constants::offsets::EQUIPMENT_TYPE);

        if equip_type != constants::CHARM_EQUIPMENT_TYPE {
            return None;
        }

        let slots = self.get_charm_slots(charm_location);
        let skills = self.get_charm_skills(charm_location);
        let levels = self.get_charm_skill_levels(charm_location);

        Some(data::Charm {
            primary_skill: data::CharmSkill {
                skill_id: skills[0],
                skill_level: levels[0],
            },

            secondary_skill: data::CharmSkill {
                skill_id: skills[1],
                skill_level: levels[1],
            },

            slot_levels: slots,
        })
    }

    pub fn get_charm_slots(&self, charm_location: usize) -> [u32; 3] {
        let slot_ptr: usize = self
            .process
            .read_value_at(charm_location + constants::offsets::SLOT_POINTER);

        dbg!(slot_ptr);

        self.process
            .read_value_at(slot_ptr + constants::offsets::SLOT_VALUES)
    }

    pub fn get_charm_skills(&self, charm_location: usize) -> [u8; 2] {
        let skill_ptr: usize = self
            .process
            .read_value_at(charm_location + constants::offsets::SKILL_ID_POINTER);

        dbg!(skill_ptr);

        self.process
            .read_value_at(skill_ptr + constants::offsets::SKILL_ID_VALUES)
    }

    pub fn get_charm_skill_levels(&self, charm_location: usize) -> [u32; 2] {
        let level_ptr: usize = self
            .process
            .read_value_at(charm_location + constants::offsets::SKILL_LVL_POINTER);

        dbg!(level_ptr);

        self.process
            .read_value_at(level_ptr + constants::offsets::SKILL_LVL_VALUES)
    }
}

pub trait ProcessHandleExt {
    fn read_value_at<T: Copy>(&self, location: usize) -> T;
}

impl ProcessHandleExt for ProcessHandle {
    fn read_value_at<T: Copy>(&self, location: usize) -> T {
        DataMember::<T>::new_offset(self.clone(), vec![location])
            .read()
            .unwrap()
    }
}
