import 'package:flutter/material.dart';
import 'package:tetris/src/rust/core/types.dart';

Color cellTypeToColor(CellType type) {
  return switch (type) {
    CellType.empty => Colors.transparent,
    CellType.i => const Color(0xFF00F0F0),
    CellType.o => const Color(0xFFF0F000),
    CellType.t => const Color(0xFFA000F0),
    CellType.s => const Color(0xFF00F000),
    CellType.z => const Color(0xFFF00000),
    CellType.j => const Color(0xFF0000F0),
    CellType.l => const Color(0xFFF0A000),
  };
}

Color cellTypeToGhostColor(CellType type) {
  final base = cellTypeToColor(type);
  return base.withValues(alpha: 0.25);
}
