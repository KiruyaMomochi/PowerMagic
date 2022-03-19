use std::collections::HashMap;
use tokio_stream::StreamExt;

use super::cache::UniqueEquipmentCache;
use crate::manager::{DataManager, DataManagerError};
use crate::model;
use crate::unit::*;

/// All data that needed to calculate the unit's stats.
#[derive(Debug)]
pub struct UnitData {
    pub unit_id: i64,
    pub promotion: Vec<UnitPromotion>,
    pub rarity: Vec<model::UnitRarity>,
    pub unique_equip: Vec<model::UnitUniqueEquip>,
    pub unlock_rarity_6: Option<[HashMap<i64, model::UnlockRarity6>; 3]>,
    pub skill_data: model::UnitSkillData,
    pub stories: HashMap<i64, StoryData>,
}

impl UnitData {
    pub fn base_param(&self, level: i32, rarity: i32, rank: i32) -> UnitStatus<i64> {
        let promotion = self.promotion[rank as usize - 1].clone();
        let rarity = self.rarity[rarity as usize - 1].clone();
        let rank_up = rank_up_bonus(rank);

        let mut param = rarity.status();

        if let Some(status) = promotion.status {
            param += status.status();
        }

        param += rarity.status_growth() * (level as f64);
        param += rarity.status_growth().component_mul(&rank_up);

        if let Some(status) = promotion.bonus {
            param += status.status();
        }

        param.map(|x| x.cy_round())
    }
}

#[derive(Debug, Default)]
pub struct StoryData(pub Vec<model::CharaStoryStatus>);

impl StoryData {
    pub fn param_append(&self, bonus_stories: usize, param: &mut UnitStatus<i64>) {
        self.0
            .iter()
            .take(bonus_stories)
            .flat_map(|status| &status.status)
            .for_each(|(index, value)| {
                param[*index as usize - 1] += value;
            });
    }

    pub fn param(&self, bonus_stories: usize) -> UnitStatus<i64> {
        let mut param = UnitStatus::<i64>::zeros();
        self.0
            .iter()
            .take(bonus_stories)
            .flat_map(|status| &status.status)
            .for_each(|(index, value)| {
                param[*index as usize - 1] += value;
            });

        param
    }

    pub fn param_all(&self) -> UnitStatus<i64> {
        let mut param = UnitStatus::<i64>::zeros();
        self.0
            .iter()
            .flat_map(|status| &status.status)
            .for_each(|(index, value)| {
                param[*index as usize - 1] += value;
            });

        param
    }

