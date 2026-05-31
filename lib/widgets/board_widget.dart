import 'package:flutter/material.dart';
import 'package:tetris/src/rust/api/game.dart';
import 'package:tetris/src/rust/core/types.dart';
import 'package:tetris/theme/tetris_theme.dart';

class BoardWidget extends StatelessWidget {
  final GameStateView state;
  final double cellSize;

  const BoardWidget({super.key, required this.state, this.cellSize = 28});

  @override
  Widget build(BuildContext context) {
    final cells = <Widget>[];

    for (int y = 0; y < 20; y++) {
      for (int x = 0; x < 10; x++) {
        final idx = y * 10 + x;
        final cellType = state.grid[idx];
        Color color;

        if (cellType == CellType.empty) {
          if (isGhostCell(x, y)) {
            color = cellTypeToGhostColor(state.currentPiece);
          } else {
            color = const Color(0xFF1A1A2E);
          }
        } else {
          color = cellTypeToColor(cellType);
        }

        cells.add(
          Positioned(
            left: x * cellSize,
            top: y * cellSize,
            child: Container(
              width: cellSize,
              height: cellSize,
              decoration: BoxDecoration(
                color: color,
                border: cellType != CellType.empty || isGhostCell(x, y)
                    ? Border.all(
                        color: Colors.white.withValues(alpha: 0.2),
                        width: 0.5,
                      )
                    : Border.all(
                        color: Colors.white.withValues(alpha: 0.05),
                        width: 0.5,
                      ),
              ),
            ),
          ),
        );
      }
    }

    return Container(
      decoration: BoxDecoration(
        border: Border.all(color: Colors.white24, width: 2),
        borderRadius: BorderRadius.circular(4),
      ),
      child: SizedBox(
        width: 10 * cellSize,
        height: 20 * cellSize,
        child: Stack(children: cells),
      ),
    );
  }

  bool isGhostCell(int x, int y) {
    final piece = state.currentPiece;
    final rot = state.currentRotation;
    final gx = state.ghostX;
    final gy = state.ghostY;
    final cx = state.currentX;
    final cy = state.currentY;

    if (gx == cx && gy == cy) return false;

    bool onCurrent = false;
    bool onGhost = false;

    for (final offset in _pieceCells(piece, rot)) {
      final px = cx + offset.$1;
      final py = cy + offset.$2;
      if (px == x && py == y) onCurrent = true;
    }

    for (final offset in _pieceCells(piece, rot)) {
      final px = gx + offset.$1;
      final py = gy + offset.$2;
      if (px == x && py == y) onGhost = true;
    }

    return onGhost && !onCurrent;
  }
}

List<(int, int)> _pieceCells(CellType piece, int rotation) {
  return switch ((piece, rotation)) {
    // I
    (CellType.i, 0) => [(0, 1), (1, 1), (2, 1), (3, 1)],
    (CellType.i, 1) => [(2, 0), (2, 1), (2, 2), (2, 3)],
    (CellType.i, 2) => [(0, 2), (1, 2), (2, 2), (3, 2)],
    (CellType.i, 3) => [(1, 0), (1, 1), (1, 2), (1, 3)],
    // O
    (CellType.o, _) => [(0, 0), (1, 0), (0, 1), (1, 1)],
    // T
    (CellType.t, 0) => [(1, 0), (0, 1), (1, 1), (2, 1)],
    (CellType.t, 1) => [(1, 0), (1, 1), (2, 1), (1, 2)],
    (CellType.t, 2) => [(0, 1), (1, 1), (2, 1), (1, 2)],
    (CellType.t, 3) => [(1, 0), (0, 1), (1, 1), (1, 2)],
    // S
    (CellType.s, 0) => [(1, 0), (2, 0), (0, 1), (1, 1)],
    (CellType.s, 1) => [(1, 0), (1, 1), (2, 1), (2, 2)],
    (CellType.s, 2) => [(1, 1), (2, 1), (0, 2), (1, 2)],
    (CellType.s, 3) => [(0, 0), (0, 1), (1, 1), (1, 2)],
    // Z
    (CellType.z, 0) => [(0, 0), (1, 0), (1, 1), (2, 1)],
    (CellType.z, 1) => [(2, 0), (1, 1), (2, 1), (1, 2)],
    (CellType.z, 2) => [(0, 1), (1, 1), (1, 2), (2, 2)],
    (CellType.z, 3) => [(1, 0), (0, 1), (1, 1), (0, 2)],
    // J
    (CellType.j, 0) => [(0, 0), (0, 1), (1, 1), (2, 1)],
    (CellType.j, 1) => [(1, 0), (2, 0), (1, 1), (1, 2)],
    (CellType.j, 2) => [(0, 1), (1, 1), (2, 1), (2, 2)],
    (CellType.j, 3) => [(1, 0), (1, 1), (0, 2), (1, 2)],
    // L
    (CellType.l, 0) => [(2, 0), (0, 1), (1, 1), (2, 1)],
    (CellType.l, 1) => [(1, 0), (1, 1), (1, 2), (2, 2)],
    (CellType.l, 2) => [(0, 1), (1, 1), (2, 1), (0, 2)],
    (CellType.l, 3) => [(0, 0), (1, 0), (1, 1), (1, 2)],
    _ => [(0, 0), (0, 0), (0, 0), (0, 0)],
  };
}
