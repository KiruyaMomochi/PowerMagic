mod round;
pub use round::CyRound;

use crate::model;
use crate::unit::*;

pub fn is_evolution_skill<T>(skill_id: T) -> bool
where
    T: From<i32> + std::ops::Div<Output = T> + std::ops::Rem<Output = T> + std::cmp::PartialEq,
{
    skill_id / T::from(10) % T::from(10) == T::from(1)
}

pub fn skill_level<'a>(
    skill: &'a UnitSkill,
    level: i32,
    status_coefficient: &'a model::UnitStatusCoefficient,
    rarity: i32,
    is_unique_equipped: bool,
) -> f64 {
    let mut skill_level = 0f64;

    for _ in &skill.union_burst {
        skill_level += status_coefficient.union_burst_power(level, rarity >= 6);
    }

    for skill in &skill.main_skill {
        skill_level += status_coefficient.main_skill_power(
            level,
            if is_unique_equipped {
                skill.skill_evolution_id.is_some()
            } else {
                false
            },
        );
    }

    for _ in &skill.ex_skill {
        skill_level += status_coefficient.ex_skill_power(level, rarity >= 5);
    }

    for _ in &skill.free_skill {
        skill_level += level as f64;
    }

    skill_level
}

macro_rules! rankup_bonus {
    ($type:ty, $rank:expr) => {
        nalgebra::vector![
            $rank as $type,
            $rank as $type,
            $rank as $type,
            $rank as $type,
            $rank as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type,
            1 as $type
        ]
    };

    ($rank:expr) => {
        nalgebra::vector![
            $rank,
            $rank,
            $rank,
            $rank,
            $rank,
            1f64,
            1f64,
            1f64,
            1f64,
            1f64,
            1f64,
            1f64,
            1f64,
            1f64,
            1f64,
            1f64,
            1f64
        ]
    };
}

const RANK_UP_BONUS: [UnitStatus<f64>; 33] = [
    rankup_bonus!(0f64),
    rankup_bonus!(1f64),
    rankup_bonus!(2f64),
    rankup_bonus!(3f64),
    rankup_bonus!(4f64),
    rankup_bonus!(5f64),
    rankup_bonus!(6f64),
    rankup_bonus!(7f64),
    rankup_bonus!(8f64),
    rankup_bonus!(9f64),
    rankup_bonus!(10f64),
    rankup_bonus!(11f64),
    rankup_bonus!(12f64),
    rankup_bonus!(13f64),
    rankup_bonus!(14f64),
    rankup_bonus!(15f64),
    rankup_bonus!(16f64),
    rankup_bonus!(17f64),
    rankup_bonus!(18f64),
    rankup_bonus!(19f64),
    rankup_bonus!(20f64),
    rankup_bonus!(21f64),
    rankup_bonus!(22f64),
    rankup_bonus!(23f64),
    rankup_bonus!(24f64),
    rankup_bonus!(25f64),
    rankup_bonus!(26f64),
    rankup_bonus!(27f64),
    rankup_bonus!(28f64),
    rankup_bonus!(29f64),
    rankup_bonus!(30f64),
    rankup_bonus!(31f64),
    rankup_bonus!(32f64),
];

pub fn rank_up_bonus(rank: i32) -> UnitStatus<f64> {
    let bonus = &RANK_UP_BONUS;
    if (rank as usize) < bonus.len() {
        bonus[rank as usize]
    } else {
        rankup_bonus!(f64, rank)
    }
}
