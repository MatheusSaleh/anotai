# Contributing to AnotAI

Thanks for your interest in AnotAI! Contributions of every size are welcome, whether a typo fix, a bug report, benchmark results from your hardware or a new feature.

🇧🇷 Contribuições em português também são bem-vindas: issues e pull requests podem ser escritos em português ou em inglês.

## Ways to contribute

- **Report a bug:** open an issue with the steps to reproduce it, what you expected, what happened, and your Windows version, CPU/GPU and Whisper model.
- **Suggest a feature:** open an issue describing the problem it solves before writing code, so we can agree on the approach first.
- **Pick an issue:** issues labeled [`good first issue`](https://github.com/MatheusSaleh/anotai/labels/good%20first%20issue) are scoped for newcomers, and [`help wanted`](https://github.com/MatheusSaleh/anotai/labels/help%20wanted) ones are open for anyone. Comment on the issue to claim it.
- **Share benchmark results:** run the suite in [`bench/`](bench/) on your machine and open a PR with the results. Data from different hardware is very valuable.
- **Improve the docs:** README, ADRs, code comments and translations all count.

## Development setup

Follow [Build from source](README.md#build-from-source-windows) in the README. In short:

```bash
git clone https://github.com/MatheusSaleh/anotai.git
cd anotai
cargo test --workspace
cd apps/desktop && pnpm install && pnpm tauri dev
```

The desktop app must be developed on Windows, because audio capture uses WASAPI. The crates in `crates/`, the `anotai` CLI and the sync server also build on Linux and macOS, including WSL.

Crate layout: `crates/core` (`anotai-core`), `crates/audio` (`anotai-audio`), `crates/transcribe` (`anotai-transcribe`), `crates/session` (`anotai-session`), `crates/storage` (`anotai-storage`) and `apps/cli` (`anotai-cli`, binary `anotai`). See [Project structure](README.md#project-structure).

## Workflow

1. Fork the repository and create a branch from `main`:
   `feat/live-partials`, `fix/vad-cuts-first-word`, `docs/adr-sqlite`.
2. Make focused changes. One pull request = one logical change.
3. Write commits following [Conventional Commits](https://www.conventionalcommits.org):
   ```
   feat(transcribe): stream partial results to the UI
   fix(audio): avoid clicks when resampling 48 kHz to 16 kHz
   docs(adr): record SQLite as local storage
   ```
4. Open a pull request against `main` and fill in the template. Link the issue it closes (`Closes #42`).

## Before you open a pull request

Run the same checks the CI runs:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd apps/desktop && pnpm lint && pnpm typecheck && pnpm test
```

Checklist:

- [ ] Code is formatted and passes clippy and the linters with no warnings
- [ ] New behavior is covered by tests, and existing tests still pass
- [ ] Public APIs and non-obvious decisions are documented
- [ ] Rust types shared with the UI were regenerated (`cargo test -p anotai-core export_bindings`)
- [ ] `CHANGELOG.md` has an entry under **Unreleased** for user-facing changes
- [ ] The change follows the privacy rules below

## Code guidelines

- **Keep the core independent.** Crates in `crates/` must not depend on Tauri, React or the sync server. Put UI and transport concerns in `apps/`.
- **Never block the async runtime.** Whisper inference and other CPU-heavy work run on a dedicated thread or in `spawn_blocking`.
- **Never hold a lock across `.await`.** Share state with `Arc` and message passing where possible.
- **Errors are typed.** Libraries use `thiserror`, while applications may use `anyhow` at the edges. Avoid `unwrap()` outside tests.
- **Serialization goes through `serde`** with `#[serde(rename_all = "camelCase")]` for anything the UI consumes, and `#[serde(deny_unknown_fields)]` on inputs.
- **Significant decisions get an ADR.** If your change picks a library, a storage format or an architectural pattern, add a short record in [`docs/adr/`](docs/adr/) explaining the context, the options and the trade-offs.

## Privacy rules

Privacy is AnotAI's core promise. Pull requests that break these rules will not be merged:

1. **Raw audio is never written to disk or sent over the network.** It lives only in bounded in-memory buffers.
2. **Capture never starts without an explicit user action**, and the recording indicator stays visible for the whole session.
3. **Logs, telemetry and crash reports never contain audio, transcripts, note content or API keys.**
4. **Only transcript text goes to an AI provider**, and only to the one the user configured.
5. **No test fixtures or benchmark data from real classes.** Use openly licensed datasets such as Mozilla Common Voice.

## Reporting security issues

Please **do not** open public issues for security vulnerabilities. Report them privately through [GitHub Security Advisories](https://github.com/MatheusSaleh/anotai/security/advisories/new).

## License

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in AnotAI by you, as defined in the Apache-2.0 license, shall be dual licensed under the MIT and Apache-2.0 licenses, without any additional terms or conditions.
