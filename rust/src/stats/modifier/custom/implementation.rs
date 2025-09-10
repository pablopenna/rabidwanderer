use crate::stats::modifier::custom::definition::CustomModifier;

pub(crate) fn get_implementation_for_custom_modifier(custom_mod: CustomModifier) -> fn(u16) -> i16 {
    match custom_mod {
        CustomModifier::ONE => one_modifier,
    }
}

fn one_modifier(stat: u16) -> i16 {
    (stat as i16 * -1) + 1
}