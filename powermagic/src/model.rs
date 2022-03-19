use derive_macro::impl_status;
use sqlx::Row;

// use crate::data::StatusParam;

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status("{}_coefficient")]
pub struct UnitStatusCoefficient {
    pub coefficient_id: i64,
    pub hp_coefficient: f64,
    pub atk_coefficient: f64,
    pub magic_str_coefficient: f64,
    pub def_coefficient: f64,
    pub magic_def_coefficient: f64,
    pub physical_critical_coefficient: f64,
    pub magic_critical_coefficient: f64,
    pub wave_hp_recovery_coefficient: f64,
    pub wave_energy_recovery_coefficient: f64,
    pub dodge_coefficient: f64,
    pub physical_penetrate_coefficient: f64,
    pub magic_penetrate_coefficient: f64,
    pub life_steal_coefficient: f64,
    pub hp_recovery_rate_coefficient: f64,
    pub energy_recovery_rate_coefficient: f64,
    pub energy_reduce_rate_coefficient: f64,
    pub skill_lv_coefficient: f64,
    pub exskill_evolution_coefficient: i64,
    pub overall_coefficient: f64,
    pub accuracy_coefficient: f64,
    pub skill1_evolution_coefficient: i64,
    pub skill1_evolution_slv_coefficient: f64,
    pub ub_evolution_coefficient: i64,
    pub ub_evolution_slv_coefficient: f64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UnitData {
    pub unit_id: i64,
    pub unit_name: String,
    pub kana: String,
    pub prefab_id: i64,
    pub prefab_id_battle: i64,
    pub is_limited: i64,
    pub rarity: i64,
    pub motion_type: i64,
    pub se_type: i64,
    pub move_speed: i64,
    pub search_area_width: i64,
    pub atk_type: i64,
    pub normal_atk_cast_time: f64,
    pub cutin_1: i64,
    pub cutin_2: i64,
    pub cutin1_star6: i64,
    pub cutin2_star6: i64,
    pub guild_id: i64,
    pub exskill_display: i64,
    pub comment: String,
    pub only_disp_owned: i64,
    pub start_time: String,
    pub end_time: String,
    pub original_unit_id: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status]
pub struct PromotionBonus {
    pub unit_id: i64,
    pub promotion_level: i64,
    pub hp: f64,
    pub atk: f64,
    pub magic_str: f64,
    pub def: f64,
    pub magic_def: f64,
    pub physical_critical: f64,
    pub magic_critical: f64,
    pub wave_hp_recovery: f64,
    pub wave_energy_recovery: f64,
    pub dodge: f64,
    pub physical_penetrate: f64,
    pub magic_penetrate: f64,
    pub life_steal: f64,
    pub hp_recovery_rate: f64,
    pub energy_recovery_rate: f64,
    pub energy_reduce_rate: f64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status("{}", "{}_growth")]
pub struct UnitRarity {
    pub unit_id: i64,
    pub rarity: i64,
    pub hp: f64,
    pub hp_growth: f64,
    pub atk: f64,
    pub atk_growth: f64,
    pub magic_str: f64,
    pub magic_str_growth: f64,
    pub def: f64,
    pub def_growth: f64,
    pub magic_def: f64,
    pub magic_def_growth: f64,
    pub physical_critical: f64,
    pub physical_critical_growth: f64,
    pub magic_critical: f64,
    pub magic_critical_growth: f64,
    pub wave_hp_recovery: f64,
    pub wave_hp_recovery_growth: f64,
    pub wave_energy_recovery: f64,
    pub wave_energy_recovery_growth: f64,
    pub dodge: f64,
    pub dodge_growth: f64,
    pub physical_penetrate: f64,
    pub physical_penetrate_growth: f64,
    pub magic_penetrate: f64,
    pub magic_penetrate_growth: f64,
    pub life_steal: f64,
    pub life_steal_growth: f64,
    pub hp_recovery_rate: f64,
    pub hp_recovery_rate_growth: f64,
    pub energy_recovery_rate: f64,
    pub energy_recovery_rate_growth: f64,
    pub energy_reduce_rate: f64,
    pub energy_reduce_rate_growth: f64,
    pub unit_material_id: i64,
    pub consume_num: i64,
    pub consume_gold: i64,
    pub accuracy: f64,
    pub accuracy_growth: f64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status]
pub struct UnitPromotionStatus {
    pub unit_id: i64,
    pub promotion_level: i64,
    pub hp: f64,
    pub atk: f64,
    pub magic_str: f64,
    pub def: f64,
    pub magic_def: f64,
    pub physical_critical: f64,
    pub magic_critical: f64,
    pub wave_hp_recovery: f64,
    pub wave_energy_recovery: f64,
    pub dodge: f64,
    pub physical_penetrate: f64,
    pub magic_penetrate: f64,
    pub life_steal: f64,
    pub hp_recovery_rate: f64,
    pub energy_recovery_rate: f64,
    pub energy_reduce_rate: f64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status]
pub struct UnlockRarity6 {
    pub unit_id: i64,
    pub slot_id: i64,
    pub unlock_level: i64,
    pub unlock_flag: i64,
    pub consume_gold: i64,
    pub material_type: i64,
    pub material_id: i64,
    pub material_count: i64,
    pub hp: i64,
    pub atk: i64,
    pub magic_str: i64,
    pub def: i64,
    pub magic_def: i64,
    pub physical_critical: i64,
    pub magic_critical: i64,
    pub wave_hp_recovery: i64,
    pub wave_energy_recovery: i64,
    pub dodge: i64,
    pub physical_penetrate: i64,
    pub magic_penetrate: i64,
    pub life_steal: i64,
    pub hp_recovery_rate: i64,
    pub energy_recovery_rate: i64,
    pub energy_reduce_rate: i64,
    pub accuracy: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status]
pub struct EquipmentData {
    pub equipment_id: i64,
    pub equipment_name: String,
    pub description: String,
    pub promotion_level: i64,
    pub craft_flg: i64,
    pub equipment_enhance_point: i64,
    pub sale_price: i64,
    pub require_level: i64,
    pub hp: f64,
    pub atk: f64,
    pub magic_str: f64,
    pub def: f64,
    pub magic_def: f64,
    pub physical_critical: f64,
    pub magic_critical: f64,
    pub wave_hp_recovery: f64,
    pub wave_energy_recovery: f64,
    pub dodge: f64,
    pub physical_penetrate: f64,
    pub magic_penetrate: f64,
    pub life_steal: f64,
    pub hp_recovery_rate: f64,
    pub energy_recovery_rate: f64,
    pub energy_reduce_rate: f64,
    pub enable_donation: i64,
    pub accuracy: f64,
    pub display_item: i64,
    pub item_type: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status]
pub struct EquipmentEnhanceRate {
    pub equipment_id: i64,
    pub equipment_name: String,
    pub description: String,
    pub promotion_level: i64,
    pub hp: f64,
    pub atk: f64,
    pub magic_str: f64,
    pub def: f64,
    pub magic_def: f64,
    pub physical_critical: f64,
    pub magic_critical: f64,
    pub wave_hp_recovery: f64,
    pub wave_energy_recovery: f64,
    pub dodge: f64,
    pub physical_penetrate: f64,
    pub magic_penetrate: f64,
    pub life_steal: f64,
    pub hp_recovery_rate: f64,
    pub energy_recovery_rate: f64,
    pub energy_reduce_rate: f64,
    pub accuracy: f64,
}

impl EquipmentEnhanceRate {
    pub const ENHANCE_LV_OFFSET: i32 = 0;
}

#[derive(Debug, Clone)]
pub struct UnitPromotion {
    pub unit_id: i64,
    pub promotion_level: i64,
    pub equip_slot: [i64; 6],
    // pub equip_slot_1: i64,
    // pub equip_slot_2: i64,
    // pub equip_slot_3: i64,
    // pub equip_slot_4: i64,
    // pub equip_slot_5: i64,
    // pub equip_slot_6: i64,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for UnitPromotion {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let len = row.len();
        if len != 8 {
            return Err(sqlx::Error::ColumnNotFound(
                format!("UnitPromotion has {} lines, expected 8", len),
            ));
        }

