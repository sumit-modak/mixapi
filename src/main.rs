#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cfg: mixapi::AppConfig = confy::load("mixapi", None)?;

    // mixapi::temp_cli::main(&mut cfg)?;
    mixapi::args::redirect(&mut cfg).await?;

    anyhow::Ok(())
}
