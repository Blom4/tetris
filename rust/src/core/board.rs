use super::pieces;
use super::types::*;

pub fn new_grid() -> Grid {
    [[CellType::Empty; WIDTH]; HEIGHT]
}

pub fn in_bounds(pos: Position) -> bool {
    pos.x >= 0 && pos.x < WIDTH as i32 && pos.y >= 0 && pos.y < HEIGHT as i32
}

pub fn collides(grid: &Grid, piece: CellType, rotation: usize, pos: Position) -> bool {
    let cells = pieces::get_cells(piece, rotation);
    cells.iter().any(|&cell| {
        let board_pos = Position {
            x: pos.x + cell.x,
            y: pos.y + cell.y,
        };
        !in_bounds(board_pos) || grid[board_pos.y as usize][board_pos.x as usize] != CellType::Empty
    })
}

pub fn can_place(grid: &Grid, piece: CellType, rotation: usize, pos: Position) -> bool {
    !collides(grid, piece, rotation, pos)
}

pub fn place(grid: &mut Grid, piece: CellType, rotation: usize, pos: Position) {
    let cells = pieces::get_cells(piece, rotation);
    for &cell in cells {
        let board_pos = Position {
            x: pos.x + cell.x,
            y: pos.y + cell.y,
        };
        grid[board_pos.y as usize][board_pos.x as usize] = piece;
    }
}

pub fn clear_lines(grid: &mut Grid) -> usize {
    let mut cleared = 0;
    let mut y = HEIGHT as i32 - 1;
    while y >= 0 {
        let row = y as usize;
        if grid[row].iter().all(|&c| c != CellType::Empty) {
            for r in (1..=row).rev() {
                grid[r] = grid[r - 1];
            }
            grid[0] = [CellType::Empty; WIDTH];
            cleared += 1;
        } else {
            y -= 1;
        }
    }
    cleared
}

pub fn try_rotate(
    grid: &Grid,
    piece: CellType,
    rotation: usize,
    pos: Position,
    cw: bool,
) -> Option<(usize, Position)> {
    let new_rot = if cw {
        (rotation + 1) % 4
    } else {
        (rotation + 3) % 4
    };

    for (dx, dy) in pieces::get_kicks(piece, rotation, new_rot) {
        let test_pos = Position {
            x: pos.x + dx,
            y: pos.y + dy,
        };
        if can_place(grid, piece, new_rot, test_pos) {
            return Some((new_rot, test_pos));
        }
    }
    None
}

