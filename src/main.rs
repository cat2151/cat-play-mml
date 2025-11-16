use anyhow::Result;

mod app;
mod cli;
mod client_manager;
mod converter;
mod input;
mod process_manager;

use app::App;

fn main() -> Result<()> {
    let app = App::new();
    app.run()
}
