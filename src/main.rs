#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
                                                                   // use tracing_subscriber;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Some(egui::Vec2::new(1400.0, 800.0)),
        min_window_size: Some(egui::Vec2::new(1400.0, 800.0)),
        max_window_size: None,
        resizable: true,
        transparent: false,
        mouse_passthrough: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        run_and_return: true,
        event_loop_builder: None,
        shader_version: None,
        centered: true,
    };
    eframe::run_native(
        "Bulk File Renamer",
        native_options,
        Box::new(|cc| Box::new(bulk_file_renamer::Renamer::new(cc))),
    );
}
