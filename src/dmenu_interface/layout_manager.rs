// TODO: rewrite this code into smaller parts, rewrite partial code parts in more convenient way
// TODO: create layout_manager.rs mod file instead of mod.rs file and put here pub mod layout_creator
// and pub use layout_manager::layout_creator::LayoutCreator to be able to use crate::dmenu_interface::layout_manager::LayoutCreator
use super::layout_creator::LayoutCreator;
use crate::cmd::dmenu::DmenuDefaults;
use crate::monitor_layout::MonitorLayouts;

pub fn auto_detect_layout() {
    unimplemented!();
}

pub fn create_new_layout(user_layouts: &MonitorLayouts) {
    let layout_creator = LayoutCreator::default();
}

pub fn disconnect_all_monitors() {
    unimplemented!();
}

pub fn remove_layout(user_layouts: &mut MonitorLayouts) {
    if user_layouts.is_empty() {
        return DmenuDefaults::exec_no_layouts_found();
    }
    let layout_name = DmenuDefaults::exec_layout_to_remove(&user_layouts.names());
    match user_layouts.find_layout(&layout_name) {
        Ok(pos) if DmenuDefaults::confirmed() => user_layouts.remove_layout(pos),
        Err(err) => DmenuDefaults::exec_err(err, &layout_name),
        _ => {}
    };
}

pub fn apply_layout(user_layouts: &MonitorLayouts, layout_name: &str) {
    match user_layouts.get_layout_by(layout_name) {
        Ok(layout) if DmenuDefaults::confirmed() => layout.apply(),
        Err(err) => DmenuDefaults::exec_err(err, layout_name),
        _ => {}
    };
}
