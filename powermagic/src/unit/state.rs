use std::collections::HashMap;

use crate::model;
use crate::unit::*;

/// State of a unit
/// All parameters needed to calculate the unit's power
/// Does not include actual unit data
#[derive(Debug)]
pub struct UnitState {
    /// Unit id
    pub id: i64,
    /// Number of stars
    pub rarity: i32,
    /// Lv
    pub level: i32,
    /// Rank
    pub promotion: i32,
    /// Skill,
    pub skill: UnitSkill,
    /// Equipment slots
    pub equip_slot: Vec<EquipSlot>,
    /// Unique equipment slots
    pub unique_equip_slot: Vec<EquipSlot>,
    /// Unlock rarity 6 slot
    pub unlock_rarity_6_slot: Option<UnlockRarity6Slot>,
    /// Watched stories
    pub story: HashMap<i64, StoryGroup>,
}

impl model::UnitSkillData {
    pub fn unit_skill_state(&self) -> UnitSkill {
        let skill = self;

        let mut union_burst = vec![];
        let mut main_skill = vec![];
        let mut ex_skill = vec![];
        let free_skill = vec![];

        for i in 0..skill.union_burst.len() {
            union_burst.push(SkillLevelInfo {
                skill_id: skill.union_burst[i],
                skill_evolution_id: if i == 0 && skill.union_burst_evolution != 0 {
                    Some(skill.union_burst_evolution)
                } else {
                    None
                },
                skill_level: 1,
            })
        }

        for i in 0..skill.main_skill.len() {
            main_skill.push(SkillLevelInfo {
                skill_id: skill.main_skill[i],
                skill_evolution_id: skill.main_skill_evolution.get(i).and_then(|x| {
                    if *x == 0 {
                        None
                    } else {
                        Some(*x)
                    }
                }),
                skill_level: 1,
            })
        }

        for i in 0..skill.ex_skill.len() {
            ex_skill.push(SkillLevelInfo {
                skill_id: skill.ex_skill[i],
                skill_evolution_id: skill.ex_skill_evolution.get(i).and_then(|x| {
                    if *x == 0 {
                        None
                    } else {
                        Some(*x)
                    }
                }),
                skill_level: 1,
            })
        }

        UnitSkill {
            union_burst,
            main_skill,
            ex_skill,
            free_skill,
        }
    }
}

#[derive(Debug)]
pub struct UnitStateBuilder<'a> {
    unit_config: &'a UnitData,
    pub rarity: Option<i32>,
    pub level: Option<i32>,
    pub promotion: Option<i32>,
    pub skill: Option<UnitSkill>,
    pub equip_slot: Option<Vec<EquipSlot>>,
    pub unique_equip_slot: Vec<EquipSlot>,
    pub unlock_rarity_6_slot: Option<UnlockRarity6Slot>,
    pub story: HashMap<i64, StoryGroup>,
}

#[derive(thiserror::Error, Debug)]
pub enum UnitStateBuilderError {
    #[error("Rarity not set")]
    Rarity,
    #[error("Level not set")]
    Level,
    #[error("Promotion not set")]
    Promotion,
    #[error("Skill not set")]
    Skill,
    #[error("Equip slot not set")]
    EquipSlot,
    #[error("Unique equip slot not set")]
    UniqueEquipSlot,
    #[error("Unlock rarity 6 slot not set")]
    UnlockRarity6Slot,
    #[error("Story not set")]
    Story,
    #[error("Unit doesn't unlocked rarity 6")]
    Rarity6,
}

impl<'a> UnitStateBuilder<'a> {
    pub fn with_config(unit_config: &'a UnitData) -> UnitStateBuilder<'a> {
        let unlock_6 = &unit_config.unlock_rarity_6;
        let unlock_rarity_6_slot = if unlock_6.is_some() {
            Some(UnlockRarity6Slot {
                slot_1_level: 0,
                slot_2_level: 0,
                slot_3_level: 0,
            })
        } else {
            None
        };

        let unique_equip_slot = unit_config
            .unique_equip
            .iter()
            .map(|equip| {
                if equip.equip_id == 999999 {
                    EquipSlot::None
                } else {
                    EquipSlot::Unequipped {
                        id: equip.equip_id,
                        max_enhancement_level: None,
                    }
                }
            })
            .collect();

        let story = unit_config
            .stories
            .iter()
            .map(|(group_id, status)| {
                (
                    *group_id,
                    StoryGroup {
                        story_group_id: *group_id,
                        total: status.0.len(),
                        watched: 0,
                    },
                )
            })
            .collect();

        UnitStateBuilder {
            unit_config,
            rarity: None,
            level: None,
            promotion: None,
            skill: None,
            equip_slot: None,
            unique_equip_slot,
            unlock_rarity_6_slot,
            story,
        }
    }

    pub fn level(mut self, level: i32) -> UnitStateBuilder<'a> {
        self.level = Some(level);
        self
    }

