extern crate glium;
#[macro_use]
extern crate imgui;
//extern crate imgui_sys;
extern crate imgui_glium_renderer;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use imgui::*;

mod support;
pub mod styles;
pub mod ssb;
pub mod sbot;

struct State {
    open: bool,

    auto_scroll: bool,

    show_intro_screen: bool,
    show_chat_screen: bool,

    chat_main_buf: ImString,
    chat_input_buf: ImString,

    chat_start_load: bool,
    chat_message_type: String,
    chat_network_name: String,

    chat_networks_available: Vec<String>,
    chat_networks_current_index: i32,

    user_handle: ImString,
    test_data: String,

    chat_instance: Option<ssb::ChatInstance>,
}

impl Default for State {
    fn default() -> Self {
        let mut chat_main_buf = ImString::with_capacity(10240);
        chat_main_buf.push_str("none");
        let mut chat_input_buf = ImString::with_capacity(128);
        chat_input_buf.push_str("tzag");
//        let mut text_multiline = ImString::with_capacity(128);
//        text_multiline.push_str("Hello, world!\nMultiline");
        let mut user_handle = ImString::with_capacity(32);
        user_handle.push_str("USER_HANDLE_NOT_FOUND");
        //let mut chat_instance = ssb::ChatInstance::new();
        let mut chat_networks_available = vec!(String::from("ssb"), String::from("misc-altnet"));
        State {
            open: true,

            auto_scroll: true,

            show_intro_screen: true,
            show_chat_screen: false,

            chat_main_buf,
            chat_input_buf,

            chat_start_load: false,
            chat_message_type: String::new(),
            chat_network_name: String::from("ssb"),
            chat_networks_available,
            chat_networks_current_index: 0,

            user_handle,
            test_data: String::from("nothing"),

            chat_instance: None,
        }
    }
}

struct ColorEditState {
    color: [f32; 4],
    hdr: bool,
    alpha_preview: bool,
    alpha_half_preview: bool,
    options_menu: bool,
    alpha: bool,
    alpha_bar: bool,
    side_preview: bool,
    ref_color: bool,
    ref_color_v: [f32; 4],
}

impl Default for ColorEditState {
    fn default() -> Self {
        ColorEditState {
            color: [114.0 / 255.0, 144.0 / 255.0, 154.0 / 255.0, 200.0 / 255.0],
            hdr: false,
            alpha_preview: true,
            alpha_half_preview: false,
            options_menu: true,
            alpha: true,
            alpha_bar: true,
            side_preview: true,
            ref_color: false,
            ref_color_v: [1.0, 0.0, 1.0, 0.5],
        }
    }
}

struct FileMenuState {
    enabled: bool,
    f: f32,
    n: i32,
    b: bool,
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            enabled: true,
            f: 0.5,
            n: 0,
            b: true,
        }
    }
}

struct AutoResizeState {
    lines: i32,
}

impl Default for AutoResizeState {
    fn default() -> Self {
        AutoResizeState { lines: 10 }
    }
}

struct CustomRenderingState {
    sz: f32,
    col: [f32; 3],
    points: Vec<(f32, f32)>,
    adding_line: bool,
}

impl Default for CustomRenderingState {
    fn default() -> Self {
        CustomRenderingState {
            sz: 36.0,
            col: [1.0, 1.0, 0.4],
            points: vec![],
            adding_line: false,
        }
    }
}

fn main() {
    let mut state = State::default();

    support::run("misc test".to_owned(), styles::DARK_COLOR, |ui| {

        ui.with_style_and_color_vars(&styles::dark_style(), &styles::dark_colors(), || {

            if state.chat_start_load {
                state.chat_instance =
                    Some(ssb::ChatInstance::new(
                        state.chat_message_type.clone(),
                        state.chat_network_name.clone()));
                state.chat_main_buf.push_str(&state.chat_instance.as_mut().unwrap().format_messages_to_string());
                state.user_handle = ImString::new(ssb::get_user_handle(
                    ssb::whoami(&state.chat_network_name), &state.chat_network_name));
                state.show_chat_screen = true;
                state.show_intro_screen = false;
                state.chat_start_load = false;
            }

            if state.show_intro_screen {
                show_intro_screen(ui, &mut state);
            }
//        show_test_window(ui, &mut state, &mut open);
            if state.show_chat_screen {
                show_chat_screen(ui, &mut state);
            }

        });
        state.open
    });
}

