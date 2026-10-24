//! Session pipeline for AnotAI: capture, VAD, transcription and events.

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "anotai-session");
    }
}
