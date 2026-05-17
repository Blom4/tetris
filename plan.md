# Tetris — Implementation Plan

```
┌──────────────┐   input events (keyboard/touch)   ┌──────────────┐
│   Flutter    │ ───────────────────────────────►   │    Rust      │
│   UI Layer   │                                    │  Game Engine │
│  (rendering) │ ◄───────────────────────────────  │  (logic)     │
└──────────────┘       GameState snapshot           └──────────────┘
```

Flutter runs the game loop (`Ticker`) and sends inputs to Rust. Rust returns updated `GameState` — no mutable singletons, pure stateless functions.

---

## Phase 1: Rust Core — Board & Pieces

**Files:** `core/board.rs`, `core/pieces.rs`, `core/types.rs` (all new, under `rust/src/`)

- `CellType` enum, `Position`/`Board` structs
- Grid collision detection, line clearing
- All 7 tetrominoes with SRS rotation states (4 per piece)
- SRS wall kick tables (JLSTZ + I)
- **Verification:** `cargo test` with unit tests for rotations, collision, line clearing

---

## Phase 2: Rust Game Logic

**Files:** `core/game.rs` (new)

- `GameState` struct (board, current piece, ghost piece, next queue, hold,
  score, level, lines, game_over)
- Piece spawn + lock mechanics
- Ghost piece projection
- Move left/right, rotate CW/CCW, soft drop, hard drop
- Tick (gravity), hold piece
- **Verification:** `cargo test` — simulate full games, verify state transitions

---

## Phase 3: Rust Meta-Game

**Files:** `core/bag.rs`, `core/scoring.rs` (new)

- 7-bag randomizer with next-piece preview queue
- Guideline scoring (Single/Double/Triple/Tetris, T-Spin 3-corner rule,
  soft/hard drop bonuses)
- Level progression (every 10 lines → speed increases)
- Game over detection
- **Verification:** `cargo test` — score edge cases, bag distribution, level speeds

---

## Phase 4: Bridge + Flutter Skeleton

**Files:** `api/game.rs` (new), `lib/main.dart` (update),
`lib/theme/tetris_theme.dart`, `lib/screens/game_screen.dart`,
`lib/widgets/board_widget.dart`, `lib/controllers/game_controller.dart`

- Flutter-annotated API functions in `api/game.rs`
- Run `flutter_rust_bridge_codegen generate`
- Piece color theme (classic NES / modern palette)
- `BoardWidget` renders the 10×20 grid from `GameState`
- `GameController` with `Ticker`-based loop calling Rust `tick()`
- `GameScreen` assembles board and starts loop
- **Verification:** App launches, board renders, pieces fall automatically

---

## Phase 5: Flutter Complete UI

**Files:** `lib/widgets/next_piece_widget.dart`,
`lib/widgets/hold_piece_widget.dart`, `lib/widgets/score_panel.dart`

- Next piece preview (shows next 3–5 pieces from queue)
- Hold piece widget with indicator
- Score / level / lines cleared display
- Game over overlay with restart option
- **Verification:** All UI elements visible and updating live

---

## Phase 6: Input + Polish

**Files:** `lib/screens/game_screen.dart` (update)

| Platform | Controls |
|----------|----------|
| Desktop | Arrow keys (L/R/D), Z (CCW), X/Up (CW), Space (hard drop), Shift/C (hold) |
| Mobile/Web | On-screen D-pad + action buttons (or swipe gestures) |

- Keyboard event handlers for desktop
- Touch-friendly control overlay (or swipe: left/right/tap/down)
- Refinements: animations, sound, visual polish
- **Verification:** Full playable game on all target platforms
