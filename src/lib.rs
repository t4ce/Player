//! Library surface for `player-tui`.
//!
//! The binary is the visual Ratatui demo. The library exposes the command
//! parser and dispatch trait so playback, playlist, recording, and editor
//! behavior can be wired behind the UI without coupling that logic to drawing.

pub mod control;
