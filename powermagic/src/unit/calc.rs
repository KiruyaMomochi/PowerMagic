use std::cell::RefCell;

use crate::unit::*;

/// Calculate unit power
pub struct UnitCalculator<'a> {
    pub(crate) cache: &'a UnitCache,
    pub(crate) state: UnitState,
}

pub struct MemorizedUnitCalculator<'a> {
    pub(crate) calculator: UnitCalculator<'a>,
    memo: RefCell<UnitMemo>,
    need_update: RefCell<UnitCalculatorNeedUpdate>,
}

/// Constructor
impl<'a> UnitCalculator<'a> {
    pub fn new(cache: &'a UnitCache) -> Self {
        Self {
            cache,
            state: cache.unit_state(),
        }
    }

    pub fn memorized(self) -> MemorizedUnitCalculator<'a> {
        MemorizedUnitCalculator {
            calculator: self,
            memo: RefCell::new(UnitMemo::default()),
            need_update: RefCell::new(UnitCalculatorNeedUpdate::default()),
        }
    }
}

pub trait StatusSetter
where
    Self: std::marker::Sized,
{
    fn set_level(self, level: i32) -> Self;
    fn set_skill_level(self, level: i32) -> Self;
    fn wear_unlock_rarity_6_equipment(self) -> Self;
    fn set_rarity(self, rarity: i32) -> Self;
    fn set_promotion(self, promotion: i32) -> Self;
    fn wear_all_equipments_0(self) -> Self;
    fn wear_all_equipments(self, level: i32) -> Self;
    fn unequip_all_equipments(self) -> Self;
    fn wear_unique_equipment(self, level: i32) -> Self;
    fn watch_all_stories(self) -> Self;
    fn watch_story(self, story_group_id: i64, watched_count: usize) -> Self;
    fn set_equipment(self, slot_id: usize, slot: EquipSlot) -> Self;
    fn set_equipments(self, equipments: Vec<EquipSlot>) -> Self;

    fn watch_stories(self, stories: &[(i64, usize)]) -> Self {
        stories
            .iter()
            .fold(self, |acc, (story_group_id, watched_count)| {
                acc.watch_story(*story_group_id, *watched_count)
            })
    }

    fn set_star(self, star: i32) -> Self {
        self.set_rarity(star)
    }

    fn set_rank(self, rank: i32) -> Self {
        self.set_promotion(rank)
    }

    fn set_all_level(self, level: i32) -> Self {
        self.set_level(level).set_skill_level(level)
    }
}

macro_rules! memorized_setter {
    ($fn:ident($( $arg:ident : $type:ty),*): $on:ident) => {
        fn $fn(mut self $(, $arg: $type)*) -> Self {
            self.need_update.borrow_mut().$on();
            self.calculator = self.calculator.$fn($($arg),*);
            self
        }
    };
}

impl StatusSetter for MemorizedUnitCalculator<'_> {
    memorized_setter!(set_level(level: i32): on_level_change);
    memorized_setter!(set_skill_level(level: i32): on_skill_change);
    memorized_setter!(wear_unlock_rarity_6_equipment(): on_rarity_6_change);
    memorized_setter!(set_rarity(rarity: i32): on_rarity_change);
    memorized_setter!(set_promotion(promotion: i32): on_promotion_change);
    memorized_setter!(wear_all_equipments_0(): on_rank_equip_change);
    memorized_setter!(wear_all_equipments(level: i32): on_rank_equip_change);
    memorized_setter!(unequip_all_equipments(): on_rank_equip_change);
    memorized_setter!(wear_unique_equipment(level: i32): on_unique_change);
    memorized_setter!(watch_all_stories(): on_story_change);
    memorized_setter!(watch_story(story_id: i64, watched_count: usize): on_story_change);
    memorized_setter!(set_equipment(slot_id: usize, slot: EquipSlot): on_rank_equip_change);
    memorized_setter!(set_equipments(equipments: Vec<EquipSlot>): on_rank_equip_change);
}