fn show_intro_screen(ui: &Ui, state: &mut State) {
    let (display_x, display_y) = ui.imgui().display_size();
    //ui.text(im_str!("{} {}", display_x, display_y));
    let (window_size_x,  window_size_y) = (500.0, 200.0);
    let window_pos = (display_x * 0.5 - window_size_x * 0.5, display_y * 0.5 - window_size_y * 0.5);
    ui.with_color_var(ImGuiCol::WindowBg, (0.1, 0.13, 0.16, 1.0), || {
        ui.window(im_str!("endgame overlay"))
            .opened(&mut true)
            .position(window_pos, ImGuiCond::Always)
            .title_bar(false)
            .resizable(false)
            .always_auto_resize(true)
            .movable(false)
            .save_settings(false)
            .build(|| {
                ui.text_colored((0.9,0.9,0.9,0.8),
                                im_str!("MISC"));
                ui.same_line(55.0);
                ui.text_colored((0.5,0.5,0.5,0.4),
                                im_str!("| MInimal Scuttlebutt Chat"));
                ui.separator();
                ui.spacing();
                ui.text_colored((0.7,0.7,0.7,0.5),
                                im_str!("This is an experiment. Things will break!"));
                ui.spacing();

                let mut network_names: Vec<&ImStr> = Vec::new();
                for n in state.chat_networks_available.as_slice() {
//                    network_names.push(&ImStr::new(&n));

                    unsafe {
                        let imstring = imgui::ImStr::from_utf8_with_nul_unchecked(n.as_bytes());
                        network_names.push(imstring);
                    };

                }
                ui.push_item_width(320.0);
                if ui.combo(
                    im_str!(" "),
                    &mut state.chat_networks_current_index,
//                    &[
//                        im_str!("ssb mainnet"),
//                        im_str!("altnet0"),
//                        im_str!("altnet1"),
//                        im_str!("altnet2"),
//                        im_str!("altnet3"),
//                    ],
                        network_names.as_slice(),
//                    &state.chat_networks_available.iter().map(|name| ImStr::new(&name)),
                    -1) {
                    state.chat_network_name = state.chat_networks_available[state.chat_networks_current_index as usize].clone();
                }

                ui.with_color_vars(&[(ImGuiCol::Text, (0.5,0.5,0.5,0.8))], || {
                    ui.same_line(340.0);
                    ui.text_colored((0.7,0.7,0.7,0.5),
                                im_str!("network"));
                });
                ui.same_line(410.0);
                show_help_marker(&ui, "Choose the network to use.\n\nCurrently you can only use the ssb mainnet.");

                ui.push_item_width(320.0);
                ui.combo(
                    im_str!("  "),
                    &mut 0,
                    &[
                        im_str!("scat_message"),
                        im_str!("friendly_chat"),
                        im_str!("something_else_entirely"),
                    ],
                    -1,
                );

                ui.with_color_vars(&[(ImGuiCol::Text, (0.5,0.5,0.5,0.8))], || {
                    ui.same_line(340.0);
                    ui.text_colored((0.7,0.7,0.7,0.5),
                                    im_str!("message type"));
                });
                ui.same_line(455.0);
                show_help_marker(&ui, "Only messages of one specific type\nwill be used for a single chat instance.");


                ui.input_text(im_str!("   "), &mut state.user_handle)
                    .read_only(true)
                    .build();

                ui.with_color_vars(&[(ImGuiCol::Text, (0.5,0.5,0.5,0.8))], || {
                    ui.same_line(340.0);
                    ui.text_colored((0.7,0.7,0.7,0.5),
                                    im_str!("handle"));
                });
                ui.same_line(400.0);
                show_help_marker(&ui,
                                 "This is the latest handle\nyou've picked for yourself.\n\n\
                                     It's based on the 'about' type message\nwith a 'name' content entry.");

                ui.pop_item_width();

                ui.spacing();
                if ui.button(im_str!("open"), (320.0, 25.0)) {
                    show_loading_screen(&ui);
                    state.chat_start_load = true;
                    state.chat_message_type = String::from("scat_message");
//                    state.chat_instance = ssb::ChatInstance::new_from_msg_type(String::from("scat_message"));
//                    state.chat_main_buf.push_str(&state.chat_instance.format_messages_to_string());
//                    state.show_chat_screen = true;
//                    state.show_intro_screen = false;
                }
                ui.same_line(0.0);
                if ui.button(im_str!("quit"), (170.0, 25.0)) {
                    state.open = false;
                }
                ui.same_line(0.0);
            })
    })
}

