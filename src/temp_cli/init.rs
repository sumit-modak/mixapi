use anyhow::{Ok, Result};
use clap::Args;

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

pub fn main(args: &InitArgs, cfg: &mut crate::AppConfig) -> Result<()> {
    if args.db_conn_str == "" {
        println!("please provide the db connection string");
        return Ok(());
    }

    let mut config = crate::AppConfig::new();
    config
        .set_db_conn_str(&args.db_conn_str)
        .set_config_path(&args.default_path)
        .build();
    cfg.update(&config);
    println!("from the init command");
    Ok(())
}
