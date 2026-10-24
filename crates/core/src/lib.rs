//! Domain types and session state machine for AnotAI (no I/O, no async).
//!
//! Types consumed by the UI derive [`ts_rs::TS`] and are exported to TypeScript when
//! `cargo test -p anotai-core` runs. The output folder comes from `TS_RS_EXPORT_DIR`
//! in `.cargo/config.toml`.

mod session;
mod state;
mod timestamp;

pub use session::{AudioSourceKind, PartialText, Segment, Session, SessionEvent, SessionId};
pub use state::{SessionCommand, SessionState, TransitionError};
pub use timestamp::format_timestamp;

#[cfg(test)]
mod tests {
    /// The core crate stays free of async runtimes, file system and network access.
    #[test]
    fn has_no_io_or_async_dependencies() {
        let manifest = include_str!("../Cargo.toml");
        for dep in ["tokio", "async-std", "reqwest", "hyper"] {
            assert!(
                !manifest.contains(dep),
                "anotai-core must not depend on {dep}"
            );
        }

        let sources = [
            include_str!("lib.rs"),
            include_str!("session.rs"),
            include_str!("state.rs"),
            include_str!("timestamp.rs"),
        ];
        let forbidden = [
            ["std", "::fs"].concat(),
            ["std", "::net"].concat(),
            ["async", " fn"].concat(),
        ];
        for source in sources {
            for pattern in &forbidden {
                assert!(!source.contains(pattern.as_str()), "found `{pattern}`");
            }
        }
    }
}
