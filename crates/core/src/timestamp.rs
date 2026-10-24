/// Formats a session offset in milliseconds as `mm:ss`, or `hh:mm:ss` from one hour on.
///
/// Milliseconds are truncated, so `59_999` is `00:59`.
pub fn format_timestamp(ms: u64) -> String {
    let total_secs = ms / 1000;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::format_timestamp;

    #[test]
    fn formats_timestamps() {
        let cases = [
            (0, "00:00"),
            (999, "00:00"),
            (1_000, "00:01"),
            (59_999, "00:59"),
            (61_500, "01:01"),
            (3_599_999, "59:59"),
            (3_600_000, "01:00:00"),
            (5_025_000, "01:23:45"),
            (36_000_000, "10:00:00"),
            (360_000_000, "100:00:00"),
        ];
        for (ms, expected) in cases {
            assert_eq!(format_timestamp(ms), expected, "{ms} ms");
        }
    }
}
