use super::types::*;

// Each piece has 4 rotation states, each with 4 cell offsets relative to its origin.
// Indices: [piece_index][rotation_state][cell_index]
// piece_index order: I=0, O=1, T=2, S=3, Z=4, J=5, L=6
const PIECE_CELLS: [[[Position; 4]; 4]; 7] = [
    // I
    [
        [Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }, Position { x: 3, y: 1 }],
        [Position { x: 2, y: 0 }, Position { x: 2, y: 1 }, Position { x: 2, y: 2 }, Position { x: 2, y: 3 }],
        [Position { x: 0, y: 2 }, Position { x: 1, y: 2 }, Position { x: 2, y: 2 }, Position { x: 3, y: 2 }],
        [Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 1, y: 2 }, Position { x: 1, y: 3 }],
    ],
    // O
    [
        [Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }],
        [Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }],
        [Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }],
        [Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }],
    ],
    // T
    [
        [Position { x: 1, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }],
        [Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }, Position { x: 1, y: 2 }],
        [Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }, Position { x: 1, y: 2 }],
        [Position { x: 1, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 1, y: 2 }],
    ],
    // S
    [
        [Position { x: 1, y: 0 }, Position { x: 2, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }],
        [Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }, Position { x: 2, y: 2 }],
        [Position { x: 1, y: 1 }, Position { x: 2, y: 1 }, Position { x: 0, y: 2 }, Position { x: 1, y: 2 }],
        [Position { x: 0, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 1, y: 2 }],
    ],
    // Z
    [
        [Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }],
        [Position { x: 2, y: 0 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }, Position { x: 1, y: 2 }],
        [Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 1, y: 2 }, Position { x: 2, y: 2 }],
        [Position { x: 1, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 0, y: 2 }],
    ],
    // J
    [
        [Position { x: 0, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }],
        [Position { x: 1, y: 0 }, Position { x: 2, y: 0 }, Position { x: 1, y: 1 }, Position { x: 1, y: 2 }],
        [Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }, Position { x: 2, y: 2 }],
        [Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 0, y: 2 }, Position { x: 1, y: 2 }],
    ],
    // L
    [
        [Position { x: 2, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }],
        [Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 1, y: 2 }, Position { x: 2, y: 2 }],
        [Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }, Position { x: 0, y: 2 }],
        [Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 1, y: 2 }],
    ],
];

fn piece_index(piece: CellType) -> usize {
    match piece {
        CellType::I => 0,
        CellType::O => 1,
        CellType::T => 2,
        CellType::S => 3,
        CellType::Z => 4,
        CellType::J => 5,
        CellType::L => 6,
        CellType::Empty => panic!("CellType::Empty has no piece data"),
    }
}

pub fn get_cells(piece: CellType, rotation: usize) -> &'static [Position; 4] {
    &PIECE_CELLS[piece_index(piece)][rotation % 4]
}

// SRS offset tables
// Each rotation state has 5 offset tests for wall kicks.

// JLSTZ offset table: [rotation][test_index] = (dx, dy)
const JLSTZ_OFFSETS: [[(i32, i32); 5]; 4] = [
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
];

// I offset table
const I_OFFSETS: [[(i32, i32); 5]; 4] = [
    [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
    [(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
    [(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
    [(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],
];

pub fn get_kicks(piece: CellType, from: usize, to: usize) -> [(i32, i32); 5] {
    // O piece has no kicks (all offset tests are (0,0) since it doesn't meaningfully rotate)
    if piece == CellType::O {
        return [(0, 0); 5];
    }
    let offsets = match piece {
        CellType::I => &I_OFFSETS,
        _ => &JLSTZ_OFFSETS,
    };
    let from_off = offsets[from % 4];
    let to_off = offsets[to % 4];
    let mut kicks = [(0i32, 0i32); 5];
    for i in 0..5 {
        kicks[i] = (from_off[i].0 - to_off[i].0, from_off[i].1 - to_off[i].1);
    }
    kicks
}

pub fn spawn_pos(piece: CellType) -> Position {
    match piece {
        CellType::I | CellType::T | CellType::S | CellType::Z | CellType::J | CellType::L => {
            Position { x: 3, y: 0 }
        }
        CellType::O => Position { x: 4, y: 0 },
        CellType::Empty => Position { x: 0, y: 0 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cells_t_spawn() {
        let cells = get_cells(CellType::T, 0);
        assert_eq!(cells[0], Position { x: 1, y: 0 });
        assert_eq!(cells[1], Position { x: 0, y: 1 });
        assert_eq!(cells[2], Position { x: 1, y: 1 });
        assert_eq!(cells[3], Position { x: 2, y: 1 });
    }

    #[test]
    fn test_get_cells_o_always_same() {
        for rot in 0..4 {
            let cells = get_cells(CellType::O, rot);
            assert_eq!(cells[0], Position { x: 0, y: 0 });
            assert_eq!(cells[1], Position { x: 1, y: 0 });
            assert_eq!(cells[2], Position { x: 0, y: 1 });
            assert_eq!(cells[3], Position { x: 1, y: 1 });
        }
    }

    #[test]
    fn test_each_piece_has_4_cells_per_rotation() {
        for piece in &[CellType::I, CellType::O, CellType::T, CellType::S, CellType::Z, CellType::J, CellType::L] {
            for rot in 0..4 {
                let cells = get_cells(*piece, rot);
                assert_eq!(cells.len(), 4);
            }
        }
    }

    #[test]
    fn test_spawn_positions() {
        assert_eq!(spawn_pos(CellType::I), Position { x: 3, y: 0 });
        assert_eq!(spawn_pos(CellType::O), Position { x: 4, y: 0 });
        assert_eq!(spawn_pos(CellType::T), Position { x: 3, y: 0 });
    }

    #[test]
    fn test_get_kicks_jlstz_0_to_r() {
        let kicks = get_kicks(CellType::T, 0, 1);
        // (0,0)-(0,0) = (0,0)
        assert_eq!(kicks[0], (0, 0));
        // (0,0)-(1,0) = (-1,0)
        assert_eq!(kicks[1], (-1, 0));
    }

    #[test]
    fn test_get_kicks_i_0_to_r() {
        let kicks = get_kicks(CellType::I, 0, 1);
        // (0,0)-(-1,0) = (1,0)
        assert_eq!(kicks[0], (1, 0));
    }

    #[test]
    fn test_get_kicks_cw_and_ccw_symmetric() {
        let cw = get_kicks(CellType::T, 0, 1);
        let ccw = get_kicks(CellType::T, 1, 0);
        // CW kicks should be the negation of CCW kicks for JLSTZ
        for i in 0..5 {
            assert_eq!(cw[i], (-ccw[i].0, -ccw[i].1));
        }
    }

    #[test]
    fn test_rotation_cycles_all_four_states() {
        let t = CellType::T;
        let _base = Position { x: 0, y: 0 };
        let mut rot = 0;
        // CW rotations should cycle through all 4 states
        for _ in 0..4 {
            let kicks = get_kicks(t, rot, (rot + 1) % 4);
            // First kick is always (0,0) for JLSTZ
            assert_eq!(kicks[0], (0, 0));
            rot = (rot + 1) % 4;
        }
    }

    #[test]
    fn test_o_piece_no_kick() {
        let kicks = get_kicks(CellType::O, 0, 1);
        for i in 0..5 {
            assert_eq!(kicks[i], (0, 0));
        }
    }
}
