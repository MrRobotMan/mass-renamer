#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
                                                                   // use tracing_subscriber;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Bulk File Renamer",
        native_options,
        Box::new(|cc| Box::new(bulk_file_renamer::Renamer::new(cc))),
    );
}
