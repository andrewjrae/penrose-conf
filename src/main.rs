#[macro_use]
extern crate penrose;

// #[macro_use]
extern crate penrose_ajrae;

use penrose::{
    core::{
        // bindings::KeyEventHandler,
        // client::Client,
        // hooks::Hook,
        config::Config,
        helpers:: index_selectors,
        // xconnection::{XConn, Xid},
        // manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector, Result
};

use penrose_ajrae::{
    hooks::{StartupScript, CenterFloat},
    TERMINAL, LAUNCHER, BROWSER, EDITOR, START_SCRIPT
};

use simplelog::{LevelFilter, SimpleLogger};

use std::collections::HashMap;

// Start hook to run all the important stuff like picom etc
fn main() -> Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    let floating_classes = vec![
        "rofi",
        "dmenu",
        "Arandr",
        "Fsearch",
        "arcologout.py",
        "pinentry-gtk-2",
        "polybar",
        "floating",
        ];

    let config = Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .floating_classes(floating_classes)
        // .layouts(my_layouts())
        .border_px(3)
        .bar_height(30)
        .focused_border("#c678dd")?
        .build()
        .unwrap();

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

        "M-C-w" => run_external!("networkmanager_dmenu");

        // Media keys
        "XF86AudioMute" => run_external!("amixer -q set Master toggle");
        "XF86AudioLowerVolume" => run_external!("amixer -q set Master 5%-");
        "XF86AudioRaiseVolume" => run_external!("amixer -q set Master 5%+");
        "XF86MonBrightnessUp" => run_external!("xbacklight -inc 5");
        "XF86MonBrightnessDown" => run_external!("xbacklight -dec 5");
        "XF86AudioPlay" => run_external!("playerctl play-pause");
        "XF86AudioNext" => run_external!("playerctl next");
        "XF86AudioPrev" => run_external!("playerctl previous");
        "XF86AudioStop" => run_external!("playerctl stop");

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
        "M-C-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
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

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
            "M-C-{}" => client_to_workspace (REF);
        };
    };

    let mut wm = new_xcb_backed_window_manager(
        config,
        vec![StartupScript::new(START_SCRIPT), CenterFloat::new("floating", 0.9)],
        logging_error_handler())?;

    wm.init()?;
    wm.grab_keys_and_run(key_bindings, HashMap::new())?;

    Ok(())
}
