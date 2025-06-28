use colored::Colorize;

pub fn fetch_memory() -> String {
    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
    let lines = meminfo.lines();
    let mut mem_total = 0u64;
    let mut mem_used = 0u64;

    for line in lines.into_iter() {
        let mut it = line.split_ascii_whitespace();
        let key = it.next().unwrap();
        let val = it.next().unwrap().parse::<u64>().unwrap();

        if key == "MemTotal:" {
            mem_total += val;
            mem_used += val;
        } else if key == "Shmem:" {
            mem_used += val;
        } else if key == "MemFree:" {
            mem_used -= val;
        } else if key == "Buffers:" {
            mem_used -= val;
        } else if key == "Cached:" {
            mem_used -= val;
        } else if key == "SReclaimable:" {
            mem_used -= val;
        }
    }

    format!(
        "{}: {}\n{}: {}\n{}: {}\n",
        "MemTotal".red(),
        super::formatted_memory(mem_total),
        "MemUsed".red(),
        super::formatted_memory(mem_used),
        "MemAvail".red(),
        super::formatted_memory(mem_total - mem_used)
    )
}
