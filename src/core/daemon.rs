use crate::{cmd::xrandr::XrandrCmd, params::Params, screen_opts::ScreenOptions};
use std::{str::FromStr, thread, time};

pub struct MSDaemon;

impl MSDaemon {
    pub fn start() {
        loop {
            MSDaemon::detect_connected_monitors();
            let sleep_time = time::Duration::from_millis(Params::daemon_sleep_time_millis());
            thread::sleep(sleep_time);
        }
    }

    fn detect_connected_monitors() {
        ScreenOptions::from_str(&XrandrCmd::get_display_options());
    }
}
