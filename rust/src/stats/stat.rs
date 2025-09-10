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
    pub(crate) fn from_gstring(stat: GString) -> Stat {
        Stat::from_variant(&stat.to_variant())
    }
}
