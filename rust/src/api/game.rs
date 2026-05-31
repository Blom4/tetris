use crate::core::bag::Bag;
use crate::core::board::new_grid;
use crate::core::game::GameState;
use crate::core::types::*;

/// FRB-friendly snapshot of the game state.
/// Grid is flat row-major Vec (200 elements): index = y * WIDTH + x
pub struct GameStateView {
    pub grid: Vec<CellType>,
    pub current_piece: CellType,
    pub current_rotation: i32,
    pub current_x: i32,
    pub current_y: i32,
    pub ghost_x: i32,
    pub ghost_y: i32,
    pub hold_piece: Option<CellType>,
    pub can_hold: bool,
    pub next_queue: Vec<CellType>,
    pub score: u32,
    pub level: u32,
    pub lines: u32,
    pub game_over: bool,
    pub bag_queue: Vec<CellType>,
    pub bag_rng: i64,
}

fn flatten_grid(grid: &Grid) -> Vec<CellType> {
    let mut flat = Vec::with_capacity(WIDTH * HEIGHT);
    for row in grid.iter() {
        flat.extend_from_slice(row);
    }
    flat
}

fn unflatten_grid(flat: &[CellType]) -> Grid {
    let mut grid = new_grid();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            grid[y][x] = flat[y * WIDTH + x];
        }
    }
    grid
}

impl From<GameState> for GameStateView {
    fn from(gs: GameState) -> Self {
        GameStateView {
            grid: flatten_grid(&gs.grid),
            current_piece: gs.current_piece,
            current_rotation: gs.current_rotation as i32,
            current_x: gs.current_pos.x,
            current_y: gs.current_pos.y,
            ghost_x: gs.ghost_pos.x,
            ghost_y: gs.ghost_pos.y,
            hold_piece: gs.hold_piece,
            can_hold: gs.can_hold,
            next_queue: gs.next_queue,
            score: gs.score,
            level: gs.level,
            lines: gs.lines,
            game_over: gs.game_over,
            bag_queue: gs.bag.remaining().to_vec(),
            bag_rng: gs.bag.rng_seed() as i64,
        }
    }
}

impl From<GameStateView> for GameState {
    fn from(v: GameStateView) -> Self {
        GameState {
            grid: unflatten_grid(&v.grid),
            current_piece: v.current_piece,
            current_rotation: v.current_rotation as usize,
            current_pos: Position {
                x: v.current_x,
                y: v.current_y,
            },
            ghost_pos: Position {
                x: v.ghost_x,
                y: v.ghost_y,
            },
            hold_piece: v.hold_piece,
            can_hold: v.can_hold,
            next_queue: v.next_queue,
            score: v.score,
            level: v.level,
            lines: v.lines,
            game_over: v.game_over,
            bag: Bag::from_parts(v.bag_queue, v.bag_rng as u64),
        }
    }
}

fn update(gsv: GameStateView, f: impl FnOnce(&mut GameState)) -> GameStateView {
    let mut gs: GameState = gsv.into();
    f(&mut gs);
    gs.into()
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_init() -> GameStateView {
    GameState::new().into()
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_tick(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| gs.tick())
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_move_left(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| { gs.move_left(); })
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_move_right(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| { gs.move_right(); })
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_move_down(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| { gs.move_down(); })
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_rotate_cw(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| { gs.rotate_cw(); })
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_rotate_ccw(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| { gs.rotate_ccw(); })
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_soft_drop(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| { gs.soft_drop(); })
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_hard_drop(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| { gs.hard_drop(); })
}

#[flutter_rust_bridge::frb(sync)]
pub fn game_hold(gsv: GameStateView) -> GameStateView {
    update(gsv, |gs| { gs.hold(); })
}
