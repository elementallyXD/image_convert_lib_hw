# Contributing

## Prerequisites
- Rust toolchain (stable) with `cargo`
- `cargo fmt` and `cargo clippy` available via `rustfmt`/`clippy` components

## Setup
- Clone the repo and `cd` into it.
- Ensure sample fixtures exist under `examples/` (PNG/JPEG) if you add new image cases.

## Workflow
1. Format: `cargo fmt`
2. Lint: `cargo clippy --all-targets --all-features`
3. Test: `cargo test` (integration tests read from `examples/`)
4. If adding fixtures, prefer small files and note any format quirks in the test names/comments.

## Code guidelines
- Keep typestate flow intact: `Loaded -> Raw (RGBA8) -> Encoded`.
- Extend `UserFormat` instead of exposing `image::ImageFormat`.
- Use `get_and_reset` consistently to return data and restore a raw state.
- Add concise comments only when behavior isn’t obvious.

## Pull requests
- Describe the change and its rationale.
- Mention any new fixtures or dependencies.
- Include test output (`cargo test`, plus clippy/fmt) if possible.
