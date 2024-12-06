use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, propagate_version = true)]
#[command(
    about = "A cli app made up of clap, axum and ratatui",
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
pub struct TuiArgs {
    pub layout: String,
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct SysArgs {}

// Redirects user to their correct function
pub async fn redirect(mut cfg: &mut crate::AppConfig) -> anyhow::Result<()> {
    let cli = CliArgs::parse();

    match cli.command {
        Commands::Web(args) => {
            crate::web::main(args, &mut cfg).await;
        }
        Commands::Tui(args) => {
            crate::tui::main(args, &mut cfg)?;
        }
        Commands::Sys(args) => {
            crate::sys::main(args, &mut cfg).await;
        }
    }

    anyhow::Ok(())
}
