mod spell_handling;

pub use crate::spell_handling::spell_views;

pub fn spell_workbench() {
    spell_views::print_craftable_spells();
}