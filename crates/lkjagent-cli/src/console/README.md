# Console Source

## Purpose

This directory holds the terminal console rendering code used by
`lkjagent console`.

## Table of Contents

- [render.rs](render.rs): transcript-first screen with bottom status deck.
- [size.rs](size.rs): live terminal size resolution and minimum clamps.
- [display.rs](display.rs): terminal display-width wrapping and truncation.
- [event_view.rs](event_view.rs): transcript and last-output formatting.
- [input.rs](input.rs): line and terminal input adapters for the console loop.
- [style.rs](style.rs): ANSI terminal styling for the console screen.
- [terminal_input.rs](terminal_input.rs): terminal mode and character input reader.
