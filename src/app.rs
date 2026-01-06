mod app;

use app::App;

fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    app.run()?;
    Ok(())
}
