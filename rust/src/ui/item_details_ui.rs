use godot::classes::*;
use godot::prelude::*;

use crate::global_signals::GlobalSignals;
use crate::item::item::Item;
use crate::item::item_definition::ItemDefinition;
use crate::stats::modifier::modifier::StatModifier;
use crate::stats::modifier::r#type::ModifierType;
use crate::stats::stat::Stat;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ItemDetailsUi {
    base: Base<Node>,
    #[export]
    panel: OnEditor<Gd<PanelContainer>>,
    #[export]
    title: OnEditor<Gd<Label>>,
    #[export]
    icon: OnEditor<Gd<TextureRect>>,
    #[export]
    modifiers_text_container: OnEditor<Gd<Container>>,
    #[export]
    close_button: OnEditor<Gd<Button>>,
}

#[godot_api]
impl INode for ItemDetailsUi {
    fn ready(&mut self) {
        self.setup();
    }
}

#[godot_api]
impl ItemDetailsUi {
    fn setup(&mut self) {
        self.close_button.signals().pressed().connect_other(self, Self::on_close);
        GlobalSignals::get_singleton().signals().inventory_ui_item_clicked().connect_other(self, |self_ref, item| {
            self_ref.display_item(item);
        });
    }

    pub(crate) fn display_item(&mut self, item: Gd<Item>) {
        self.set_title_text(&item);
        self.set_icon_texture(&item);
        self.set_modifiers_text(&item);
        self.open();
    }

    fn set_title_text(&mut self, item: &Gd<Item>) {
        let title_text = ItemDefinition::from_gstring(item.bind().get_name()).get_display_name();
        self.title.set_text(title_text);
    }

    fn set_icon_texture(&mut self, item: &Gd<Item>) {
        let icon_texture = item.bind().get_icon();
        if icon_texture.is_none() {
            return;
        }
        self.icon.set_texture(&icon_texture.unwrap());
    }

    fn set_modifiers_text(&mut self, item: &Gd<Item>) {
        let children = self.modifiers_text_container.get_children();
        children.iter_shared().for_each(|child| self.modifiers_text_container.remove_child(&child));
        
        let modifiers = item.bind().get_all_modifiers();
        modifiers.iter_shared().for_each(|modifier| self.create_label_for_modifier(modifier));
    }

    fn create_label_for_modifier(&mut self, modifier: Gd<StatModifier>) {
        let mod_type = ModifierType::from_gstring(modifier.bind().get_mod_type());
        let mod_value = modifier.bind().get_value();
        let mod_stat = Stat::from_gstring(modifier.bind().get_stat()).get_display_name();
        let mod_text = match mod_type {
            ModifierType::FLAT => format!("+ {} {}", mod_value, mod_stat),
            ModifierType::PERCENTAGE => format!("* {} {}", mod_value, mod_stat),
            _ => format!("?"),
        };

        let mut new_label = Label::new_alloc();
        new_label.set_text(&mod_text);
        self.modifiers_text_container.add_child(&new_label);
    }

    fn open(&mut self) {
        self.panel.set_visible(true);
    }

    fn on_close(&mut self) {
        self.panel.set_visible(false);
    }
}
