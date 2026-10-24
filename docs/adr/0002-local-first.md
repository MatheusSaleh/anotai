# ADR 0002: Local-first, with an optional server

- Status: Accepted
- Date: 2026-10-23

## Context

AnotAI captures the audio of a class and turns it into a transcript and, later, study notes. That audio may contain the voices of teachers and classmates who never agreed to be sent to a third party. Students also often study with unreliable connections and cannot pay for cloud speech-to-text.

Options considered:

1. **Cloud first:** upload audio to a backend that runs speech-to-text and stores everything. Simplest to build, but audio leaves the device, it needs accounts and paid infrastructure, and it stops working offline.
2. **Local first, optional server:** capture, transcription and storage run on the student's computer. A server exists only to sync notebooks between devices, and only if the user sets it up.
3. **Local only:** like option 2 but with no sync at all. Simplest, but users with more than one device lose their notes.

## Decision

We choose option 2.

- Audio capture, resampling, voice activity detection and transcription (whisper.cpp) run on the user's machine.
- Raw audio lives only in bounded in-memory buffers. It is never written to disk or sent over the network.
- Transcripts and notes are stored locally in SQLite, which is the source of truth.
- In v0.1, the only network traffic is the download of Whisper models.
- AI notes (v0.2) send transcript text only, to a provider the user configures, or to a local model.
- The sync server (v0.4) is self-hostable and optional. The app is fully usable without it.

## Consequences

- Privacy is a property of the architecture, not a policy promise, which is easier to audit in an open-source codebase.
- The app works offline and costs nothing to run.
- Transcription speed and quality depend on the user's hardware, so we publish benchmarks (`bench/`) and support several model sizes.
- Installers must ship or download native code and models, which makes builds and releases heavier.
- Sync has to handle conflicts between devices instead of relying on a single central database.
