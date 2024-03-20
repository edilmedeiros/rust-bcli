//
// utils.rs
//

pub fn format_uptime(uptime_seconds: u64) -> String {
    let days = uptime_seconds / 86_400;
    let hours = (uptime_seconds % 86_400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    let seconds = uptime_seconds % 60;

    format!(
        "{} days, {} hours, {} minutes, {} seconds",
        days, hours, minutes, seconds
    )
}
