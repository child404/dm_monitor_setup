use crate::{core::handlers::xrandr::Xrandr, defaults};
use std::{thread, time};

pub struct MSDaemon;

impl MSDaemon {
    pub fn start() {
        loop {
            MSDaemon::detect_connected_monitors();
            let sleep_time = time::Duration::from_millis(defaults::DAEMON_SLEEP_TIME_MILLIS);
            thread::sleep(sleep_time);
        }
    }

    fn detect_connected_monitors() {
        if let Some(opts) = Xrandr::get_display_modes() {
            let monitor_opts = opts;
        }
    }
}
