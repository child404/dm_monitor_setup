use crate::{
    cmd::{
        dmenu::{DmenuCmd, DmenuDefaults},
        xrandr::XrandrCmd,
    },
    monitor_layout::{Monitor, MonitorLayout, MonitorLayouts, MonitorPosition},
    screen_opts::ScreenOptions,
};

#[derive(Default)]
pub struct LayoutCreator {
    _is_primary_selected: bool,
    _selected_opts: ScreenOptions,
    _conn_opt: String,
    _current_monitor: Monitor,
    pub final_layout: MonitorLayout,
}

impl LayoutCreator {
    pub fn is_empty(&self) -> bool {
        self.final_layout.monitors.is_empty()
    }
    fn _read_res(&mut self) {
        if let Ok(screen_res) = DmenuCmd::new(
            &self._selected_opts.resolutions(),
            format!("Which resolution for {}? ", self._current_monitor.name),
        )
        .exec()
        .parse()
        {
            self._current_monitor.res = screen_res;
        } else {
            self._read_res();
        }
    }

    fn _read_rate(&mut self) {
        if let Ok(rate) = DmenuCmd::new(
            &self._selected_opts.rates(),
            format!("Which rate for {}? ", self._current_monitor.name),
        )
        .exec()
        .parse()
        {
            self._current_monitor.rate = rate;
        } else {
            self._read_rate();
        }
    }

    fn _read_res_and_rate(&mut self) {
        self._read_res();
        self._read_rate();
    }

    fn _read_conn_opt(&mut self) {
        let conn_opts = ["Connect".to_string(), "Duplicate".to_string()];
        match DmenuCmd::new(&conn_opts, "Connect or duplicate monitor? ".to_string()).exec() {
            conn_opt if conn_opts.contains(&conn_opt) => {
                self._conn_opt = conn_opt;
            }
            _ => self._read_conn_opt(),
        }
    }

    fn _read_monitor_name(&mut self, monitor_names: &[String]) {
        match DmenuCmd::new(
            monitor_names,
            format!(
                "Which monitor to add ({})? ",
                &self._conn_opt.to_lowercase()
            ),
        )
        .exec()
        {
            monitor_name if monitor_names.contains(&monitor_name) => {
                self._current_monitor.name = monitor_name;
            }
            _ => self._read_monitor_name(monitor_names),
        }
    }

    fn _read_monitor_pos(&mut self) {
        if self.final_layout.monitors.is_empty() {
            return;
        }
        // TODO: add here check for incorrect pos or monitor name
        match DmenuDefaults::exec_position(&self._current_monitor.name).as_str() {
            "Skip" => {}
            pos => {
                let related_monitor = DmenuCmd::new(
                    &self.final_layout.monitor_names(),
                    format!(
                        "Place {} {} which monitor? ",
                        &self._current_monitor.name, &pos
                    ),
                )
                .exec();
                self._current_monitor.pos = MonitorPosition::new(pos, &related_monitor);
            }
        };
    }

    fn _read_primary(&mut self) {
        if !self._is_primary_selected {
            self._current_monitor.is_primary = DmenuDefaults::exec_is_primary();
            self._is_primary_selected = self._current_monitor.is_primary;
        }
    }

    fn _read_layout_name(&mut self, user_layouts: &MonitorLayouts) {
        self.final_layout.name = DmenuCmd::new(
            &user_layouts.names(),
            String::from("Choose the name for your layout: "),
        )
        .exec();
        if user_layouts.find_layout_pos(&self.final_layout.name).ok() != None
            && !DmenuDefaults::exec_overwrite_layout(&self.final_layout.name)
        {
            self._read_layout_name(user_layouts);
        }
    }

    pub fn create_layout(&mut self, user_layouts: &MonitorLayouts) {
        if DmenuDefaults::exec_is_inherit_layout() {
            unimplemented!();
        }
        let mut screen_to_opts = XrandrCmd::get_display_options().unwrap_or_default();
        loop {
            self._read_conn_opt();
            self._read_monitor_name(&screen_to_opts.keys().cloned().collect::<Vec<String>>());
            self._selected_opts = screen_to_opts[&self._current_monitor.name].clone();
            self._read_res_and_rate();
            self._read_monitor_pos();
            self._read_primary();

            if !DmenuDefaults::confirmed() {
                continue;
            }

            screen_to_opts.remove(&self._current_monitor.name);
            self._current_monitor.is_auto = true;
            self.final_layout
                .monitors
                .push(self._current_monitor.clone());

            if screen_to_opts.is_empty() || !DmenuDefaults::exec_continue() {
                break;
            }
        }
        if self.is_empty() {
            return;
        }

        self._read_layout_name(user_layouts);
    }
}
