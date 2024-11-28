// use std::process::Stdio;
use tokio::process::Command;

pub async fn get_pstree() {
    let output = Command::new("pstree").output().await.unwrap();

    println!(
        "{}\n{}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );
}

pub async fn get_ps_list() {
    let output = Command::new("ps").arg("aux").output().await.unwrap();

    println!(
        "{}\n{}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );
}