        Ok(UnitPromotion {
            unit_id: row.try_get("unit_id")?,
            promotion_level: row.try_get("promotion_level")?,
            equip_slot: [
                row.try_get("equip_slot_1")?,
                row.try_get("equip_slot_2")?,
                row.try_get("equip_slot_3")?,
                row.try_get("equip_slot_4")?,
                row.try_get("equip_slot_5")?,
                row.try_get("equip_slot_6")?,
            ],
        })
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EquipmentEnhanceData {
    pub promotion_level: i64,
    pub equipment_enhance_level: i64,
    pub needed_point: i64,
    pub total_point: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UniqueEquipmentEnhanceData {
    pub equip_slot: i64,
    pub enhance_level: i64,
    pub needed_point: i64,
    pub total_point: i64,
    pub needed_mana: i64,
    pub rank: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status]
pub struct UniqueEquipmentData {
    pub equipment_id: i64,
    pub equipment_name: String,
    pub description: String,
    pub promotion_level: i64,
    pub craft_flg: i64,
    pub equipment_enhance_point: i64,
    pub sale_price: i64,
    pub require_level: i64,
    pub hp: f64,
    pub atk: f64,
    pub magic_str: f64,
    pub def: f64,
    pub magic_def: f64,
    pub physical_critical: f64,
    pub magic_critical: f64,
    pub wave_hp_recovery: f64,
    pub wave_energy_recovery: f64,
    pub dodge: f64,
    pub physical_penetrate: f64,
    pub magic_penetrate: f64,
    pub life_steal: f64,
    pub hp_recovery_rate: f64,
    pub energy_recovery_rate: f64,
    pub energy_reduce_rate: f64,
    pub enable_donation: i64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[impl_status]
pub struct UniqueEquipmentEnhanceRate {
    pub equipment_id: i64,
    pub equipment_name: String,
    pub description: String,
    pub promotion_level: i64,
    pub hp: f64,
    pub atk: f64,
    pub magic_str: f64,
    pub def: f64,
    pub magic_def: f64,
    pub physical_critical: f64,
    pub magic_critical: f64,
    pub wave_hp_recovery: f64,
    pub wave_energy_recovery: f64,
    pub dodge: f64,
    pub physical_penetrate: f64,
    pub magic_penetrate: f64,
    pub life_steal: f64,
    pub hp_recovery_rate: f64,
    pub energy_recovery_rate: f64,
    pub energy_reduce_rate: f64,
    pub accuracy: f64,
}

impl UniqueEquipmentEnhanceRate {
    pub const ENHANCE_LV_OFFSET: i32 = -1;
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UnitUniqueEquip {
    pub unit_id: i64,
    pub equip_slot: i64,
    pub equip_id: i64,
}

#[derive(Debug, Clone)]
pub struct UnitSkillData {
    pub unit_id: i64,
    // pub union_burst: i64,
    pub union_burst: [i64; 1],
    pub union_burst_evolution: i64,
    // pub main_skill_1: i64,
    // pub main_skill_2: i64,
    // pub main_skill_3: i64,
    // pub main_skill_4: i64,
    // pub main_skill_5: i64,
    // pub main_skill_6: i64,
    // pub main_skill_7: i64,
    // pub main_skill_8: i64,
    // pub main_skill_9: i64,
    // pub main_skill_10: i64,
    pub main_skill: Vec<i64>,
    // pub ex_skill_1: i64,
    // pub ex_skill_evolution_1: i64,
    // pub ex_skill_2: i64,
    // pub ex_skill_evolution_2: i64,
    // pub ex_skill_3: i64,
    // pub ex_skill_evolution_3: i64,
    // pub ex_skill_4: i64,
    // pub ex_skill_evolution_4: i64,
    // pub ex_skill_5: i64,
    // pub ex_skill_evolution_5: i64,
    pub ex_skill: Vec<i64>,
    pub ex_skill_evolution: Vec<i64>,
    pub sp_union_burst: i64,
    // pub sp_skill_1: i64,
    // pub sp_skill_2: i64,
    // pub sp_skill_3: i64,
    // pub sp_skill_4: i64,
    // pub sp_skill_5: i64,
    pub sp_skill: Vec<i64>,
    // pub main_skill_evolution_1: i64,
    // pub main_skill_evolution_2: i64,
    pub main_skill_evolution: Vec<i64>,
    // pub sp_skill_evolution_1: i64,
    // pub sp_skill_evolution_2: i64,
    pub sp_skill_evolution: Vec<i64>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for UnitSkillData
{
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {

        let len = row.len();
        if len != 33 {
            return Err(sqlx::Error::ColumnNotFound(
                format!("UnitSkillData has {} lines, expected 33", len),
            ));
        }

        let unit_id = row.try_get("unit_id")?;
        let union_burst = [row.try_get("union_burst")?];

        let trim_zeros = |vector: &mut Vec<i64>| {
            while !vector.is_empty() && *vector.last().unwrap() == 0 {
                vector.pop();
            }
        };

        let mut main_skill = Vec::new();
        for i in 1..=10 {
            let skill_id = row.try_get(&*format!("main_skill_{}", i))?;
            main_skill.push(skill_id);
        }
        trim_zeros(&mut main_skill);

        let mut ex_skill = Vec::new();
        for i in 1..=5 {
            let skill_id = row.try_get(&*format!("ex_skill_{}", i))?;
            ex_skill.push(skill_id);
        }
        trim_zeros(&mut ex_skill);

        let mut ex_skill_evolution = Vec::new();
        for i in 1..=5 {
            let skill_id = row.try_get(&*format!("ex_skill_evolution_{}", i))?;
            ex_skill_evolution.push(skill_id);
        }
        trim_zeros(&mut ex_skill_evolution);

        let sp_union_burst = row.try_get("sp_union_burst")?;

        let mut sp_skill = Vec::new();
        for i in 1..=5 {
            let skill_id = row.try_get(&*format!("sp_skill_{}", i))?;
                sp_skill.push(skill_id);
        }        
        trim_zeros(&mut sp_skill);
        
        let union_burst_evolution = row.try_get("union_burst_evolution")?;

        let mut main_skill_evolution = Vec::new();
        for i in 1..=2 {
            let skill_id = row.try_get(&*format!("main_skill_evolution_{}", i))?;
            main_skill_evolution.push(skill_id);
        }
        trim_zeros(&mut main_skill_evolution);
        
        let mut sp_skill_evolution = Vec::new();
        for i in 1..=2 {
            let skill_id = row.try_get(&*format!("sp_skill_evolution_{}", i))?;
            sp_skill_evolution.push(skill_id);
        }
        trim_zeros(&mut sp_skill_evolution);

        Ok(UnitSkillData {
            unit_id,
            union_burst,
            main_skill,
            ex_skill,
            ex_skill_evolution,
            sp_union_burst,
            sp_skill,
            union_burst_evolution,
            main_skill_evolution,
            sp_skill_evolution,
        })
    }
}


#[derive(Debug, Clone)]
pub struct CharaStoryStatus {
    pub story_id: i64,
    pub unlock_story_name: String,
    // pub status_type_1: i64,
    // pub status_rate_1: i64,
    // pub status_type_2: i64,
    // pub status_rate_2: i64,
    // pub status_type_3: i64,
    // pub status_rate_3: i64,
    // pub status_type_4: i64,
    // pub status_rate_4: i64,
    // pub status_type_5: i64,
    // pub status_rate_5: i64,
    pub status: Vec<(i64, i64)>,
    // pub chara_id_1: i64,
    // pub chara_id_2: i64,
    // pub chara_id_3: i64,
    // pub chara_id_4: i64,
    // pub chara_id_5: i64,
    // pub chara_id_6: i64,
    // pub chara_id_7: i64,
    // pub chara_id_8: i64,
    // pub chara_id_9: i64,
    // pub chara_id_10: i64,
    pub chara_id: Vec<i64>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for CharaStoryStatus {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let len = row.len();
        if len != 22 {
            return Err(sqlx::Error::ColumnNotFound(
                format!("CharaStoryStatus has {} lines, expected 22", len),
            ));
        }

        let story_id = row.try_get("story_id")?;
        let unlock_story_name = row.try_get("unlock_story_name")?;

        let mut status = Vec::new();
        for i in 1..=5 {
            let status_type = row.try_get(&*format!("status_type_{}", i))?;
            let status_rate = row.try_get(&*format!("status_rate_{}", i))?;
            if status_type != 0 {
                status.push((status_type, status_rate));
            }
        }

        let mut chara_id = Vec::new();
        for i in 1..=10 {
            let chara_id_i = row.try_get(&*format!("chara_id_{}", i))?;
            if chara_id_i != 0 {
                chara_id.push(chara_id_i);
            }
        }

        Ok(CharaStoryStatus {
            story_id,
            unlock_story_name,
            status,
            chara_id,
        })
    }
}
