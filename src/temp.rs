use anyhow::Result;
use clap::{Args, Parser, Subcommand};

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
    Init(InitArgs),

    /// Update
    Update(UpdateArgs),

    /// Configure
    Conf(ConfArgs),
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct ConfArgs {}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct InitArgs {
    #[arg(
        default_value = "mixapi",
        short = 'n',
        help = "the name of the cli application"
    )]
    app_name: String,

    #[arg(
        default_value = "sumit-modak/mixapi",
        short = 'p',
        help = "the default path of the app config file"
    )]
    default_path: String,

    #[arg(short = 's', help = "the sql server database connection string")]
    pub db_conn_str: String,
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct UpdateArgs {
    #[arg(short = 'n', help = "some sample number")]
    number: u8,
}

pub fn redirect(mut cfg: &mut crate::AppConfig) -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(args) => {
            init_main(args, &mut cfg)?;
        }
        Commands::Update(args) => {
            update_main(args, &mut cfg)?;
        }
        Commands::Conf(args) => {
            conf_main(args, &mut cfg)?;
        }
    }

    anyhow::Ok(())
}

pub fn conf_main(args: &ConfArgs, cfg: &mut crate::AppConfig) -> Result<()> {
    println!("command args: {:?}", args);
    let full_path = confy::get_configuration_file_path("myserv-cli", None);
    println!("config file full_path: {:?}", full_path);
    println!("app-config: {:?}", cfg);
    println!("redacted db conn str: {:?}", redact_str(&cfg.db_conn_str));
    Ok(())
}

fn redact_str(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    result.extend((0..std::cmp::max(text.len() - 6, 0)).map(|_| '*'));
    result.push_str(&text[std::cmp::max(text.len() - 6, 0)..]);
    result
}

pub fn init_main(args: &InitArgs, _cfg: &mut crate::AppConfig) -> Result<()> {
    if args.db_conn_str == "" {
        println!("please provide the db connection string");
        return Ok(());
    }

    let mut _config = crate::AppConfig::new();
    println!("from the init command");
    Ok(())
}

pub fn update_main(args: &UpdateArgs, cfg: &mut crate::AppConfig) -> Result<()> {
    println!("command args: {:?}", cfg);
    println!("update args: {:?}", args);
    Ok(())
}
