use std::collections::HashMap;

use super::data::*;
use super::utils::*;
use crate::manager::{DataManager, DataManagerError};
use crate::model;
use crate::unit::*;

// TODO: conversion unit

/// Cached data for faster power calculation
#[derive(Debug)]
pub struct UnitCache {
    /// Id
    pub unit_id: i64,
    /// Skill
    pub skill: UnitSkill,
    /// Star
    pub rarity: Vec<UnitRarityCache>,
    /// Rank promotions
    pub promotion: Vec<UnitPromotionCache>,
    /// Union burst
    pub unique_equip: Option<UniqueEquipmentCache>,
    /// Unlock rarity 6 slot
    pub unlock_rarity_6: Option<[Vec<UnlockRarity6Cache>; 3]>,
    /// Stories
    pub story: HashMap<i64, StoryData>,
    pub status_coefficient: model::UnitStatusCoefficient,
    pub status_coefficient_cache: UnitStatus<f64>,
}

impl UnitCache {
    pub fn base_param(&self, rank: i32, level: i32, rarity: i32) -> UnitStatus<i64> {
        let rarity = &self.rarity[rarity as usize - 1];
        let rank_up = rank_up_bonus(rank);
        let mut param = rarity.status;
        param += rarity.growth * (level as f64);
        param += rarity.growth.component_mul(&rank_up);

        let promotion = &self.promotion[rank as usize - 1];
        if promotion.status.is_some() {
            param += promotion.status.unwrap();
        }
        if promotion.bonus.is_some() {
            param += promotion.bonus.unwrap();
        }

        param.map(|x| x.cy_round())
    }

    pub fn equip_param(&self, rank: i32, equip: &[(bool, i32)]) -> UnitStatus<f64> {
        let promotion = &self.promotion[rank as usize - 1];
        let mut param = UnitStatus::zeros();
        promotion.equipments.iter().zip(equip.iter()).for_each(
            |(equipment, (equipped, enhance_level))| {
                if !(*equipped) {
                    return;
                }

                if equipment.is_none() {
                    return;
                }

                param += equipment.as_ref().unwrap().param(*enhance_level);
            },
        );

        param
    }

    pub fn unique_equip_param(&self, enhancement_level: i32) -> UnitStatus<f64> {
        self.unique_equip.as_ref().unwrap().param(enhancement_level)
    }

    pub fn unlock_rarity_6_param(&self, level: [i32; 3]) -> UnitStatus<i64> {
        let mut param = UnitStatus::zeros();

        self.unlock_rarity_6
            .as_ref()
            .unwrap()
            .iter()
            .zip(level)
            .filter(|(_, l)| *l > 0)
            .map(|(r, l)| &r[l as usize - 1])
            .for_each(|r| {
                param += r.status;
            });

        param
    }

    pub fn story_param(&self, watched_bonus: HashMap<i64, i64>) -> UnitStatus<i64> {
        let mut param = UnitStatus::zeros();
        for (group_id, stories) in self.story.iter() {
            let watched = watched_bonus.get(group_id).unwrap_or(&0);
            stories.param_append(*watched as usize, &mut param);
        }

        param
    }

    pub fn skill_level(&self, level: i32, rarity: i32, is_unique_equipped: bool) -> f64 {
        skill_level(
            &self.skill,
            level,
            &self.status_coefficient,
            rarity,
            rarity >= 6 || is_unique_equipped,
        )
    }
}

#[derive(Debug)]
pub struct UnlockRarity6Cache {
    pub status: UnitStatus<i64>,
}

#[derive(Debug)]
pub struct EquipmentCache {
    pub id: i64,
    pub data: UnitStatus<f64>,
    pub enhance_rate: UnitStatus<f64>,
    pub max_enhance_level: i32,
}

impl EquipmentCache {
    pub fn param(&self, enhancement_level: i32) -> UnitStatus<f64> {
        self.data
            + (self.enhance_rate
                * (enhancement_level + model::EquipmentEnhanceRate::ENHANCE_LV_OFFSET) as f64)
                .map(|x| x.ceil())
    }
}

impl Slot for EquipmentCache {
    fn slot(&self) -> EquipSlot {
        EquipSlot::Unequipped {
            id: self.id,
            max_enhancement_level: Some(self.max_enhance_level),
        }
    }
}

#[derive(Debug)]
pub struct UniqueEquipmentCache {
    pub id: i64,
    pub status: UnitStatus<f64>,
    pub enhance_rate: UnitStatus<f64>,
    pub max_enhancement_level: i32,
}

