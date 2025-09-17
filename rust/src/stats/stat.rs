use godot::prelude::*;

#[derive(GodotConvert, Var, Export, PartialEq, Default)]
#[godot(via = GString)]
pub(crate) enum Stat {
    MaxHp,
    #[default]
    Attack,
    Speed,
}

impl Stat {
    pub(crate) fn get_display_name(&self) -> &'static str {
        match self {
            Stat::MaxHp => "Max HP",
            Stat::Attack => "Attack",
            Stat::Speed => "Speed",
        }
    }

    pub(crate) fn from_gstring(stat: GString) -> Stat {
        Stat::from_variant(&stat.to_variant())
    }
}
