#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        // bindings::KeyEventHandler,
        hooks::Hook,
        config::Config,
        helpers::{
            index_selectors,
            spawn,
        },
        xconnection::XConn,
        // manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector, WindowManager,
};

use simplelog::{LevelFilter, SimpleLogger};


// Replace these with your preferred terminal and program launcher
const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "rofi -show run";
const BROWSER: &str = "google-chrome-stable";
const EDITOR: &str = "emacsclient -c -a emacs";
const START_SCRIPT: &str = "./scripts/autostart.sh";

// Start hook to run all the important stuff like picom etc
struct StartHook {}

impl StartHook {
    pub fn new() -> Box<Self> {
        Box::new(Self{})
    }
}

impl<X: XConn> Hook<X> for StartHook {
    fn startup(&mut self, _wm: &mut WindowManager<X>) -> penrose::Result<()> {
        spawn(START_SCRIPT)
    }
}

fn main() -> penrose::Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    let floating_classes = vec![
        "rofi",
        "dmenu",
        "Arandr",
        "Fsearch",
        "floating"];

    let config = Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .floating_classes(floating_classes)
        // .layouts(my_layouts())
        .border_px(3)
        .focused_border(0xc678dd)
        .bar_height(30)
        .build()
        .unwrap();

    // let config = Config::default();
    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-r" => run_external!(LAUNCHER);
        "M-t" => run_external!(TERMINAL);
        "M-b" => run_external!(BROWSER);
        "M-e" => run_external!(EDITOR);
        // TODO: make a new macro for running terminal cmds
        // "M-h" => run_external!(concat!(TERMINAL, " -e htop"));
        "M-s" => run_external!("rofi -show ssh");
        "M-w" => run_external!("rofi -show window");
        "M-o" => run_external!("fsearch");
        "M-p" => run_external!("rofi-pass");
        "M-u" => run_external!("arcolinux-logout");

        // Exit Penrose (important to remember this one!)
        "M-A-C-Escape" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-n" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-a" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-n" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-a" => run_internal!(drag_client, Backward);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-x" => run_internal!(kill_client);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);

        // Layout management
        "M-space" => run_internal!(cycle_layout, Forward);
        "M-S-space" => run_internal!(cycle_layout, Backward);
        "M-Up" => run_internal!(update_max_main, More);
        "M-Down" => run_internal!(update_max_main, Less);
        "M-Right" => run_internal!(update_main_ratio, More);
        "M-Left" => run_internal!(update_main_ratio, Less);

        refmap [ config.ws_range() ] in {
            "M-{}" => focus_workspace [ index_selectors(config.workspaces().len()) ];
            "M-S-{}" => client_to_workspace [ index_selectors(config.workspaces().len()) ];
            "M-C-{}" => client_to_workspace [ index_selectors(config.workspaces().len()) ];
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, vec![StartHook::new()], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})
}
