# Tetris — AGENTS.md

## Stack
- Flutter (Dart) UI + Rust game engine via `flutter_rust_bridge` v2.12.0
- Flutter entrypoint: `lib/main.dart`
- Rust crate: `rust_lib_tetris` (`rust/`), library not binary

## Bridge setup
- Rust API exposed to Dart lives in `rust/src/api/` — this is the bridge boundary
- Dart bridge code auto-generated in `lib/src/rust/` — **do not edit manually**
- Config: `flutter_rust_bridge.yaml` — `rust_input: crate::api`, `rust_root: rust/`, `dart_output: lib/src/rust`
- Init: `RustLib.init()` must be called before any bridge calls (already in `main()`)
- Sync calls: annotate with `#[flutter_rust_bridge::frb(sync)]`
- Init fn: annotate with `#[flutter_rust_bridge::frb(init)]`

## Commands
| Action | Command | Workspace |
|--------|---------|-----------|
| Codegen | `flutter_rust_bridge_codegen generate` | repo root |
| Run app | `flutter run` | repo root |
| Rust tests | `cargo test` | `rust/` |
| Dart tests | `flutter test` | repo root |
| Integration | `flutter test integration_test/` | repo root |
| Dart analyze | `flutter analyze` | repo root |
| Rust lint | `cargo clippy` | `rust/` |

## Architecture (from plan.md)
- **Rust**: game logic in `rust/src/core/` (board, pieces, SRS, bag, scoring, game state) — pure stateless functions
- **Rust**: bridge API layer in `rust/src/api/game.rs` — thin wrappers with `#[frb]` annotations
- **Flutter**: UI in `lib/screens/`, `lib/widgets/`, `lib/controllers/`, `lib/theme/`
- Game loop runs in Flutter (`Ticker`), calls Rust functions with `GameState` in/out
- SRS rotation, 7-bag randomizer, ghost piece, hold, T-spin detection

## Test conventions
- Rust unit tests: alongside code in `core/` modules or `tests/` directory
- Dart unit tests: `test/` directory
- Integration tests: `integration_test/` directory
- Integration tests require `RustLib.init()` in `setUpAll`

## Gotchas
- Rust crate has no binary — only `cdylib` + `staticlib` (FFI lib). Use `flutter run`, not `cargo run`
- `rust_builder/` is a Flutter plugin that handles cross-compilation via `cargokit/`
- After adding new Rust API functions, run codegen before testing Flutter side
- Piece `core/` modules need `pub mod` declarations in `core/mod.rs` + `api/mod.rs`
