# Contributing to fastgraph

## Getting Started

1. Fork the repository
2. Create a branch from `main`
3. Run `cargo test` and `cargo clippy` before submitting
4. Open a PR against `main`

## Code Style

We use `rustfmt` with the project's `rustfmt.toml`. Run `cargo fmt` before committing.

## Commit Messages

Use Conventional Commits: `fix:`, `feat:`, `docs:`, `refactor:`.
Include the issue number in the commit body when applicable.

## Review

All PRs require at least one approval from a maintainer. Core subsystem changes
(storage engine, query planner) require two approvals.

See our Code of Conduct for community standards, including our policy on AI-generated contributions.
