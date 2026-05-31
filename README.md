# Tetris

A Tetris game built with Flutter (Dart) UI and a Rust game engine, connected via `flutter_rust_bridge`.

## Stack

- **Flutter** — UI layer (screens, widgets, touch controls)
- **Rust** — game logic (board, pieces, SRS rotation, 7-bag randomizer, T-spin detection, scoring)
- **flutter_rust_bridge v2.12.0** — FFI bridge

## Rust architecture

- `rust/src/core/` — pure stateless game functions (board, pieces, SRS wall kicks, bag, scoring, game state)
- `rust/src/api/` — thin bridge API layer with `#[flutter_rust_bridge::frb(sync)]` annotations
- Dart bridge code auto-generated in `lib/src/rust/` — do not edit manually

## Commands

| Action | Command |
|--------|---------|
| Codegen | `flutter_rust_bridge_codegen generate` |
| Run | `flutter run` |
| Rust tests | `cargo test` (in `rust/`) |
| Dart tests | `flutter test` |
| Dart analyze | `flutter analyze` |
| Rust lint | `cargo clippy` (in `rust/`) |
| Android APK | `flutter build apk --debug` |

## Features

- SRS (Super Rotation System) with full wall kick tables
- 7-bag randomizer (Fisher-Yates shuffle + LCG)
- Ghost piece, hold, next queue
- T-spin detection (3-corner rule)
- Guideline scoring (line clears, T-spin bonuses, drop bonuses, level progression)
- Touch controls with DAS auto-repeat
- Responsive portrait/landscape layout