fn show_chat_screen(ui: &Ui, state: &mut State) {
    let (display_x, display_y) = ui.imgui().display_size();

    ui.with_color_var(ImGuiCol::WindowBg, (0.1, 0.13, 0.16, 1.0), || {
        let mut window = ui
            .window(im_str!("Chat"))
            .title_bar(false)
            .resizable(false)
            .movable(true)
            .scroll_bar(true)
            .collapsible(false)
            .menu_bar(false)
            .position((0.0, 0.0), ImGuiCond::Always)
            .size((display_x, display_y - 50.0), ImGuiCond::Always);


        window.build(|| {
//            ui.input_text_multiline(im_str!("main_multiline"),
//                                    &mut state.chat_main_buf,
//                                    (display_x, display_y - 50.0))
//                .read_only(true)
//                .build();
            for msg in &mut state.chat_instance.as_mut().unwrap().chat_messages {
                ui.push_item_width(170.0);
                ui.text_wrapped(im_str!("{}", msg.author_handle));
                ui.pop_item_width();
                ui.same_line(180.0);
                ui.text_wrapped(im_str!("{}", msg.text));
            }
//            let s = state.chat_main_buf.clone();
//            //println!("{}", s);
//            ui.text_wrapped(&s);
            if state.auto_scroll {
                unsafe {
//                imgui::sys::igSetScrollY(1.0);
                    imgui::sys::igSetScrollHere(1.0);
                };
            }
        });


    });

    let mut window = ui
        .window(im_str!("Chat Input"))
        .title_bar(false)
        .resizable(false)
        .movable(true)
        .scroll_bar(false)
        .collapsible(false)
        .menu_bar(false)
        .position((0.0, display_y - 50.0), ImGuiCond::Always)
        .size((display_x, 50.0), ImGuiCond::Always);


    window.build(|| {

        ui.set_cursor_screen_pos((10.0, display_y - 36.0));
        if let Some(ref mut chat_instance) = state.chat_instance {
            ui.text_colored((0.5,0.5,0.5,0.8),
                            im_str!("{}:", chat_instance.current_user_handle));
        }

        ui.push_item_width(display_x - 440.0);
        ui.set_cursor_screen_pos((180.0, display_y - 38.0));
        ui.input_text(im_str!(" "), &mut state.chat_input_buf)
            .build();
//        ui.same_line(500.0);
        ui.set_cursor_screen_pos((display_x - 130.0, display_y - 40.0));
        if ui.button(im_str!("close"), (80.0, 28.0)) {
//            match &mut state.chat_instance {
//                Some(ref mut chat) => chat.kill(),
//                None => (),
//            }
            if let Some(ref mut chat_instance) = state.chat_instance {
                chat_instance.kill();
            }
//            state.chat_instance.unwrap().kill();
            state.show_chat_screen = false;
            state.show_intro_screen = true;
        }
        ui.set_cursor_screen_pos((display_x - 240.0, display_y - 40.0));
        if ui.button(im_str!("publish"), (100.0, 28.0)) {
            if let Some(ref mut chat_instance) = state.chat_instance {
                chat_instance.publish_message(state.chat_input_buf.to_str().to_string());
            }

        }
        ui.set_cursor_screen_pos((display_x - 40.0, display_y - 38.0));
        ui.checkbox(im_str!("  "), &mut state.auto_scroll);
        if ui.is_item_hovered() {
            ui.tooltip(|| {
                ui.text("auto-scroll?");
            });
        }
    });
}

fn show_loading_screen(ui: &Ui) {
    let (display_x, display_y) = ui.imgui().display_size();

    ui.with_color_var(ImGuiCol::WindowBg, (0.1, 0.13, 0.16, 1.0), || {
        let mut window = ui
            .window(im_str!("Chat"))
            .title_bar(false)
            .resizable(false)
            .movable(true)
            .scroll_bar(true)
            .collapsible(false)
            .menu_bar(false)
            .position((0.0, 0.0), ImGuiCond::Always)
            .size((display_x, display_y), ImGuiCond::Always);

        window.build(|| {
            ui.set_cursor_screen_pos((display_x * 0.5 - 100.0, display_y * 0.5 - 20.0));
            ui.text_wrapped(im_str!("connecting to scuttlebot..."));
        });


    });
}

fn show_help_marker(ui: &Ui, desc: &str) {
    ui.text_disabled(im_str!("(?)"));
    if ui.is_item_hovered() {
        ui.tooltip(|| {
            ui.text(desc);
        });
    }
}

