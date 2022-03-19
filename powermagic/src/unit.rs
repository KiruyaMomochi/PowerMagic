mod cache;
mod calc;
mod data;
mod define;
mod state;
mod utils;

pub use crate::model::UnitStatusCoefficient;
pub use cache::UnitCache;
pub use calc::{UnitCalculator, MemorizedUnitCalculator, StatusSetter};
pub use data::UnitData;
pub use state::{UnitState, UnitStateBuilder};
pub use utils::*;

pub type UnitStatus<T> = nalgebra::SVector<T, 17>;

#[derive(Debug, Clone)]
pub struct UnitSkill {
    /// Union bursts
    pub union_burst: Vec<SkillLevelInfo>,
    /// Main skill
    pub main_skill: Vec<SkillLevelInfo>,
    /// Extra skill
    pub ex_skill: Vec<SkillLevelInfo>,
    /// Free skill
    pub free_skill: Vec<SkillLevelInfo>,
}

impl UnitSkill {
    pub fn set_all_level(&mut self, level: i32) {
        self.union_burst.iter_mut().for_each(|x| {
            x.skill_level = level;
        });
        self.main_skill.iter_mut().for_each(|x| {
            x.skill_level = level;
        });
        self.ex_skill.iter_mut().for_each(|x| {
            x.skill_level = level;
        });
        self.free_skill.iter_mut().for_each(|x| {
            x.skill_level = level;
        });
    }
}

#[derive(Debug, Clone)]
pub struct SkillLevelInfo {
    pub skill_id: i64,
    pub skill_evolution_id: Option<i64>,
    pub skill_level: i32,
}

#[derive(Debug, Clone)]
pub enum EquipSlot {
    None,
    Unequipped {
        /// Equipment id
        id: i64,
        /// Max enhacnement level
        /// -1 means unknown.
        max_enhancement_level: Option<i32>,
    },
    Equipped {
        /// Equipment id
        id: i64,
        /// Enhancement level
        ///
        /// For equipment, this is number of stars
        /// For unique equipment, this is level.
        enhancement_level: i32,
        /// Max enhacnement level
        /// -1 means unknown.
        max_enhancement_level: Option<i32>,
    },
}

impl EquipSlot {
    pub fn is_equipped(&self) -> bool {
        matches!(self, EquipSlot::Equipped { .. })
    }

    pub fn is_none(&self) -> bool {
        matches!(self, EquipSlot::None)
    }

    pub fn equip(&mut self, level: i32) -> bool {
        match self {
            EquipSlot::Equipped {
                enhancement_level, ..
            } => {
                if *enhancement_level != level {
                    *enhancement_level = level;
                    true
                } else {
                    false
                }
            }
            EquipSlot::Unequipped {
                id,
                max_enhancement_level,
                ..
            } => {
                *self = EquipSlot::Equipped {
                    id: *id,
                    enhancement_level: level,
                    max_enhancement_level: *max_enhancement_level,
                };
                true
            }
            EquipSlot::None => false,
        }
    }

    pub fn equip_0(&mut self) -> bool {
        self.equip(0)
    }

    pub fn equip_full(&mut self) -> bool {
        match self {
            EquipSlot::Equipped {
                max_enhancement_level,
                enhancement_level,
                ..
            } => {
                let max_enhancement_level = max_enhancement_level.unwrap_or(0);
                if *enhancement_level != max_enhancement_level {
                    *enhancement_level = max_enhancement_level;
                    true
                } else {
                    false
                }
            }
            EquipSlot::Unequipped {
                id,
                max_enhancement_level,
                ..
            } => {
                *self = EquipSlot::Equipped {
                    id: *id,
                    enhancement_level: max_enhancement_level.unwrap_or(0),
                    max_enhancement_level: *max_enhancement_level,
                };
                true
            }
            EquipSlot::None => false,
        }
    }

    pub fn unequip(&mut self) -> bool {
        match self {
            EquipSlot::Equipped {
                id,
                max_enhancement_level,
                ..
            } => {
                *self = EquipSlot::Unequipped {
                    id: *id,
                    max_enhancement_level: *max_enhancement_level,
                };
                true
            }
            _ => false,
        }
    }
}

/// State of a unique equip slot
#[derive(Debug)]
pub struct UnlockRarity6Slot {
    /// The first slot, unit's memory piece
    ///
    /// If this is not equipped, this is 0,
    /// otherwise this is 1.
    pub slot_1_level: i32,
    /// The second slot, unit's pure memory piece
    ///
    /// If this is not equipped, this is 0,
    /// otherwise this is 1.
    pub slot_2_level: i32,
    /// The third slot, Princess Orb
    ///
    /// If this is not equipped, this is 0,
    /// otherwise this is stars on the equipment.
    ///
    /// **Note**: This value differs from the "enhancement_level"
    /// you see in game. It's always 1 larger than that.
    pub slot_3_level: i32,
}

impl UnlockRarity6Slot {
    fn slot_level(&self) -> [i32; 3] {
        [self.slot_1_level, self.slot_2_level, self.slot_3_level]
    }
}

/// State of a story group
#[derive(Debug)]
pub struct StoryGroup {
    story_group_id: i64,
    total: usize,
    watched: usize,
}

trait Slot {
    fn slot(&self) -> EquipSlot;
}

pub trait UnitStatusTrait<T> {
    fn hp(&self) -> T;
    fn atk(&self) -> T;
    fn def(&self) -> T;
    fn magic_str(&self) -> T;
    fn magic_def(&self) -> T;
    fn physical_critical(&self) -> T;
    fn magic_critical(&self) -> T;
    fn dodge(&self) -> T;
    fn life_steal(&self) -> T;
    fn wave_hp_recovery(&self) -> T;
    fn wave_energy_recovery(&self) -> T;
    fn physical_penetrate(&self) -> T;
    fn magic_penetrate(&self) -> T;
    fn energy_reduce_rate(&self) -> T;
    fn hp_recovery_rate(&self) -> T;
    fn energy_recovery_rate(&self) -> T;
    fn accuracy(&self) -> T;
}
