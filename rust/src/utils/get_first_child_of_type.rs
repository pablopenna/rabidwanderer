use godot::prelude::*;

// https://doc.rust-lang.org/book/ch10-01-syntax.html
pub(crate) fn get_first_child_of_type<T>(parent: &Node) -> Option<Gd<T>>
where
    T: GodotClass + Inherits<godot::prelude::Node>,
{
    for i in 0..parent.get_child_count() {
        let child = parent.get_child(i);
        if child.is_none() {
            continue;
        }
        let child = child.unwrap();
        let child = child.try_cast::<T>();
        if child.is_ok() {
            return child.ok();
        }
    }
    None
}
