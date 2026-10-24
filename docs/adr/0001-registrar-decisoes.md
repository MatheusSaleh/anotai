# ADR 0001: Record architecture decisions

- Status: Accepted
- Date: 2026-10-23

## Context

AnotAI is built in the open, by a small team and outside contributors, over several releases. Decisions such as "local transcription", "SQLite first" or "ts-rs for shared types" shape the code for a long time. Without a written record, the reasons get lost, the same debates come back, and newcomers cannot tell a deliberate constraint from an accident.

## Decision

We record significant decisions as Architecture Decision Records (ADRs) in `docs/adr/`, following Michael Nygard's lightweight format.

- One decision per file, named `NNNN-short-title.md`, numbered in order.
- Each ADR has: status, date, context, decision, consequences. Options considered go in the context when relevant.
- At most one page. If it needs more, the decision is probably several decisions.
- An ADR is written when a change picks a library, a storage format, a protocol or an architectural pattern, or when it changes one of the privacy rules.
- Accepted ADRs are not edited. A new ADR supersedes an old one, and the old one's status becomes `Superseded by ADR NNNN`.
- ADRs are written in English, like the rest of the repository.

## Consequences

- Reviewers can ask for an ADR before merging a structural change.
- Decisions become reviewable in pull requests, like code.
- There is a small writing cost per decision, kept low by the one-page limit.
