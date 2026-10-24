use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;

/// Lifecycle of a recording session.
///
/// ```text
/// Idle ──Start──▶ Recording ◀──Resume── Paused
///                 Recording ──Pause───▶ Paused
/// Recording | Paused ──Stop──▶ Stopping ──Finish──▶ Stopped
/// any non-terminal state ──Fail──▶ Failed
/// ```
///
/// `Stopped` and `Failed` are terminal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[ts(export)]
pub enum SessionState {
    Idle,
    Recording,
    Paused,
    /// Capture has stopped and the remaining audio is being transcribed.
    Stopping,
    Stopped,
    /// The session ended because of an error. `reason` never contains transcript text.
    Failed {
        reason: String,
    },
}

/// Requests that move a session between [`SessionState`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[ts(export)]
pub enum SessionCommand {
    Start,
    Pause,
    Resume,
    Stop,
    /// Marks the end of the work started by `Stop`.
    Finish,
    Fail {
        reason: String,
    },
}

/// A command that is not allowed in the current state.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("cannot apply {command:?} to a session in state {from:?}")]
pub struct TransitionError {
    pub from: SessionState,
    pub command: SessionCommand,
}

impl SessionState {
    /// Applies `cmd` and returns the next state, or an error if the transition is invalid.
    pub fn transition(self, cmd: SessionCommand) -> Result<SessionState, TransitionError> {
        use SessionCommand as C;
        use SessionState as S;

        match (self, cmd) {
            (S::Idle, C::Start) => Ok(S::Recording),
            (S::Recording, C::Pause) => Ok(S::Paused),
            (S::Paused, C::Resume) => Ok(S::Recording),
            (S::Recording | S::Paused, C::Stop) => Ok(S::Stopping),
            (S::Stopping, C::Finish) => Ok(S::Stopped),
            (from, C::Fail { reason }) if !from.is_terminal() => Ok(S::Failed { reason }),
            (from, command) => Err(TransitionError { from, command }),
        }
    }

    /// Whether no further transition is possible.
    pub fn is_terminal(&self) -> bool {
        matches!(self, SessionState::Stopped | SessionState::Failed { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use SessionCommand as C;
    use SessionState as S;

    fn fail() -> SessionCommand {
        C::Fail {
            reason: "device lost".into(),
        }
    }

    fn failed() -> SessionState {
        S::Failed {
            reason: "device lost".into(),
        }
    }

    #[test]
    fn valid_transitions() {
        let cases = [
            (S::Idle, C::Start, S::Recording),
            (S::Recording, C::Pause, S::Paused),
            (S::Paused, C::Resume, S::Recording),
            (S::Recording, C::Stop, S::Stopping),
            (S::Paused, C::Stop, S::Stopping),
            (S::Stopping, C::Finish, S::Stopped),
            (S::Idle, fail(), failed()),
            (S::Recording, fail(), failed()),
            (S::Paused, fail(), failed()),
            (S::Stopping, fail(), failed()),
        ];
        for (from, cmd, expected) in cases {
            let got = from.clone().transition(cmd.clone());
            assert_eq!(got, Ok(expected), "{from:?} + {cmd:?}");
        }
    }

    #[test]
    fn invalid_transitions() {
        let cases = [
            (S::Idle, C::Pause),
            (S::Idle, C::Resume),
            (S::Idle, C::Stop),
            (S::Idle, C::Finish),
            (S::Recording, C::Start),
            (S::Recording, C::Resume),
            (S::Recording, C::Finish),
            (S::Paused, C::Start),
            (S::Paused, C::Pause),
            (S::Paused, C::Finish),
            (S::Stopping, C::Start),
            (S::Stopping, C::Pause),
            (S::Stopping, C::Resume),
            (S::Stopping, C::Stop),
            (S::Stopped, C::Start),
            (S::Stopped, C::Stop),
            (S::Stopped, C::Finish),
            (S::Stopped, fail()),
            (failed(), C::Start),
            (failed(), C::Resume),
            (failed(), fail()),
        ];
        for (from, cmd) in cases {
            let got = from.clone().transition(cmd.clone());
            assert_eq!(
                got,
                Err(TransitionError {
                    from: from.clone(),
                    command: cmd.clone(),
                }),
                "{from:?} + {cmd:?} should be rejected"
            );
        }
    }

    #[test]
    fn full_lifecycle() {
        let state = [C::Start, C::Pause, C::Resume, C::Stop, C::Finish]
            .into_iter()
            .try_fold(S::Idle, S::transition);
        assert_eq!(state, Ok(S::Stopped));
    }

    #[test]
    fn error_message_names_state_and_command() {
        let err = S::Stopped.transition(C::Start).unwrap_err();
        assert_eq!(
            err.to_string(),
            "cannot apply Start to a session in state Stopped"
        );
    }

    #[test]
    fn terminal_states() {
        assert!(S::Stopped.is_terminal());
        assert!(failed().is_terminal());
        for state in [S::Idle, S::Recording, S::Paused, S::Stopping] {
            assert!(!state.is_terminal(), "{state:?}");
        }
    }
}