/// Setters
impl StatusSetter for UnitCalculator<'_> {
    fn set_level(mut self, level: i32) -> Self {
        self.state.level = level;

        self
    }

    fn set_skill_level(mut self, level: i32) -> Self {
        assert!(level <= self.state.level);
        self.state.skill.set_all_level(level);

        self
    }

    fn wear_unlock_rarity_6_equipment(mut self) -> Self {
        assert!(self.state.level != 6);
        assert!(self.state.unlock_rarity_6_slot.is_some());

        let mut slots = self.state.unlock_rarity_6_slot.as_mut().unwrap();
        slots.slot_1_level = 0;
        slots.slot_2_level = 0;
        slots.slot_3_level = 0;

        self
    }

    fn set_rarity(mut self, rarity: i32) -> Self {
        self.state.rarity = rarity;

        self
    }

    fn set_promotion(mut self, promotion: i32) -> Self {
        self.state.promotion = promotion;
        self.state.equip_slot = self.cache.promotion[promotion as usize - 1].equipments_to_slots();

        self
    }

    fn wear_all_equipments_0(mut self) -> Self {
        self.state.equip_slot.iter_mut().for_each(|slot| {
            slot.equip_0();
        });

        self
    }

    fn wear_all_equipments(mut self, level: i32) -> Self {
        self.state.equip_slot.iter_mut().for_each(|slot| {
            slot.equip(level);
        });

        self
    }

    fn wear_unique_equipment(mut self, level: i32) -> Self {
        assert!(self.cache.unique_equip.is_some());

        self.state.unique_equip_slot.iter_mut().for_each(|slot| {
            slot.equip(level);
        });

        self
    }

    fn unequip_all_equipments(mut self) -> Self {
        self.state.equip_slot.iter_mut().for_each(|slot| {
            slot.unequip();
        });

        self
    }

    fn watch_all_stories(mut self) -> Self {
        self.state.story.iter_mut().for_each(|(_, group)| {
            group.watched = group.total;
        });

        self
    }

    fn watch_story(mut self, story_id: i64, watched_count: usize) -> Self {
        let group = self.state.story.get_mut(&story_id).unwrap();
        group.watched = watched_count;

        self
    }

    fn set_equipment(mut self, slot_id: usize, slot: EquipSlot) -> Self {
        self.state.equip_slot[slot_id] = slot;

        self
    }

    fn set_equipments(mut self, equipments: Vec<EquipSlot>) -> Self {
        self.state.equip_slot = equipments;

        self
    }
}

macro_rules! memorized_getter_ng {
    ($(
        pub fn $fn:ident(
            &$self:ident,
            $($cache:ident
            $(, $state:ident
            $(, $arg:ident : $type:ty)*)?)?):
            $memo:ident -> $ret:ty
            $block:block
    )*) => {
        impl UnitCalculator<'_> {
            $(
                pub fn $fn(&$self $($($(,$arg: $type)*)?)?) -> $ret {
                    $(let $cache = &$self.cache;
                    $(let $state = &$self.state;)?)?

                    $block
                }
            )*
        }

        impl MemorizedUnitCalculator<'_> {
            $(
                pub fn $fn(&$self $($($(,$arg: $type)*)?)?) -> $ret {
                    $(let $cache = &$self.calculator.cache;
                    $(let $state = &$self.calculator.state;)?)?

                    // $cache $state

                    if $self.need_update.borrow().$memo {
                        let memo = $block;

                        $self.memo.borrow_mut().$memo = memo;
                        $self.need_update.borrow_mut().$memo = false;
                    }
                    $self.memo.borrow().$memo
                }
            )*
        }
    }
}

