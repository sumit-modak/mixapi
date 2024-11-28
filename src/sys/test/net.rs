// use std::process::Stdio;
use tokio::process::Command;

pub async fn ifconfig() {
    let ifconfig = Command::new("ifconfig").output().await.unwrap();

    println!("{:?}", ifconfig);
}
