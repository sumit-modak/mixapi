use anyhow::{Ok, Result};
use clap::Args;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct UpdateArgs {
    #[arg(short = 'n', help = "some sample number")]
    number: u8,
}

pub fn main(args: &UpdateArgs, cfg: &mut crate::AppConfig) -> Result<()> {
    println!("command args: {:?}", cfg);
    println!("update args: {:?}", args);
    Ok(())
}