memorized_getter_ng! {
    pub fn rarity_param(&self, cache, state): rarity -> UnitStatus<f64> {
        let rarity = &cache.rarity[state.rarity as usize - 1];

        rarity.status
            + rarity.growth * (state.level as f64)
            + rarity.growth.component_mul(&rank_up_bonus(state.promotion))
    }

    pub fn skill_power(&self, cache, state): skill -> f64 {
        let status_coefficient = &cache.status_coefficient;

        let skill = &state.skill;
        let rarity = state.rarity;
        let is_unique_equipped = cache.unique_equip.is_some() && state.unique_equip_slot[0].is_equipped();

        let mut power = 0f64;

        for skill in &skill.union_burst {
            power += status_coefficient.union_burst_power(skill.skill_level, rarity >= 6);
        }

        for skill in &skill.main_skill {
            power += status_coefficient.main_skill_power(
                skill.skill_level,
                if is_unique_equipped {
                    skill.skill_evolution_id.is_some()
                } else {
                    false
                },
            );
        }

        for skill in &skill.ex_skill {
            power += status_coefficient.ex_skill_power(skill.skill_level, rarity >= 5);
        }

        for skill in &skill.free_skill {
            power += skill.skill_level as f64;
        }

        power
    }

    pub fn promotion_param(&self, cache, state): promotion -> UnitStatus<f64> {
        let promotion = &cache.promotion[state.promotion as usize - 1];

        if let Some(bonus) = promotion.bonus {
            promotion.status.unwrap() + bonus
        } else {
            promotion.status.unwrap()
        }
    }

    pub fn base_param(&self, cache, state): base -> UnitStatus<i64> {
        (self.rarity_param() + self.promotion_param()).map(|x| x.cy_round())
    }

    pub fn rank_equip_param(&self, cache, state): rank_equip -> UnitStatus<f64> {
        let mut status = UnitStatus::zeros();
        state
            .equip_slot
            .iter()
            .zip(
                cache.promotion[state.promotion as usize - 1]
                    .equipments
                    .iter(),
            )
            .for_each(|(equip, promotion)| {
                if promotion.is_none() {
                    assert!(
                        equip.is_none(),
                        "equip slot is not empty but promotion is None"
                    );
                    return;
                }

                let promotion = promotion.as_ref().unwrap();
                if let EquipSlot::Equipped {
                    enhancement_level, ..
                } = equip
                {
                    status += promotion.param(*enhancement_level)
                }
            });

        status
    }

    pub fn unique_equip_param(&self, cache, state): unique -> UnitStatus<f64> {
        assert_eq!(state.unique_equip_slot.len(), 1);

        match state.unique_equip_slot[0] {
            EquipSlot::None => panic!("unique slot is not set"),
            EquipSlot::Equipped {
                enhancement_level, ..
            } => cache.unique_equip_param(enhancement_level),
            _ => UnitStatus::zeros(),
        }
    }

    pub fn rarity_6_param(&self, cache, state): rarity_6 -> UnitStatus<i64> {
        assert!(state.unlock_rarity_6_slot.is_some());

        cache
            .unlock_rarity_6_param(state.unlock_rarity_6_slot.as_ref().unwrap().slot_level())
    }

    pub fn story_param(&self, cache, state): story -> UnitStatus<i64> {
        let mut status = UnitStatus::zeros();
        state.story.iter().for_each(|(story_id, group)| {
            cache
                .story
                .get(story_id)
                .unwrap()
                .param_append(group.watched, &mut status)
        });

        status
    }

    pub fn equip_param(&self, cache, state): equip -> UnitStatus<i64> {
        let mut rank = self.rank_equip_param();
        if cache.unique_equip.is_some() {
            rank += self.unique_equip_param();
        }

        let mut rank: UnitStatus<i64> = rank.map(|x| x.cy_round());

        if cache.unlock_rarity_6.is_some() {
            rank += self.rarity_6_param();
        }

        rank
    }

    pub fn param(&self, cache, state): status -> UnitStatus<i64> {
        self.base_param() + self.equip_param() + self.story_param()
    }

    pub fn power(&self, cache, state): power -> f64 {
        self.skill_power() * cache.status_coefficient.skill_lv_coefficient
            + nalgebra::convert::<_, UnitStatus<f64>>(self.param())
                .dot(&cache.status_coefficient_cache)
    }
}

impl UnitCalculator<'_> {}

/// State of a unit that have changed
#[derive(Debug)]
pub struct UnitChangedState {
    /// Number of stars
    pub rarity: bool,
    /// Lv
    pub level: bool,
    /// Rank
    pub promotion: bool,
    /// Skill,
    pub skill: bool,
    /// Equipment slots
    pub equip_slot: bool,
    /// Unique equipment slots
    pub unique_equip_slot: bool,
    /// Unlock rarity 6 slot
    pub unlock_rarity_6_slot: bool,
    /// Watched stories
    pub story: bool,
}

