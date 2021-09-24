#[macro_use]
extern crate penrose;

use penrose::core::layout::LayoutConf;

pub mod hooks;

#[macro_export]
macro_rules! layout {
    { $name:expr, $func:expr } => {
        penrose::core::layout::Layout::new($name,
                                           penrose::core::layout::LayoutConf::default(),
                                           $func, penrose_ajrae::N_MAIN, penrose_ajrae::RATIO)
    };
    { $name:expr, $conf:expr, $func:expr } => {
        penrose::core::layout::Layout::new($name, $conf, $func, penrose_ajrae::N_MAIN, penrose_ajrae::RATIO)
    };
}

// use macro so it can be used in concat
#[macro_export]
macro_rules! TERMINAL_MACRO {
    () => { "alacritty" }
}

#[macro_export]
macro_rules! run_in_terminal {
    { $cmd:tt } => {
        Box::new(move |_: &mut penrose::core::manager::WindowManager<_>| {
            penrose::core::helpers::spawn(concat!(TERMINAL_MACRO!(), " -e ", $cmd))
        }) as penrose::core::bindings::KeyEventHandler<_>
    };
}

pub const FOLLOW_FOCUS_CONF: LayoutConf = LayoutConf {
    floating: false,
    gapless: true,
    follow_focus: true,
    allow_wrapping: true,
};

pub const RATIO: f32 = 0.5;
pub const N_MAIN: u32 = 1;
pub const FLOAT_CLASS: &str = "floating";

pub const TERMINAL: &str = TERMINAL_MACRO!();
pub const LAUNCHER: &str = "rofi -show run";
// pub const BROWSER: &str = "google-chrome-stable";
pub const BROWSER: &str = "brave";
pub const EDITOR: &str = "emacsclient -c -a emacs";
pub const START_SCRIPT: &str = "/home/ajrae/penrose-conf/src/scripts/autostart.sh";
