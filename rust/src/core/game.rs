use super::bag::Bag;
use super::board::*;
use super::pieces::*;
use super::scoring::*;
use super::types::*;

const BAG_SEED: u64 = 0x123456789abcdef;

pub struct GameState {
    pub grid: Grid,
    pub current_piece: CellType,
    pub current_rotation: usize,
    pub current_pos: Position,
    pub ghost_pos: Position,
    pub hold_piece: Option<CellType>,
    pub can_hold: bool,
    pub next_queue: Vec<CellType>,
    pub score: u32,
    pub level: u32,
    pub lines: u32,
    pub game_over: bool,
    pub bag: Bag,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        let grid = new_grid();
        let mut bag = Bag::new(BAG_SEED);
        let piece = bag.next();
        let next_queue = bag.peek(5);
        let pos = spawn_pos(piece);
        let ghost = ghost_pos(&grid, piece, 0, pos);

        GameState {
            grid,
            current_piece: piece,
            current_rotation: 0,
            current_pos: pos,
            ghost_pos: ghost,
            hold_piece: None,
            can_hold: true,
            next_queue,
            score: 0,
            level: 1,
            lines: 0,
            game_over: false,
            bag,
        }
    }

    fn update_ghost(&mut self) {
        self.ghost_pos =
            ghost_pos(&self.grid, self.current_piece, self.current_rotation, self.current_pos);
    }

    fn lock_piece(&mut self) {
        let t_spin = detect_t_spin(&self.grid, self.current_piece, self.current_pos);
        place(
            &mut self.grid,
            self.current_piece,
            self.current_rotation,
            self.current_pos,
        );
        let cleared = clear_lines(&mut self.grid);
        if cleared > 0 {
            let n = cleared as u32;
            self.lines += n;
            self.score += line_clear_score(n, self.level, t_spin);
            self.level = level_from_lines(self.lines);
        }
        self.spawn_piece();
    }

    fn spawn_piece(&mut self) {
        let piece = self.bag.next();
        self.next_queue = self.bag.peek(5);
        let pos = spawn_pos(piece);
        if collides(&self.grid, piece, 0, pos) {
            self.game_over = true;
            return;
        }
        self.current_piece = piece;
        self.current_rotation = 0;
        self.current_pos = pos;
        self.can_hold = true;
        self.update_ghost();
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }
        let below = Position {
            x: self.current_pos.x,
            y: self.current_pos.y + 1,
        };
        if can_place(&self.grid, self.current_piece, self.current_rotation, below) {
            self.current_pos = below;
            self.update_ghost();
        } else {
            self.lock_piece();
        }
    }

    pub fn move_left(&mut self) -> bool {
        if self.game_over {
            return false;
        }
        let new_pos = Position {
            x: self.current_pos.x - 1,
            y: self.current_pos.y,
        };
        if can_place(&self.grid, self.current_piece, self.current_rotation, new_pos) {
            self.current_pos = new_pos;
            self.update_ghost();
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self) -> bool {
        if self.game_over {
            return false;
        }
        let new_pos = Position {
            x: self.current_pos.x + 1,
            y: self.current_pos.y,
        };
        if can_place(&self.grid, self.current_piece, self.current_rotation, new_pos) {
            self.current_pos = new_pos;
            self.update_ghost();
            true
        } else {
            false
        }
    }

    pub fn move_down(&mut self) -> bool {
        if self.game_over {
            return false;
        }
        let below = Position {
            x: self.current_pos.x,
            y: self.current_pos.y + 1,
        };
        if can_place(&self.grid, self.current_piece, self.current_rotation, below) {
            self.current_pos = below;
            self.update_ghost();
            true
        } else {
            false
        }
    }

    pub fn rotate_cw(&mut self) -> bool {
        if self.game_over {
            return false;
        }
        if let Some((new_rot, new_pos)) = try_rotate(
            &self.grid,
            self.current_piece,
            self.current_rotation,
            self.current_pos,
            true,
        ) {
            self.current_rotation = new_rot;
            self.current_pos = new_pos;
            self.update_ghost();
            true
        } else {
            false
        }
    }

    pub fn rotate_ccw(&mut self) -> bool {
        if self.game_over {
            return false;
        }
        if let Some((new_rot, new_pos)) = try_rotate(
            &self.grid,
            self.current_piece,
            self.current_rotation,
            self.current_pos,
            false,
        ) {
            self.current_rotation = new_rot;
            self.current_pos = new_pos;
            self.update_ghost();
            true
        } else {
            false
        }
    }

    pub fn soft_drop(&mut self) -> u32 {
        if self.game_over {
            return 0;
        }
        let below = Position {
            x: self.current_pos.x,
            y: self.current_pos.y + 1,
        };
        if can_place(&self.grid, self.current_piece, self.current_rotation, below) {
            self.current_pos = below;
            self.score += soft_drop_score(1);
            self.update_ghost();
            1
        } else {
            0
        }
    }

    pub fn hard_drop(&mut self) -> u32 {
        if self.game_over {
            return 0;
        }
        let distance = self.ghost_pos.y - self.current_pos.y;
        self.current_pos = self.ghost_pos;
        let pts = hard_drop_score(distance as u32);
        self.score += pts;
        self.lock_piece();
        pts
    }

    pub fn hold(&mut self) -> bool {
        if self.game_over || !self.can_hold {
            return false;
        }
        let current = self.current_piece;
        match self.hold_piece {
            Some(held) => {
                self.hold_piece = Some(current);
                self.current_piece = held;
                self.current_rotation = 0;
                self.current_pos = spawn_pos(held);
                if collides(&self.grid, held, 0, self.current_pos) {
                    self.game_over = true;
                    return false;
                }
                self.update_ghost();
            }
            None => {
                self.hold_piece = Some(current);
                let next = self.bag.next();
                self.next_queue = self.bag.peek(5);
                self.current_piece = next;
                self.current_rotation = 0;
                self.current_pos = spawn_pos(next);
                self.update_ghost();
            }
        }
        self.can_hold = false;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> GameState {
        GameState::new()
    }

    #[test]
    fn test_new_game_state() {
        let gs = setup();
        assert_eq!(gs.score, 0);
        assert_eq!(gs.level, 1);
        assert_eq!(gs.lines, 0);
        assert!(!gs.game_over);
        assert_eq!(gs.hold_piece, None);
        assert!(gs.can_hold);
        assert_eq!(gs.next_queue.len(), 5);
        assert_ne!(gs.current_piece, CellType::Empty);
        assert_eq!(gs.current_rotation, 0);
        assert_eq!(
            gs.ghost_pos,
            ghost_pos(&gs.grid, gs.current_piece, gs.current_rotation, gs.current_pos)
        );
    }

    #[test]
    fn test_move_left() {
        let mut gs = setup();
        let x = gs.current_pos.x;
        assert!(gs.move_left());
        assert_eq!(gs.current_pos.x, x - 1);
    }

    #[test]
    fn test_full_line_clear_via_game() {
        let mut gs = setup();
        for x in 0..WIDTH {
            gs.grid[HEIGHT - 1][x] = CellType::I;
        }
        gs.lock_piece();
        assert_eq!(gs.lines, 1);
        assert_eq!(gs.score, 100);
        for x in 0..WIDTH {
            assert_eq!(gs.grid[HEIGHT - 1][x], CellType::Empty);
        }
    }

    #[test]
    fn test_move_right() {
        let mut gs = setup();
        let x = gs.current_pos.x;
        assert!(gs.move_right());
        assert_eq!(gs.current_pos.x, x + 1);
    }

    #[test]
    fn test_move_right_blocked_by_wall() {
        let mut gs = setup();
        while gs.move_right() {}
        assert!(!gs.move_right());
    }

    #[test]
    fn test_move_down() {
        let mut gs = setup();
        let y = gs.current_pos.y;
        assert!(gs.move_down());
        assert_eq!(gs.current_pos.y, y + 1);
    }

    #[test]
    fn test_move_down_blocked() {
        let mut gs = setup();
        while gs.current_pos.y < 18 {
            gs.move_down();
        }
        assert!(!gs.move_down());
    }

    #[test]
    fn test_rotate_cw() {
        let mut gs = setup();
        assert!(gs.rotate_cw());
        assert_eq!(gs.current_rotation, 1);
    }

    #[test]
    fn test_rotate_ccw() {
        let mut gs = setup();
        assert!(gs.rotate_ccw());
        assert_eq!(gs.current_rotation, 3);
    }

    #[test]
    fn test_rotate_cycles() {
        let mut gs = setup();
        for _ in 0..4 {
            gs.rotate_cw();
        }
        assert_eq!(gs.current_rotation, 0);
    }

    #[test]
    fn test_soft_drop_scores() {
        let mut gs = setup();
        let y = gs.current_pos.y;
        let s = gs.score;
        assert!(gs.soft_drop() > 0);
        assert_eq!(gs.current_pos.y, y + 1);
        assert_eq!(gs.score, s + 1);
    }

    #[test]
    fn test_soft_drop_blocked() {
        let mut gs = setup();
        while gs.move_down() {}
        assert_eq!(gs.soft_drop(), 0);
    }

    #[test]
    fn test_hard_drop_locks_and_spawns() {
        let mut gs = setup();
        let old_piece = gs.current_piece;
        let old_pos = gs.current_pos;
        let points = gs.hard_drop();
        assert!(points > 0);
        assert_ne!(gs.current_piece, old_piece);
        assert!(gs.current_pos != old_pos);
    }

    #[test]
    fn test_hold_swaps_piece() {
        let mut gs = setup();
        let first = gs.current_piece;
        assert!(gs.hold());
        assert_eq!(gs.hold_piece, Some(first));
        assert_ne!(gs.current_piece, first);
    }

    #[test]
    fn test_hold_twice_blocked() {
        let mut gs = setup();
        assert!(gs.hold());
        assert!(!gs.hold());
    }

    #[test]
    fn test_hold_resets_after_lock() {
        let mut gs = setup();
        assert!(gs.hold());
        assert!(!gs.hold());
        gs.hard_drop();
        assert!(gs.can_hold);
    }

    #[test]
    fn test_tick_moves_down() {
        let mut gs = setup();
        let y = gs.current_pos.y;
        gs.tick();
        assert_eq!(gs.current_pos.y, y + 1);
    }

    #[test]
    fn test_tick_locks_at_bottom() {
        let mut gs = setup();
        while !gs.game_over {
            gs.tick();
        }
        assert!(gs.game_over);
    }

    #[test]
    fn test_game_over_detected() {
        let mut gs = setup();
        for y in 0..2 {
            for x in 0..WIDTH {
                gs.grid[y][x] = CellType::I;
            }
        }
        gs.spawn_piece();
        assert!(gs.game_over);
    }

    #[test]
    fn test_line_clear_scores() {
        let mut gs = setup();
        for x in 0..WIDTH {
            gs.grid[HEIGHT - 1][x] = CellType::I;
        }
        let cleared = clear_lines(&mut gs.grid);
        assert_eq!(cleared, 1);
        gs.lines += cleared as u32;
        gs.score += line_clear_score(cleared as u32, gs.level, false);
        assert_eq!(gs.score, 100);
        assert_eq!(gs.lines, 1);
    }

    #[test]
    fn test_queue_is_consumed() {
        let mut gs = setup();
        assert_eq!(gs.next_queue.len(), 5);
        let first_queue = gs.next_queue.clone();
        gs.hard_drop();
        assert_eq!(gs.next_queue.len(), 5);
        assert_ne!(gs.next_queue, first_queue);
    }

    #[test]
    fn test_ghost_updates_after_move() {
        let mut gs = setup();
        let g1 = gs.ghost_pos;
        gs.move_left();
        let g2 = gs.ghost_pos;
        assert_eq!(g2.x, g1.x - 1);
    }

    #[test]
    fn test_no_ops_after_game_over() {
        let mut gs = setup();
        gs.game_over = true;
        assert!(!gs.move_left());
        assert!(!gs.move_right());
        assert!(!gs.move_down());
        assert!(!gs.rotate_cw());
        assert!(!gs.rotate_ccw());
        assert_eq!(gs.soft_drop(), 0);
        assert_eq!(gs.hard_drop(), 0);
        assert!(!gs.hold());
    }

    #[test]
    fn test_each_bag_in_full_round() {
        let mut gs = setup();
        let mut seen_all = [false; 7];
        let all = [
            CellType::I, CellType::O, CellType::T,
            CellType::S, CellType::Z, CellType::J, CellType::L,
        ];
        for _ in 0..7 {
            let idx = all.iter().position(|&x| x == gs.current_piece).unwrap();
            seen_all[idx] = true;
            gs.hard_drop();
        }
        assert!(seen_all.iter().all(|&x| x), "not all 7 pieces seen in one bag cycle");
    }
}
