#![allow(unused_imports, unused_variables)]
mod config;
mod dmenu_cmd;
mod dmenu_ms;
mod monitor_setup;
use std::env;
use std::str;

fn main() {
    match env::args().nth(1) {
        Some(arg) => match arg.as_str() {
            "--daemon" | "-d" => dmenu_ms::run_daemon(),
            "--help" | "-h" => dmenu_ms::spawn_help(),
            _ => println!("Undefined arg: {arg}"),
        },
        None => dmenu_ms::launch_ms(),
    }
}
