//! Domain types and session state machine for AnotAI (no I/O, no async).

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(env!("CARGO_PKG_NAME"), "anotai-core");
    }
}
