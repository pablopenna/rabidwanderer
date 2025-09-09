use crate::stats::modifier::custom::definition::CustomModifier;

fn get_implementation_for_custom_modifier(custom_mod: CustomModifier) -> fn(u16) -> u16 {
    match custom_mod {
        CustomModifier::ONE => one_modifier,
    }
}

fn one_modifier(_stat: u16) -> u16 {
    1
}