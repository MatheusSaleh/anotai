use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::SessionState;

/// Unique session identifier: a UUID v7, so ids sort by creation time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SessionId(Uuid);

impl SessionId {
    /// Creates a new time-ordered id from the current system time.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for SessionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Where the audio of a session comes from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[ts(export)]
pub enum AudioSourceKind {
    /// Loopback capture of what the computer is playing.
    SystemOutput {
        device_id: String,
    },
    Microphone {
        device_id: String,
    },
}

/// A recording session and its metadata. The transcript lives in [`Segment`]s.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Session {
    pub id: SessionId,
    pub title: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub state: SessionState,
    pub source: AudioSourceKind,
    /// Name of the Whisper model used for the transcription.
    pub model: String,
}

/// A confirmed piece of transcript. Partial hypotheses use [`PartialText`] instead.
///
/// Times are milliseconds from the start of the session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Segment {
    pub session_id: SessionId,
    /// Position of the segment in the session, starting at 0.
    pub seq: u32,
    #[ts(type = "number")]
    pub start_ms: u64,
    #[ts(type = "number")]
    pub end_ms: u64,
    pub text: String,
}

/// A transcript hypothesis that may still change before it becomes a [`Segment`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PartialText {
    pub session_id: SessionId,
    pub text: String,
    #[ts(type = "number")]
    pub start_ms: u64,
}

/// Events emitted while a session runs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[ts(export)]
pub enum SessionEvent {
    StateChanged {
        state: SessionState,
    },
    Partial(PartialText),
    SegmentFinal(Segment),
    /// Root mean square of the latest audio block, from 0.0 to 1.0.
    AudioLevel {
        rms: f32,
    },
    /// Audio discarded because a bounded buffer was full.
    AudioDropped {
        #[ts(type = "number")]
        ms: u64,
    },
    /// A non-fatal error. Never contains audio or transcript text.
    Error {
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use serde::de::DeserializeOwned;

    use super::*;

    fn round_trip<T>(value: &T) -> T
    where
        T: Serialize + DeserializeOwned,
    {
        let json = serde_json::to_string(value).unwrap();
        serde_json::from_str(&json).unwrap()
    }

    fn session_id() -> SessionId {
        SessionId::from(Uuid::parse_str("0192f0c8-6a3e-7b1c-9d2e-3f4a5b6c7d8e").unwrap())
    }

    fn segment() -> Segment {
        Segment {
            session_id: session_id(),
            seq: 3,
            start_ms: 61_500,
            end_ms: 64_250,
            text: "Synthetic test sentence.".into(),
        }
    }

    #[test]
    fn session_ids_are_v7_and_time_ordered() {
        let first = SessionId::new();
        let second = SessionId::new();
        assert_eq!(first.as_uuid().get_version_num(), 7);
        assert!(first < second);
    }

    #[test]
    fn session_round_trips_through_json() {
        let session = Session {
            id: session_id(),
            title: "Linear algebra, lecture 4".into(),
            started_at: Utc.with_ymd_and_hms(2026, 10, 23, 19, 0, 0).unwrap(),
            ended_at: Some(Utc.with_ymd_and_hms(2026, 10, 23, 20, 30, 0).unwrap()),
            state: SessionState::Stopped,
            source: AudioSourceKind::SystemOutput {
                device_id: "default".into(),
            },
            model: "ggml-small-q5_1".into(),
        };
        assert_eq!(round_trip(&session), session);
    }

    #[test]
    fn segment_round_trips_through_json() {
        assert_eq!(round_trip(&segment()), segment());
    }

    #[test]
    fn every_session_event_round_trips_through_json() {
        let events = [
            SessionEvent::StateChanged {
                state: SessionState::Failed {
                    reason: "device disconnected".into(),
                },
            },
            SessionEvent::Partial(PartialText {
                session_id: session_id(),
                text: "Synthetic partial".into(),
                start_ms: 1_000,
            }),
            SessionEvent::SegmentFinal(segment()),
            SessionEvent::AudioLevel { rms: 0.25 },
            SessionEvent::AudioDropped { ms: 120 },
            SessionEvent::Error {
                message: "model not found".into(),
            },
        ];
        for event in events {
            assert_eq!(round_trip(&event), event);
        }
    }

    #[test]
    fn json_uses_camel_case_and_a_type_tag() {
        let json = serde_json::to_value(SessionEvent::SegmentFinal(segment())).unwrap();
        assert_eq!(json["type"], "segmentFinal");
        assert_eq!(json["sessionId"], "0192f0c8-6a3e-7b1c-9d2e-3f4a5b6c7d8e");
        assert_eq!(json["startMs"], 61_500);

        let json = serde_json::to_value(AudioSourceKind::Microphone {
            device_id: "mic-1".into(),
        })
        .unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "type": "microphone", "deviceId": "mic-1" })
        );
    }
}
