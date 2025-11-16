use godot::classes::*;
use godot::prelude::*;

use rand::Rng;

use crate::entity::modules::item::inventory::InventoryModule;
use crate::stats::base::BaseStats;
use crate::stats::modifier::custom::definition::CustomModifier;
use crate::stats::modifier::custom::implementation::get_implementation_for_custom_modifier;
use crate::stats::modifier::modifier::StatModifier;
use crate::stats::modifier::r#type::ModifierType;
use crate::stats::stat::Stat;

const DAMAGE_VARIATION_RATIO: f32 = 0.1;

// This class is instantiated by the StatsModule
#[derive(GodotClass)]
#[class(base=Resource)]
pub(crate) struct RealStats {
    base: Base<Resource>,
    base_stats: Gd<BaseStats>,
    inventory: Option<Gd<InventoryModule>>,
    current_hp: u16,
}

#[godot_api]
impl IResource for RealStats {
    // This is required but its values should be set by StatsModules via the new() method below
    fn init(base: Base<Resource>) -> Self {
        Self {
            base,
            base_stats: BaseStats::new_gd(),
            inventory: None,
            current_hp: 0,
        }
    }
}

#[godot_api]
impl RealStats {
    #[signal]
    pub(crate) fn no_hp_left();
    #[signal]
    pub(crate) fn hp_changed(new_hp: u16);

    pub(crate) fn new(
        base_stats: Gd<BaseStats>,
        inventory: Option<Gd<InventoryModule>>,
    ) -> Gd<Self> {
        let mut new_stats = Self::new_gd();
        new_stats.bind_mut().current_hp = base_stats.bind().get_max_hp();
        new_stats.bind_mut().base_stats = base_stats;
        new_stats.bind_mut().inventory = inventory;

        new_stats
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    pub(crate) fn get_current_hp(&self) -> u16 {
        self.current_hp
    }

    pub(crate) fn set_current_hp(&mut self, hp: u16) {
        self.current_hp = hp;
        self.signals().hp_changed().emit(hp);
        if self.current_hp <= 0 {
            self.signals().no_hp_left().emit();
        }
    }

    pub(crate) fn get_stat(&self, stat: Stat) -> i16 {
        let base_value = self.get_base_stat(&stat) as i16;
        let modifier = self.get_modifier_from_items(&stat);
        let stat_value = base_value + modifier;
        if stat == Stat::Attack {
            let randomization = Self::get_randomization_for(stat_value, DAMAGE_VARIATION_RATIO);
            godot_print!(
                "[DMG CALC] base: {}, modifier: {}, rand: {}",
                base_value,
                modifier,
                randomization
            );
            return stat_value + randomization;
        }
        stat_value
    }

    fn get_base_stat(&self, stat: &Stat) -> u16 {
        match stat {
            Stat::Attack => self.base_stats.bind().get_attack(),
            Stat::Speed => self.base_stats.bind().get_speed(),
            Stat::MaxHp => self.base_stats.bind().get_max_hp(),
        }
    }

    // Retuns i16 instead of u16 because stats modifiers can be negative, but base stats are always equal or higher than 0
    fn get_modifier_from_items(&self, base_stat: &Stat) -> i16 {
        if self.inventory.is_none() {
            return 0;
        }

        let items = self.inventory.clone().unwrap().bind().get_items();
        let modifiers_from_items: Array<Gd<StatModifier>> = items
            .iter_shared()
            .flat_map(|item| {
                let modifiers = item.bind().get_modifiers_for_stat(&base_stat);
                modifiers.iter_shared().collect::<Vec<_>>()
            })
            .collect();

        let mut modified_value: i16 = 0;
        modifiers_from_items.iter_shared().for_each(|r#mod| {
            let r#type = ModifierType::from_gstring(r#mod.bind().get_mod_type());
            match r#type {
                ModifierType::FLAT => {
                    modified_value = modified_value + r#mod.bind().get_value().round() as i16;
                }
                ModifierType::PERCENTAGE => {
                    modified_value +=
                        (modified_value as f32 * r#mod.bind().get_value()).round() as i16;
                }
                ModifierType::CUSTOM => {
                    let custom_modifier =
                        CustomModifier::from_gstring(r#mod.bind().get_custom_implementation());
                    let custom_modifier_logic =
                        get_implementation_for_custom_modifier(custom_modifier);
                    modified_value = modified_value + custom_modifier_logic(modified_value)
                }
            }
        });

        modified_value
    }

    fn get_randomization_for(value: i16, ratio: f32) -> i16 {
        let mut rng = rand::rng();
        let variation: f32 = value as f32 * ratio;
        let randomized_value = rng.random_range(-variation..variation);

        randomized_value.round() as i16
    }
}
