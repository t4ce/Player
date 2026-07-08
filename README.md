# player-tui

`player-tui` is a Ratatui prototype for an audio player interface. It focuses on
layout, command parsing, status feedback, playlist browsing, and scope-style
visual demos. Media playback, recording, saving, and transcription are currently
visual/logged flows only.

## Run

```sh
cargo run
```

## Command API

The `control` module is the integration surface for future backend logic:

- `parse_command` turns prompt text into a `ParsedCommand`.
- `dispatch` routes parsed commands into a `ControlEventHandler`.
- `COMMAND_SPECS` lists the supported commands and aliases.

The binary implements `ControlEventHandler` with demo behavior today. A real
backend can implement the same trait to connect playback, recording, saving,
playlist, and scope logic later.

## Local docs

This repository keeps generated Rust documentation in `doc/` for quick local
inspection:

```sh
cargo doc --no-deps --document-private-items
```
