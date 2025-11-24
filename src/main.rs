use anyhow::Result;

mod app;
mod cli;
mod client_manager;
mod converter;
mod input;

use app::App;

fn main() -> Result<()> {
    let app = App::new();
    app.run()
}