    pub fn rank(mut self, rank: i32) -> UnitStateBuilder<'a> {
        self.promotion = Some(rank);
        let equipments = &self.unit_config.promotion[rank as usize - 1].promotion;
        let equip_slot = equipments
            .equip_slot
            .iter()
            .map(|equipment| {
                if *equipment == 999999 {
                    EquipSlot::None
                } else {
                    EquipSlot::Unequipped {
                        id: *equipment,
                        max_enhancement_level: None,
                    }
                }
            })
            .collect();
        self.equip_slot = Some(equip_slot);

        self
    }

    pub fn rarity(mut self, rarity: i32) -> UnitStateBuilder<'a> {
        self.rarity = Some(rarity);

        self.skill = Some(self.unit_config.skill_data.unit_skill_state());

        self
    }

    pub fn build(self) -> Result<UnitState, UnitStateBuilderError> {
        let rarity = self.rarity.ok_or(UnitStateBuilderError::Rarity)?;
        let level = self.level.ok_or(UnitStateBuilderError::Level)?;
        let promotion = self.promotion.ok_or(UnitStateBuilderError::Promotion)?;
        let skill = self.skill.ok_or(UnitStateBuilderError::Skill)?;
        let equip_slot = self.equip_slot.ok_or(UnitStateBuilderError::EquipSlot)?;
        let unique_equip_slot = self.unique_equip_slot;
        let unlock_rarity_6_slot = self.unlock_rarity_6_slot;
        let story = self.story;

        if unlock_rarity_6_slot.is_none() && rarity >= 6 {
            return Err(UnitStateBuilderError::Rarity6);
        }

        let unit_config = self.unit_config;
        let unit_data = UnitState {
            id: unit_config.unit_id,
            rarity,
            level,
            promotion,
            skill,
            equip_slot,
            unique_equip_slot,
            unlock_rarity_6_slot,
            story,
        };

        Ok(unit_data)
    }
}

impl model::UnitStatusCoefficient {
    fn generic_power_evolution(
        level: i32,
        is_evolution: bool,
        evolution_coefficient: f64,
        evolution_slv_coefficient: f64,
    ) -> f64 {
        if is_evolution {
            evolution_slv_coefficient * (level as f64) + evolution_coefficient as f64
        } else {
            level as f64
        }
    }

    pub fn union_burst_power(&self, level: i32, is_evolution: bool) -> f64 {
        Self::generic_power_evolution(
            level,
            is_evolution,
            self.ub_evolution_coefficient as f64,
            self.ub_evolution_slv_coefficient as f64,
        )
    }

    pub fn main_skill_power(&self, level: i32, is_evolution: bool) -> f64 {
        Self::generic_power_evolution(
            level,
            is_evolution,
            self.skill1_evolution_coefficient as f64,
            self.skill1_evolution_slv_coefficient as f64,
        )
    }

    pub fn ex_skill_power(&self, level: i32, is_evolution: bool) -> f64 {
        level as f64 + self.exskill_evolution_coefficient as f64
    }

    pub fn free_skill_power(&self, level: i32) -> f64 {
        level as f64
    }

    pub fn union_burst_id_power(&self, level: i32, skill_id: i64) -> f64 {
        self.union_burst_power(level, is_evolution_skill(skill_id))
    }

    pub fn main_skill_id_power(&self, level: i32, skill_id: i64) -> f64 {
        self.main_skill_power(level, is_evolution_skill(skill_id))
    }

    pub fn ex_skill_id_power(&self, level: i32, rarity: i32) -> f64 {
        self.ex_skill_power(level, level > 0 && rarity >= 5)
    }

    pub fn free_skill_id_power(&self, level: i32) -> f64 {
        level as f64
    }

    pub fn skill_power(&self, skill: &UnitSkill, level: i32, rarity: i32, unique_equipped: bool) -> f64 {
        skill_level(skill, level, self, rarity, unique_equipped)
    }
}

impl UnitCache {
    pub fn unit_state(&self) -> UnitState {
        UnitState {
            id: self.unit_id,
            rarity: 5,
            level: 1,
            promotion: 1,
            skill: self.skill.clone(),
            equip_slot: self.promotion[0].equipments_to_slots(),            
            unique_equip_slot: self
                .unique_equip
                .as_ref()
                .map_or(vec![], |e| vec![e.slot()]),
            unlock_rarity_6_slot: self.unlock_rarity_6.as_ref().map(|_e| UnlockRarity6Slot {
                slot_1_level: 0,
                slot_2_level: 0,
                slot_3_level: 0,
            }),
            story: self
                .story
                .iter()
                .map(|(k, v)| (*k, v.story_group()))
                .collect(),
        }
    }
}
