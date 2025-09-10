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

    pub(crate) fn new(base_stats: Gd<BaseStats>, inventory: Option<Gd<InventoryModule>>) -> Gd<Self> {
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

    pub(crate) fn get_max_hp(&self) -> u16 {
        self.base_stats.bind().get_max_hp()
    }

    pub(crate) fn get_attack(&self) -> u16 {
        self.base_stats.bind().get_attack()
    }

    pub(crate) fn calculate_attack_damage(&self) -> u16 {
        let base_damage = self.get_attack();
        let damage = if self.inventory.is_none() {
            base_damage
        } else {
            self.get_attack_from_items(base_damage)
        };
        
        let mut rng = rand::rng();
        let variation: f32 = damage as f32 * DAMAGE_VARIATION_RATIO;
        let randomized_damage = damage as f32 + rng.random_range(-variation..variation);

        randomized_damage.round() as u16
    }

    fn get_attack_from_items(&self, base_damage: u16) -> u16 {
        let items = self.inventory.clone().unwrap().bind().get_items();
        let attack_modifiers_from_items: Array<Gd<StatModifier>> = items.iter_shared().flat_map(
            |item| {
                let modifiers = item.bind().get_modifiers_for_stat(Stat::Attack);
                modifiers.iter_shared().collect::<Vec<_>>()
            }
        ).collect();

        let mut mod_damage = base_damage;
        attack_modifiers_from_items.iter_shared().for_each(|r#mod| {
            let r#type = ModifierType::from_gstring(r#mod.bind().get_mod_type());
            match r#type {
                ModifierType::FLAT => {
                    mod_damage = (mod_damage as i16 + r#mod.bind().get_value().round() as i16) as u16;
                },
                ModifierType::PERCENTAGE => {
                    mod_damage += ( mod_damage as f32 * r#mod.bind().get_value() ).round() as u16;
                },
                ModifierType::CUSTOM => {
                    let custom_modifier = CustomModifier::from_gstring(r#mod.bind().get_custom_implementation());
                    let custom_modifier_logic = get_implementation_for_custom_modifier(custom_modifier);
                    mod_damage = (mod_damage as i16 + custom_modifier_logic(mod_damage)) as u16
                },
            }
        });

        mod_damage
    }

    pub(crate) fn get_speed(&self) -> u16 {
        self.base_stats.bind().get_speed()
    }
}