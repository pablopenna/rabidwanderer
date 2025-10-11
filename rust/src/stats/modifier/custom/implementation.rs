use crate::stats::modifier::custom::definition::CustomModifier;

pub(crate) fn get_implementation_for_custom_modifier(custom_mod: CustomModifier) -> fn(i16) -> i16 {
    match custom_mod {
        CustomModifier::ONE => one_modifier,
    }
}

// Sets the stat value to 1 no matter the original value
fn one_modifier(stat: i16) -> i16 {
    (stat * -1) + 1
}