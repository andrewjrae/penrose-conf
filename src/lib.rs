#[macro_use]
extern crate penrose;

pub mod hooks;

pub const RATIO: f32 = 0.6;
pub const N_MAIN: u32 = 1;
pub const FLOAT_CLASS: &str = "floating";

pub const TERMINAL: &str = "alacritty";
pub const LAUNCHER: &str = "rofi -show run";
pub const BROWSER: &str = "google-chrome-stable";
pub const EDITOR: &str = "emacsclient -c -a emacs";
pub const START_SCRIPT: &str = "/home/ajrae/penrose-conf/src/scripts/autostart.sh";