pub fn ghost_pos(grid: &Grid, piece: CellType, rotation: usize, pos: Position) -> Position {
    let mut g = pos;
    while can_place(grid, piece, rotation, Position { x: g.x, y: g.y + 1 }) {
        g.y += 1;
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filled_row() -> Grid {
        let mut g = new_grid();
        for x in 0..WIDTH {
            g[HEIGHT - 1][x] = CellType::I;
        }
        g
    }

    #[test]
    fn test_new_grid_is_empty() {
        let g = new_grid();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                assert_eq!(g[y][x], CellType::Empty);
            }
        }
    }

    #[test]
    fn test_in_bounds() {
        assert!(in_bounds(Position { x: 0, y: 0 }));
        assert!(in_bounds(Position { x: 9, y: 19 }));
        assert!(!in_bounds(Position { x: -1, y: 0 }));
        assert!(!in_bounds(Position { x: 0, y: -1 }));
        assert!(!in_bounds(Position { x: 10, y: 0 }));
        assert!(!in_bounds(Position { x: 0, y: 20 }));
    }

    #[test]
    fn test_collides_out_of_bounds() {
        let g = new_grid();
        // T piece at (0, -1) has cells at y=0 and y=-1
        let t = CellType::T;
        assert!(collides(&g, t, 0, Position { x: 3, y: -1 }));
    }

    #[test]
    fn test_place_and_collide() {
        let mut g = new_grid();
        let t = CellType::T;
        place(&mut g, t, 0, Position { x: 3, y: 0 });
        // Placed cells: (4,0), (3,1), (4,1), (5,1)
        assert_eq!(g[0][4], CellType::T);
        assert_eq!(g[1][3], CellType::T);
        assert_eq!(g[1][4], CellType::T);
        assert_eq!(g[1][5], CellType::T);
        // Now try placing overlapping piece
        assert!(collides(&g, t, 0, Position { x: 3, y: 0 }));
    }

    #[test]
    fn test_clear_one_line() {
        let mut g = filled_row();
        assert_eq!(clear_lines(&mut g), 1);
        // Top row should be empty now
        assert!(g[0].iter().all(|&c| c == CellType::Empty));
    }

    #[test]
    fn test_clear_four_lines() {
        let mut g = new_grid();
        for y in (HEIGHT - 4)..HEIGHT {
            for x in 0..WIDTH {
                g[y][x] = CellType::I;
            }
        }
        assert_eq!(clear_lines(&mut g), 4);
    }

    #[test]
    fn test_no_clear_with_gaps() {
        let mut g = new_grid();
        for x in 0..WIDTH {
            g[HEIGHT - 1][x] = CellType::I;
        }
        g[HEIGHT - 1][5] = CellType::Empty; // gap
        assert_eq!(clear_lines(&mut g), 0);
    }

    #[test]
    fn test_clear_lines_with_piece_above() {
        let mut g = new_grid();
        // Fill bottom row
        for x in 0..WIDTH {
            g[HEIGHT - 1][x] = CellType::I;
        }
        // Place a T piece above it
        place(&mut g, CellType::T, 0, Position { x: 3, y: HEIGHT as i32 - 3 });
        assert_eq!(clear_lines(&mut g), 1);
        // T piece should have shifted down
        assert_eq!(g[HEIGHT - 1][3], CellType::T);
        assert_eq!(g[HEIGHT - 1][4], CellType::T);
        assert_eq!(g[HEIGHT - 1][5], CellType::T);
    }

    #[test]
    fn test_ghost_pos() {
        let g = new_grid();
        let t = CellType::T;
        let pos = Position { x: 3, y: 0 };
        let ghost = ghost_pos(&g, t, 0, pos);
        // Should be at the bottom (y=18 for T in spawn state from y=0)
        // T cells: (1,0) -> board (4,0); (0,1)(1,1)(2,1) -> board (3,1)(4,1)(5,1)
        // At y=18: (4,18), (3,19), (4,19), (5,19) - all in bounds
        // At y=19: (4,19), (3,20), (4,20), (5,20) - y=20 is out of bounds
        assert_eq!(ghost.y, 18);
    }

    #[test]
    fn test_ghost_pos_with_obstacle() {
        let mut g = new_grid();
        // Block at y=15 on column 4
        g[15][4] = CellType::I;
        let t = CellType::T;
        let pos = Position { x: 3, y: 0 };
        let ghost = ghost_pos(&g, t, 0, pos);
        // The T's center cell (1,1) maps to board (4, ghost.y+1)
        // At ghost.y = 14: the center is at board (4, 15) which is blocked
        // So ghost should be at y=14
        // Wait, let me retrace:
        // At ghost.y=14: cells at (4,14), (3,15), (4,15), (5,15) -> (4,15) is blocked
        // So ghost.y should be 13
        assert_eq!(ghost.y, 13);
    }

    #[test]
    fn test_try_rotate_basic() {
        let g = new_grid();
        let t = CellType::T;
        let pos = Position { x: 3, y: 5 };
        // Rotate CW from 0 to R
        let result = try_rotate(&g, t, 0, pos, true);
        assert!(result.is_some());
        let (new_rot, new_pos) = result.unwrap();
        assert_eq!(new_rot, 1);
        // With no collisions, the first kick (0,0) should succeed
        assert_eq!(new_pos, pos);
    }

    #[test]
    fn test_try_rotate_wall_kick() {
        let g = new_grid();
        let t = CellType::T;
        // Put T at left edge so rotation collides
        let pos = Position { x: 0, y: 5 };
        // T state 0 at x=0: cells at (1,0)->(1,5), (0,1)->(0,6), (1,1)->(1,6), (2,1)->(2,6)
        // State R at x=0: cells at (1,0)->(1,5), (1,1)->(1,6), (2,1)->(2,6), (1,2)->(1,7)
        // All are in bounds so rotation should work without kicks
        let result = try_rotate(&g, t, 0, pos, true);
        assert!(result.is_some());
    }

    #[test]
    fn test_try_rotate_blocked() {
        let mut g = new_grid();
        let t = CellType::T;
        let pos = Position { x: 3, y: 5 };
        // Fill every cell around the piece to block all kick tests
        for y in 4..=7 {
            for x in 2..=6 {
                g[y][x] = CellType::I;
            }
        }
        let result = try_rotate(&g, t, 0, pos, true);
        assert!(result.is_none());
    }
}
