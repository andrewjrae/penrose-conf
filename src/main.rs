#[macro_use]
extern crate penrose;

#[macro_use]
extern crate penrose_ajrae;

use penrose::{
    contrib::hooks::ManageExistingClients,
    core::{
        config::Config,
        helpers::index_selectors,
        layout::{side_stack, monocle},
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector, Result
};

use penrose_ajrae::{
    actions::{inc_redshift, soft_restart},
    hooks::{StartupScript, CenterFloats},
    TERMINAL, BROWSER, EDITOR, LAUNCHER, START_SCRIPT, FOLLOW_FOCUS_CONF
};

use std::collections::HashMap;

// Start hook to run all the important stuff like picom etc
fn main() -> Result<()> {

    log4rs::init_file("/home/ajrae/penrose-conf/logger.yml", Default::default()).unwrap();

    let floating_classes = vec![
        // "rofi",
        "fsearch",
        "arcologout.py",
        ];

    let layouts = vec![
        layout!("side", side_stack),
        layout!("mono", FOLLOW_FOCUS_CONF, monocle),
    ];

    let config = Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .floating_classes(floating_classes)
        .layouts(layouts)
        .border_px(3)
        .bar_height(30)
        .focused_border("#c678dd")?
        .build()
        .unwrap();


    let key_bindings = gen_keybindings! {
        // Main programs
        "M-r" => run_external!(LAUNCHER);
        "M-t" => run_external!(TERMINAL);
        "M-b" => run_external!(BROWSER);
        "M-e" => run_external!(EDITOR);

        // Rofi and other utils
        "M-s" => run_external!("rofi -show ssh");
        "M-w" => run_external!("rofi -show window");
        "M-o" => run_external!("fsearch");
        "M-p" => run_external!("rofi-pass");
        "M-u" => run_external!("arcolinux-logout");

        // Terminal utils
        "M-h" => run_in_terminal!("btop");
        "M-q" => run_in_terminal!("qalc");

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

        "M-XF86MonBrightnessUp" => inc_redshift(250);
        "M-XF86MonBrightnessDown" => inc_redshift(-250);

        // Screenshots
        "Print" => run_external!("flameshot gui");
        "C-Print" => run_external!("escrotum -C -s" );
        "C-S-Print" => run_external!("escrotum -C -s" );

        // Exit Penrose (important to remember this one!)
        "M-A-C-Escape" => run_internal!(exit);
        "M-A-C-r" => soft_restart();

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
        "M-space" => run_internal!(toggle_workspace);
        "M-y" => run_internal!(cycle_screen, Forward);
        "M-c" => run_internal!(cycle_screen, Backward);

        // Layout management
        "M-f" => run_internal!(cycle_layout, Forward);
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

    let float_scales: HashMap<String, f64> = [
        ("fsearch".to_string(), 0.4),
        ("arandr".to_string(), 0.6),
        ("arcologout.py".to_string(), 0.7),
    ].iter().cloned().collect();

    let center_ignores: Vec<String> = vec!["polybar".to_string(),
                                           "tray".to_string(),
                                           "pamac-tray".to_string()];

    let mut wm = new_xcb_backed_window_manager(
        config,
        vec![StartupScript::new(START_SCRIPT),
             ManageExistingClients::new(),
             CenterFloats::new(0.45, float_scales, center_ignores)
        ],
        logging_error_handler())?;

    wm.init()?;
    wm.grab_keys_and_run(key_bindings, HashMap::new())?;

    Ok(())
}