    pub fn story_group(&self) -> StoryGroup {
        StoryGroup {
            story_group_id: self.0[0].story_id / 1000,
            total: self.0.len(),
            watched: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitPromotion {
    pub promotion: model::UnitPromotion,
    pub status: Option<model::UnitPromotionStatus>,
    pub bonus: Option<model::PromotionBonus>,
}

#[derive(Debug)]
pub struct EquipmentData {
    pub id: i64,
    pub data: model::EquipmentData,
    pub enhance_rate: model::EquipmentEnhanceRate,
    pub max_enhance_level: i32,
}

impl EquipmentData {
    pub fn param(&self, enhancement_level: i32) -> UnitStatus<f64> {
        self.data.status()
            + (self.enhance_rate.status() * (enhancement_level as f64)).map(|x| x.ceil())
    }
}

#[derive(Debug)]
pub struct UniqueEquipmentData {
    pub id: i64,
    pub data: model::UniqueEquipmentData,
    pub enhance_rate: model::UniqueEquipmentEnhanceRate,
    pub max_enhancement_level: i32,
}

impl UniqueEquipmentData {
    pub fn param(&self, enhancement_level: i32) -> UnitStatus<f64> {
        self.data.status()
            + (self.enhance_rate.status()
                * (enhancement_level + model::UniqueEquipmentEnhanceRate::ENHANCE_LV_OFFSET) as f64)
                .map(|x| x.ceil())
    }

    pub fn cached(&self) -> UniqueEquipmentCache {
        UniqueEquipmentCache {
            id: self.id,
            status: self.data.status(),
            enhance_rate: self.enhance_rate.status(),
            max_enhancement_level: self.max_enhancement_level,
        }
    }
}

impl DataManager {
    pub async fn equip_data(&self, equipment_id: i64) -> Result<EquipmentData, DataManagerError> {
        if equipment_id == 999999i64 {
            return Err(DataManagerError::EquipNotFound(equipment_id));
        }

        let equipment_data = sqlx::query_as::<_, model::EquipmentData>(
            "SELECT * FROM equipment_data WHERE equipment_id == $1",
        )
        .bind(equipment_id)
        .fetch_one(&self.pool)
        .await?;

        let equipment_enhance_rate = sqlx::query_as::<_, model::EquipmentEnhanceRate>(
            "SELECT * FROM equipment_enhance_rate WHERE equipment_id == $1",
        )
        .bind(equipment_id)
        .fetch_one(&self.pool)
        .await?;

        const EMPTY_VECTOR: Vec<model::EquipmentEnhanceData> = Vec::new();
        let max_enhance_level = self
            .equipment_enhance_data
            .get(&equipment_data.promotion_level)
            .unwrap_or(&EMPTY_VECTOR)
            .iter()
            .map(|x| x.equipment_enhance_level)
            .max()
            .unwrap_or(0);

        Ok(EquipmentData {
            id: equipment_id,
            data: equipment_data,
            enhance_rate: equipment_enhance_rate,
            max_enhance_level: max_enhance_level as i32,
        })
    }

    pub async fn unique_equip_data(
        &self,
        unique_equipment_id: i64,
    ) -> Result<UniqueEquipmentData, DataManagerError> {
        let unique_equipment_data = sqlx::query_as::<_, model::UniqueEquipmentData>(
            "SELECT * FROM unique_equipment_data WHERE equipment_id == $1",
        )
        .bind(unique_equipment_id)
        .fetch_one(&self.pool)
        .await?;

        let unique_equipment_enhance_rate = sqlx::query_as::<_, model::UniqueEquipmentEnhanceRate>(
            "SELECT * FROM unique_equipment_enhance_rate WHERE equipment_id == $1",
        )
        .bind(unique_equipment_id)
        .fetch_one(&self.pool)
        .await?;

        let max_enhancement_level = self
            .unique_equipment_enhance_data
            .get(&1)
            .unwrap()
            .iter()
            .map(|x| x.enhance_level)
            .max()
            .ok_or(DataManagerError::UniqueEquipmentEnhanceData(
                unique_equipment_id,
            ))?;

        Ok(UniqueEquipmentData {
            id: unique_equipment_id,
            data: unique_equipment_data,
            enhance_rate: unique_equipment_enhance_rate,
            max_enhancement_level: max_enhancement_level as i32,
        })
    }

    pub async fn unit_data(&self, unit_id: i64) -> Result<UnitData, DataManagerError> {
        // let promotion_bonus = HashMap::new();

        // RIGHT and FULL OUTER JOINs are not currently supported
        let mut promotion: Vec<UnitPromotion> = sqlx::query_as::<_, model::UnitPromotion>(
            "SELECT * FROM unit_promotion WHERE unit_promotion.unit_id == $1 ORDER BY promotion_level ASC",
        )
        .bind(unit_id)
        // .bind(rank)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|promotion| {
            UnitPromotion {
                promotion,
                bonus: None,
                status: None
            }
        })
        .collect();

        let mut promotion_status = sqlx::query_as::<_, model::UnitPromotionStatus>(
            "SELECT * FROM unit_promotion_status WHERE unit_id == $1 ORDER BY promotion_level ASC",
        )
        .bind(unit_id)
        // .bind(rank)
        .fetch(&self.pool);

        while let Some(promotion_status) = promotion_status.try_next().await? {
            let rank = promotion_status.promotion_level as usize;
            assert_eq!(
                promotion[rank - 1].promotion.promotion_level,
                promotion_status.promotion_level
            );
            promotion[rank - 1].status = Some(promotion_status);
        }

        let mut promotion_bonus = sqlx::query_as::<_, model::PromotionBonus>(
            "SELECT * FROM promotion_bonus WHERE unit_id == $1",
        )
        .bind(unit_id)
        // .bind(rank)
        .fetch(&self.pool);

        while let Some(promotion_bonus) = promotion_bonus.try_next().await? {
            let rank = promotion_bonus.promotion_level as usize;
            promotion[rank - 1].bonus = Some(promotion_bonus);
        }

        let rarity = sqlx::query_as::<_, model::UnitRarity>(
            "SELECT * FROM unit_rarity WHERE unit_id == $1 ORDER BY rarity ASC",
        )
        .bind(unit_id)
        .fetch_all(&self.pool)
        .await?;

        let unlock_rarity_6_rows: Vec<model::UnlockRarity6> =
            sqlx::query_as::<_, model::UnlockRarity6>(
                "SELECT * FROM unlock_rarity_6 WHERE unit_id == $1 AND unlock_level != 0 ORDER BY slot_id, unlock_level",
            )
            .bind(unit_id)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let unlock_rarity_6 = if !unlock_rarity_6_rows.is_empty() {
            let mut slots = [
                HashMap::<i64, model::UnlockRarity6>::new(),
                HashMap::<i64, model::UnlockRarity6>::new(),
                HashMap::<i64, model::UnlockRarity6>::new(),
            ];

            for row in unlock_rarity_6_rows.into_iter() {
                if row.slot_id > 3 {
                    return Err(DataManagerError::UnlockRarity6SlotOutOfRange(row.slot_id));
                }

                slots[row.slot_id as usize - 1].insert(row.unlock_level, row);
            }

            for (i, slot) in slots.iter().enumerate() {
                if slot.is_empty() {
                    return Err(DataManagerError::UnlockRarity6NotFound(i));
                }
            }

            Some(slots)
        } else {
            None
        };

        // .map(|unlock_rarity_6| {
        //     (
        //         (unlock_rarity_6.slot_id, unlock_rarity_6.unlock_level),
        //         unlock_rarity_6,
        //     )
        // })
        // .collect();

        let unique_equip = sqlx::query_as::<_, model::UnitUniqueEquip>(
            "SELECT * FROM unit_unique_equip WHERE unit_id == $1 ORDER BY equip_slot ASC",
        )
        .bind(unit_id)
        .fetch_all(&self.pool)
        .await?;

        let skill_data = sqlx::query_as::<_, model::UnitSkillData>(
            "SELECT * FROM unit_skill_data WHERE unit_id == $1",
        )
        .bind(unit_id)
        .fetch_one(&self.pool)
        .await?;

        let story_bonus_vec = sqlx::query_as::<_, model::CharaStoryStatus>(
            "SELECT * FROM chara_story_status WHERE $1 in (chara_id_1, chara_id_2, chara_id_3, chara_id_4, chara_id_5, chara_id_6, chara_id_7, chara_id_8, chara_id_9, chara_id_10) ORDER BY story_id ASC",
        )
        .bind(unit_id / 100)
        .fetch_all(&self.pool)
        .await?;

        let mut story_status: HashMap<i64, StoryData> = HashMap::new();

        for story_bonus_item in story_bonus_vec.into_iter() {
            let story_id = story_bonus_item.story_id;
            let story_group_id = story_id / 1000;
            let entry = story_status.entry(story_group_id).or_default();
            entry.0.push(story_bonus_item);
        }

        Ok(UnitData {
            unit_id,
            rarity,
            unlock_rarity_6,
            promotion,
            unique_equip,
            skill_data,
            stories: story_status,
        })
    }
}
