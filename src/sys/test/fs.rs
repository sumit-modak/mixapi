// use std::process::Stdio;
use tokio::process::Command;

pub async fn check_path(query: String) {
    let result = match tokio::fs::read_to_string(&query).await {
        Ok(data) => data,
        Err(_) => {
            let ls = Command::new("ls")
                .args(["-aFl", "--group-directories-first"])
                .arg(&query)
                .output()
                .await
                .unwrap()
                .stdout;
            String::from_utf8(ls).unwrap()
        }
    };

    println!("{}", result);
}
