use colored::Colorize;
use std::path::PathBuf;

pub fn fetch_shell_info() -> String {
    let shell_path = PathBuf::from(env!("SHELL"));
    let op = String::from_utf8(
        std::process::Command::new(&shell_path)
            .arg("--version")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let mut version = op.split_ascii_whitespace().nth(3).unwrap().to_string();

    format!(
        "{}: {} {}\n",
        "Shell".red(),
        shell_path.file_name().unwrap().to_str().unwrap(),
        version
            .drain(..version.find('(').unwrap_or(version.len()))
            .collect::<String>(),
    )
}