impl UniqueEquipmentCache {
    pub fn param(&self, enhancement_level: i32) -> UnitStatus<f64> {
        self.status
            + (self.enhance_rate
                * (enhancement_level + model::UniqueEquipmentEnhanceRate::ENHANCE_LV_OFFSET) as f64)
                .map(|x| x.ceil())
    }
}

impl Slot for UniqueEquipmentCache {
    fn slot(&self) -> EquipSlot {
        EquipSlot::Unequipped {
            id: self.id,
            max_enhancement_level: Some(self.max_enhancement_level),
        }
    }
}

#[derive(Debug)]
pub struct UnitPromotionCache {
    pub equipments: Vec<Option<EquipmentCache>>,
    pub status: Option<UnitStatus<f64>>,
    pub bonus: Option<UnitStatus<f64>>,
}

impl UnitPromotionCache {
    pub fn equipments_to_slots(&self) -> Vec<EquipSlot> {
        self.equipments
            .iter()
            .map(|e| e.as_ref().map_or(EquipSlot::None, |e| e.slot()))
            .collect()
    }
}

#[derive(Debug)]
pub struct UnitRarityCache {
    pub status: UnitStatus<f64>,
    pub growth: UnitStatus<f64>,
}

impl DataManager {
    pub async fn unit_cache(&self, unit_id: i64) -> Result<UnitCache, DataManagerError> {
        let unit_config = self.unit_data(unit_id).await?;

        let mut promotion_cache = vec![];
        for promotion in unit_config.promotion.iter() {
            let promotion_status: Option<UnitStatus<f64>> =
                promotion.status.as_ref().map(|s| s.status());
            let promotion_bonus: Option<UnitStatus<f64>> =
                promotion.bonus.as_ref().map(|b| b.status());
            let mut equipment_status = vec![];

            for equipment_id in promotion.promotion.equip_slot {
                if equipment_id == 999999 {
                    equipment_status.push(None)
                } else {
                    equipment_status.push(Some(self.equip_data(equipment_id).await?.cached()));
                }
            }

            promotion_cache.push(UnitPromotionCache {
                equipments: equipment_status,
                status: promotion_status,
                bonus: promotion_bonus,
            })
        }

        let unique_equip = if unit_config.unique_equip.len() == 1 {
            Some(
                self.unique_equip_data(unit_config.unique_equip[0].equip_id)
                    .await?
                    .cached(),
            )
        } else if unit_config.unique_equip.is_empty() {
            None
        } else {
            return Err(DataManagerError::InvalidUniqueEquipMoreThanOne(unit_id))
        };

        let unlock_rarity_6_status = if let Some(config_unlock_rarity_6) =
            unit_config.unlock_rarity_6
        {
            let mut unlock_rarity_6_status: [Vec<UnlockRarity6Cache>; 3] = [vec![], vec![], vec![]];
            for (status, data_status) in unlock_rarity_6_status
                .iter_mut()
                .zip(config_unlock_rarity_6.iter())
            {
                *status = data_status
                    .iter()
                    .map(|(_, unlock)| unlock.cached())
                    .collect();
            }
            Some(unlock_rarity_6_status)
        } else {
            None
        };

        let rarity = unit_config.rarity.iter().map(|r| r.cached()).collect();

        Ok(UnitCache {
            unit_id,
            rarity,
            skill: unit_config.skill_data.unit_skill_state(),
            status_coefficient: self.status_coefficient.clone(),
            promotion: promotion_cache,
            unique_equip,
            unlock_rarity_6: unlock_rarity_6_status,
            story: unit_config.stories,
            status_coefficient_cache: self.status_coefficient.status_coefficient(),
        })
    }
}

impl model::UnitRarity {
    pub fn cached(&self) -> UnitRarityCache {
        UnitRarityCache {
            status: self.status(),
            growth: self.status_growth(),
        }
    }
}

impl model::UnlockRarity6 {
    pub fn cached(&self) -> UnlockRarity6Cache {
        UnlockRarity6Cache {
            status: self.status(),
        }
    }
}

impl super::data::EquipmentData {
    pub fn cached(&self) -> EquipmentCache {
        EquipmentCache {
            id: self.id,
            data: self.data.status(),
            enhance_rate: self.enhance_rate.status(),
            max_enhance_level: self.max_enhance_level,
        }
    }
}