impl Default for UnitChangedState {
    fn default() -> Self {
        Self {
            rarity: true,
            level: true,
            promotion: true,
            skill: true,
            equip_slot: true,
            unique_equip_slot: true,
            unlock_rarity_6_slot: true,
            story: true,
        }
    }
}

pub struct UnitMemo {
    pub power: f64,
    pub skill: f64,
    pub status: UnitStatus<i64>,
    pub base: UnitStatus<i64>,
    pub rarity: UnitStatus<f64>,
    pub promotion: UnitStatus<f64>,
    pub equip: UnitStatus<i64>,
    pub rank_equip: UnitStatus<f64>,
    pub unique: UnitStatus<f64>,
    pub rarity_6: UnitStatus<i64>,
    pub story: UnitStatus<i64>,
}

impl Default for UnitMemo {
    fn default() -> Self {
        Self {
            power: 0.0,
            skill: 0.0,
            status: UnitStatus::zeros(),
            base: UnitStatus::zeros(),
            rarity: UnitStatus::zeros(),
            promotion: UnitStatus::zeros(),
            equip: UnitStatus::zeros(),
            rank_equip: UnitStatus::zeros(),
            unique: UnitStatus::zeros(),
            rarity_6: UnitStatus::zeros(),
            story: UnitStatus::zeros(),
        }
    }
}

pub struct UnitCalculatorNeedUpdate {
    pub power: bool,
    pub skill: bool,
    pub status: bool,
    pub base: bool,
    pub rarity: bool,
    pub promotion: bool,
    pub equip: bool,
    pub rank_equip: bool,
    pub unique: bool,
    pub rarity_6: bool,
    pub story: bool,
}

impl Default for UnitCalculatorNeedUpdate {
    fn default() -> Self {
        Self {
            power: true,
            skill: true,
            status: true,
            base: true,
            rarity: true,
            promotion: true,
            equip: true,
            rank_equip: true,
            unique: true,
            rarity_6: true,
            story: true,
        }
    }
}

macro_rules! set_dependent {
    ($setter:ident($field:ident) $(-> $($parent_setter:ident),+)?) => {
        fn $setter(&mut self) -> &Self {
            if !self.$field {
                self.$field = true;
                $($(self.$parent_setter();)+)?
            }

            self
        }
    };
}

macro_rules! set_change {
    ($change:ident $(-> $($setter:ident),+)?) => {
        fn $change(&mut self) -> &Self {
            $($(self.$setter();)+)?
            self
        }
    };
}

impl UnitCalculatorNeedUpdate {
    set_dependent!(set_power(power));
    set_dependent!(set_skill(skill) -> set_power);
    set_dependent!(set_status(status) -> set_skill);
    set_dependent!(set_base(base) -> set_status);
    set_dependent!(set_equip(equip) -> set_status);
    set_dependent!(set_story(story) -> set_status);
    set_dependent!(set_rarity(rarity) -> set_base);
    set_dependent!(set_promotion(promotion) -> set_base);
    set_dependent!(set_rank_equip(rank_equip) -> set_equip);
    set_dependent!(set_unique(unique) -> set_equip);
    set_dependent!(set_rarity_6(rarity_6) -> set_equip);

    pub fn reset(&mut self) -> &Self {
        self.power = false;
        self.skill = false;
        self.status = false;
        self.base = false;
        self.rarity = false;
        self.promotion = false;
        self.equip = false;
        self.rank_equip = false;
        self.unique = false;
        self.rarity_6 = false;
        self.story = false;

        self
    }

    set_change!(on_promotion_change -> set_promotion, set_rank_equip, set_rarity);
    set_change!(on_level_change -> set_rarity);
    set_change!(on_story_change -> set_story);
    set_change!(on_skill_change -> set_skill);
    set_change!(on_rarity_change -> set_rarity);
    set_change!(on_rank_equip_change -> set_rank_equip);
    set_change!(on_unique_change -> set_unique);
    set_change!(on_rarity_6_change -> set_rarity_6);
}
