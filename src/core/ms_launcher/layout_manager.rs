// TODO: rewrite this code into smaller parts, rewrite partial code parts in more convenient way
// TODO: create layout_manager.rs mod file instead of mod.rs file and put here pub mod layout_creator
// and pub use layout_manager::layout_creator::LayoutCreator to be able to use crate::dmenu_interface::layout_manager::LayoutCreator
use super::layout_creator::LayoutCreator;
use crate::{core::utils::layout_config::LayoutConfig, ui::dmenu_ui::DmenuUI};

#[derive(Default)]
pub struct LayoutManager {
    pub layouts_config: LayoutConfig,
}

impl LayoutManager {
    pub fn user_layouts_names(&self) -> Vec<String> {
        self.layouts_config.user_layouts.names()
    }
    pub fn auto_detect_layout(&self) {
        unimplemented!();
    }

    pub fn create_new_layout(&mut self) {
        let mut layout_creator = LayoutCreator::default();
        layout_creator.create_layout(&self.layouts_config.user_layouts);
        if !layout_creator.is_empty() {
            self.layouts_config
                .add_or_overwrite_if_exists(&layout_creator.final_layout);
        }
    }

    pub fn disconnect_all_monitors(&mut self) {
        // here the CurrentLayout is needed
        // also, need to recognize an in-build monitor of the setup
        // alternatively, it's needed to keep only primary monitor enabled
        if self.layouts_config.current_layout.monitors.is_empty() {
            unimplemented!();
        }
    }

    pub fn remove_layout(&mut self) {
        if self.layouts_config.user_layouts.is_empty() {
            return DmenuUI::exec_no_layouts_found();
        }
        let layout_name = DmenuUI::exec_layout_to_remove(&self.layouts_config.user_layouts.names());
        match self
            .layouts_config
            .user_layouts
            .find_layout_pos(&layout_name)
        {
            Ok(pos) if DmenuUI::confirmed() => self.layouts_config.user_layouts.remove_layout(pos),
            Ok(_) => {} // skip if not confirmed
            Err(err) => DmenuUI::exec_err(err, &layout_name),
        };
    }

    pub fn apply_layout(&self, layout_name: &str) {
        match self.layouts_config.user_layouts.get_layout_by(layout_name) {
            Ok(layout) if DmenuUI::confirmed() => layout.apply(),
            Ok(_) => {} // skip if not confirmed
            Err(err) => DmenuUI::exec_err(err, layout_name),
        };
    }
}
