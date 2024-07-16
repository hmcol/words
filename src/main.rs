use std::io;

// -----------------------------------------------------------------------------

mod app;
mod pred;
mod tui;
mod words;

// =============================================================================

fn main() -> std::io::Result<()> {
    colog::basic_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();
    log::info!("logging initialized");

    let mut terminal = tui::init()?;
    let app_result = app::App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

// -----------------------------------------------------------------------------
