use colored::Colorize;

pub fn fetch_prompt() -> String {
    let user = env!("USER");
    let hostname = std::fs::read_to_string("/etc/hostname").unwrap();
    format!("{}{}{}\n", user.yellow(), "@".green(), hostname.blue())
}
