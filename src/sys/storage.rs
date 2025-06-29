use colored::Colorize;

pub fn fetch_disk_usage() -> String {
    if let Ok(df) = std::process::Command::new("df").output() {
        for line in String::from_utf8(df.stdout).unwrap().lines() {
            let mut s = line.split_ascii_whitespace();
            if let Some("/") = s.clone().nth(5) {
                s.next();
                return format!(
                    "{}: {}\n{}: {}\n{}: {}\n",
                    "DiskSize".red(),
                    super::formatted_memory(s.next().unwrap().parse().unwrap()),
                    "DiskUsed".red(),
                    super::formatted_memory(s.next().unwrap().parse().unwrap()),
                    "DiskAvail".red(),
                    super::formatted_memory(s.next().unwrap().parse().unwrap()),
                );
            }
        }
    }

    format!("")
}
