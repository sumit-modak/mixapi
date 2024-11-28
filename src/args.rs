use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, propagate_version = true)]
#[command(
    about = "A cli app made up of axum, ratatui and std",
    long_about = None,
)]
pub struct CliArgs {
    /// Determines what type of app you want to run
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a web API
    Web(WebArgs),

    /// Run TUI app
    Tui(TuiArgs),

    /// Fetch systems information
    Sys(SysArgs),
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct WebArgs {}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct TuiArgs {}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct SysArgs {}

// Redirects user to their correct function
pub async fn redirect(mut cfg: &mut crate::AppConfig) -> anyhow::Result<()> {
    let cli = CliArgs::parse();

    match cli.command {
        Commands::Web(args) => {
            crate::routes::main(args, &mut cfg).await;
        }
        Commands::Tui(_args) => {
            // crate::tui::main(args, &mut cfg)?;
        }
        Commands::Sys(args) => {
            crate::sys::main(args, &mut cfg).await;
        }
    }

    anyhow::Ok(())
}
