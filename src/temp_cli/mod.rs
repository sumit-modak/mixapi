pub mod conf;
pub mod init;
pub mod update;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, propagate_version = true,
    about = "A cli app made up of axum, ratatui and std",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, short, help = "rand argument")]
    abc: Option<String>,

    #[arg(long, short, help = "rand argument")]
    lol: Option<u8>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize
    Init(init::InitArgs),

    /// Update
    Update(update::UpdateArgs),

    /// Configure
    Conf(conf::ConfArgs),
}

pub fn redirect(mut cfg: &mut crate::AppConfig) -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(args) => {
            init::main(args, &mut cfg)?;
        }
        Commands::Update(args) => {
            update::main(args, &mut cfg)?;
        }
        Commands::Conf(args) => {
            conf::main(args, &mut cfg)?;
        }
    }

    anyhow::Ok(())
}
