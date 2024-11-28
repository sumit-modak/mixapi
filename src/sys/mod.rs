mod cpu;
mod kernel;
mod memory;
mod os;
mod prompt;
mod resolution;
mod shell;
mod storage;
mod uptime;

// remaining
mod battery;
mod gpu;

// untested
pub mod test;

pub use cpu::fetch_cpu;
pub use gpu::fetch_gpu;
pub use kernel::fetch_kernel_info;
pub use memory::fetch_memory;
pub use os::fetch_os_info;
pub use prompt::fetch_prompt;
pub use resolution::fetch_resolution; //
pub use shell::fetch_shell_info; //
pub use storage::fetch_disk_usage; //
pub use uptime::fetch_uptime;

pub(crate) fn formatted_memory(kb: u64) -> String {
    let total_bytes = 1000 * kb;

    let (gib, rem_bytes) = (total_bytes / 1073741824, total_bytes % 1073741824);
    let (mib, rem_bytes) = (rem_bytes / 1048576, rem_bytes % 1048576);
    let (kib, bytes) = (rem_bytes / 1024, rem_bytes % 1024);

    format!("{gib}G {mib}M {kib}K {bytes}B")
}

pub async fn main(args: crate::args::SysArgs, cfg: &mut crate::AppConfig) {
    let mut response = String::with_capacity(1024);

    response.push_str(&fetch_prompt());
    response.push_str(&fetch_os_info());
    response.push_str(&fetch_kernel_info());
    response.push_str(&fetch_shell_info());
    response.push_str(&fetch_uptime());
    response.push_str(&fetch_disk_usage());
    response.push_str(&fetch_memory());
    response.push_str(&fetch_cpu());
    response.push_str(&fetch_resolution());

    // crate::test::fs::check_path(".".to_string()).await;
    // crate::test::ps::get_pstree().await;
    // crate::test::ps::get_ps_list().await;

    println!("{response}");
}
