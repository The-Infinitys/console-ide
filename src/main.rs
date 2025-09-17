use clap::Parser;
use console_ide::app::App;
use console_ide::utils::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut app = App::new();
    if let Some(workspace) = args.path {
        app.set_workspace(workspace);
    }
    app.run()?;

    Ok(())
}
