use crate::cmd::{
    dmenu::{DmenuCmd, DmenuDefaults},
    xrandr::XrandrCmd,
};
use crate::layouts_config::LayoutsConfig;
use crate::monitor_layout::{
    Monitor, MonitorDuplicated, MonitorLayout, MonitorLayouts, MonitorPosition,
};

#[derive(Default)]
pub struct LayoutCreator {
    final_layout: MonitorLayout,
}

impl LayoutCreator {
    pub fn create_layout(&mut self) {
        if DmenuDefaults::exec_inherit_layout() == "Yes" {
            unimplemented!();
        }
        let mut screen_to_opts = XrandrCmd::get_display_options().unwrap_or_default();
        let mut is_continue = true;
        while is_continue && !screen_to_opts.is_empty() {
            let conn_opt = DmenuCmd::new(
                &["Connect".to_string(), "Duplicate".to_string()],
                "Connect or duplicate monitor? ".to_string(),
            )
            .exec();

            let selected_monitor_name = DmenuCmd::new(
                &screen_to_opts.keys().cloned().collect::<Vec<String>>(),
                format!("Which monitor to add ({})? ", &conn_opt.to_lowercase()),
            )
            .exec();

            let selected_monitor = &screen_to_opts[&selected_monitor_name];
            let selected_res = DmenuCmd::new(
                &selected_monitor.resolutions(),
                format!("Which resolution for {}? ", selected_monitor_name),
            )
            .exec();
            let selected_rate = DmenuCmd::new(
                &selected_monitor.rates(),
                format!("Which rate for {}? ", selected_monitor_name),
            )
            .exec();

            let mut monitor_position = MonitorPosition::default();
            if !self.final_layout.monitors.is_empty() {
                match DmenuDefaults::exec_position(&selected_monitor_name).as_str() {
                    "Skip" => {}
                    pos => {
                        let related_monitor = DmenuCmd::new(
                            &self.final_layout.monitor_names(),
                            format!("Place {} {} which monitor? ", &selected_monitor_name, &pos),
                        )
                        .exec();
                        monitor_position = MonitorPosition::related(pos, &related_monitor);
                    }
                }
            }

            if !DmenuDefaults::confirmed() {
                continue;
            }
            screen_to_opts.remove(&selected_monitor_name);
            if let [height_px, width_px] = &selected_res
                .split('x')
                .map(|x| x.parse::<u16>().expect("Parsed monitor resolution"))
                .collect::<Vec<u16>>()[..]
            {
                self.final_layout.monitors.push(Monitor {
                    name: selected_monitor_name,
                    rate: selected_rate
                        .parse::<f32>()
                        .unwrap_or_else(|_| panic!("Parsed monitor rate {}", &selected_rate)),
                    height_px: *height_px,
                    width_px: *width_px,
                    pos: monitor_position,
                    is_auto: true,
                    is_primary: false,
                    dupl: MonitorDuplicated::default(),
                })
            }
            is_continue = DmenuDefaults::exec_continue();
        }
        monitor_layout.name =
            DmenuCmd::new(&[], String::from("Choose the name for your layout: ")).exec();

        LayoutsConfig::add(MonitorLayouts {
            layouts: vec![monitor_layout],
        });
    }

    pub fn remove_layout(&self) {}
}
