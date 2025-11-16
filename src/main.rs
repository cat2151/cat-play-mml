use anyhow::Result;

mod app;
mod cli;
#[cfg(windows)]
mod client_manager;
mod converter;
mod input;
#[cfg(windows)]
mod process_manager;

use app::App;

fn main() -> Result<()> {
    let app = App::new();
    app.run()
}
