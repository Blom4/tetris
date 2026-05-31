import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter/services.dart';
import 'package:tetris/src/rust/api/game.dart';
import 'package:tetris/src/rust/core/types.dart';
import 'package:tetris/theme/tetris_theme.dart';
import 'package:tetris/widgets/board_widget.dart';

class GameScreen extends StatefulWidget {
  const GameScreen({super.key});

  @override
  State<GameScreen> createState() => _GameScreenState();
}

class _GameScreenState extends State<GameScreen>
    with SingleTickerProviderStateMixin {
  late GameStateView _state;
  late Ticker _ticker;
  Duration _accumulated = Duration.zero;

  Timer? _repeatTimer;
  Timer? _dasTimer;
  VoidCallback? _repeatAction;

  @override
  void initState() {
    super.initState();
    _state = gameInit();
    _ticker = createTicker(_onTick)..start();
  }

  @override
  void dispose() {
    _ticker.dispose();
    _cancelRepeat();
    super.dispose();
  }

  void _cancelRepeat() {
    _dasTimer?.cancel();
    _repeatTimer?.cancel();
    _dasTimer = null;
    _repeatTimer = null;
    _repeatAction = null;
  }

  void _startRepeat(VoidCallback action) {
    _cancelRepeat();
    _repeatAction = action;
    action();
    _dasTimer = Timer(const Duration(milliseconds: 170), () {
      _repeatTimer = Timer.periodic(const Duration(milliseconds: 50), (_) {
        if (_repeatAction != null) _repeatAction!();
      });
    });
  }

  Duration get _dropInterval {
    final ms = (1000 - (_state.level - 1) * 75).clamp(50, 1000);
    return Duration(milliseconds: ms);
  }

  void _onTick(Duration elapsed) {
    _accumulated += elapsed;
    if (_accumulated >= _dropInterval) {
      _accumulated -= _dropInterval;
      setState(() {
        _state = gameTick(gsv: _state);
      });
    }
  }

  void _restart() {
    setState(() {
      _state = gameInit();
      _accumulated = Duration.zero;
    });
  }

  void _moveLeft() {
    if (!_state.gameOver) {
      setState(() => _state = gameMoveLeft(gsv: _state));
    }
  }

  void _moveRight() {
    if (!_state.gameOver) {
      setState(() => _state = gameMoveRight(gsv: _state));
    }
  }

  void _rotateCw() {
    if (!_state.gameOver) {
      setState(() => _state = gameRotateCw(gsv: _state));
    }
  }

  void _rotateCcw() {
    if (!_state.gameOver) {
      setState(() => _state = gameRotateCcw(gsv: _state));
    }
  }

  void _softDrop() {
    if (!_state.gameOver) {
      setState(() => _state = gameSoftDrop(gsv: _state));
    }
  }

  void _hardDrop() {
    if (!_state.gameOver) {
      setState(() => _state = gameHardDrop(gsv: _state));
    }
  }

  void _holdPiece() {
    if (!_state.gameOver) {
      setState(() => _state = gameHold(gsv: _state));
    }
  }

  void _onKey(Set<LogicalKeyboardKey> keys) {
    if (_state.gameOver) {
      if (keys.contains(LogicalKeyboardKey.space) ||
          keys.contains(LogicalKeyboardKey.enter)) {
        _restart();
      }
      return;
    }
    if (keys.contains(LogicalKeyboardKey.arrowLeft)) {
      _moveLeft();
    } else if (keys.contains(LogicalKeyboardKey.arrowRight)) {
      _moveRight();
    } else if (keys.contains(LogicalKeyboardKey.arrowDown)) {
      _softDrop();
    } else if (keys.contains(LogicalKeyboardKey.arrowUp) ||
        keys.contains(LogicalKeyboardKey.keyX)) {
      _rotateCw();
    } else if (keys.contains(LogicalKeyboardKey.keyZ)) {
      _rotateCcw();
    } else if (keys.contains(LogicalKeyboardKey.space)) {
      _hardDrop();
    } else if (keys.contains(LogicalKeyboardKey.shift) ||
        keys.contains(LogicalKeyboardKey.keyC)) {
      _holdPiece();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0F0F1A),
      body: Focus(
        autofocus: true,
        onKeyEvent: (node, event) {
          if (event is KeyDownEvent || event is KeyRepeatEvent) {
            _onKey({event.logicalKey});
          }
          return KeyEventResult.handled;
        },
        child: SafeArea(
          child: LayoutBuilder(
            builder: (context, constraints) {
              final maxW = constraints.maxWidth;
              final maxH = constraints.maxHeight;
              final isLandscape = maxW > maxH;

              if (isLandscape) {
                return _landscapeBody(maxW, maxH);
              }
              return _portraitBody(maxW, maxH);
            },
          ),
        ),
      ),
    );
  }

  // ─── Portrait ───────────────────────────────────────────────────

  Widget _portraitBody(double width, double height) {
    const controlsHeight = 136.0;
    const sidePanelWidth = 72.0;
    const gap = 4.0;

    final availableBoardHeight = height - controlsHeight;
    final cellSize = (availableBoardHeight / 20).floorToDouble().clamp(12.0, 32.0);

    final boardWidth = 10 * cellSize;
    final totalWidth = sidePanelWidth + gap + boardWidth + gap + sidePanelWidth;

    final board = BoardWidget(state: _state, cellSize: cellSize);

    return Column(
      children: [
        Expanded(
          child: Center(
            child: totalWidth > width
                ? FittedBox(
                    fit: BoxFit.scaleDown,
                    child: _gameRow(board, cellSize, sidePanelWidth, gap),
                  )
                : _gameRow(board, cellSize, sidePanelWidth, gap),
          ),
        ),
        SizedBox(
          height: controlsHeight,
          child: _touchControls(),
        ),
      ],
    );
  }

  Widget _gameRow(Widget board, double cellSize, double pw, double gap) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        SizedBox(width: pw, child: _holdPanel(cellSize)),
        SizedBox(width: gap),
        Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            board,
            if (_state.gameOver)
              Padding(
                padding: const EdgeInsets.only(top: 8),
                child: GestureDetector(
                  onTap: _restart,
                  child: Column(
                    children: [
                      const Text('GAME OVER',
                          style: TextStyle(
                              color: Colors.redAccent,
                              fontSize: 18,
                              fontWeight: FontWeight.bold)),
                      Text('Tap to restart',
                          style: TextStyle(color: Colors.white38, fontSize: 12)),
                    ],
                  ),
                ),
              ),
          ],
        ),
        SizedBox(width: gap),
        SizedBox(width: pw, child: _infoPanel(cellSize)),
      ],
    );
  }

  // ─── Landscape ──────────────────────────────────────────────────

  Widget _landscapeBody(double width, double height) {
    const sidePanelWidth = 72.0;
    const gap = 4.0;

    final cellSize = (height / 20).floorToDouble().clamp(12.0, 32.0);
    final board = BoardWidget(state: _state, cellSize: cellSize);

    return Row(
      children: [
        _touchControls(),
        const SizedBox(width: 8),
        Expanded(
          child: Center(
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                SizedBox(width: sidePanelWidth, child: _holdPanel(cellSize)),
                SizedBox(width: gap),
                Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    board,
                    if (_state.gameOver)
                      Padding(
                        padding: const EdgeInsets.only(top: 8),
                        child: GestureDetector(
                          onTap: _restart,
                          child: const Text('GAME OVER',
                              style: TextStyle(
                                  color: Colors.redAccent,
                                  fontSize: 18,
                                  fontWeight: FontWeight.bold)),
                        ),
                      ),
                  ],
                ),
                SizedBox(width: gap),
                SizedBox(width: sidePanelWidth, child: _infoPanel(cellSize)),
              ],
            ),
          ),
        ),
      ],
    );
  }

  // ─── Touch controls ─────────────────────────────────────────────

  static const _btn = 44.0;

  Widget _touchControls() {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const SizedBox(height: 4),
        Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            _tBtn(Icons.rotate_left, _rotateCcw),
            const SizedBox(width: 6),
            _tBtn(Icons.swap_horiz, _holdPiece),
            const SizedBox(width: 6),
            _tBtn(Icons.rotate_right, _rotateCw),
            const SizedBox(width: 6),
            _tBtn(Icons.vertical_align_bottom, _hardDrop),
          ],
        ),
        const SizedBox(height: 6),
        Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            _hBtn(Icons.arrow_back, _moveLeft),
            const SizedBox(width: 6),
            _hBtn(Icons.arrow_downward, _softDrop),
            const SizedBox(width: 6),
            _hBtn(Icons.arrow_forward, _moveRight),
          ],
        ),
      ],
    );
  }

  Widget _tBtn(IconData icon, VoidCallback onTap) {
    return SizedBox(
      width: _btn,
      height: _btn,
      child: Material(
        color: Colors.white.withValues(alpha: 0.08),
        borderRadius: BorderRadius.circular(8),
        child: InkWell(
          borderRadius: BorderRadius.circular(8),
          onTap: onTap,
          child: Icon(icon, color: Colors.white70, size: 24),
        ),
      ),
    );
  }

  Widget _hBtn(IconData icon, VoidCallback onTap) {
    return SizedBox(
      width: _btn,
      height: _btn,
      child: GestureDetector(
        onTapDown: (_) => _startRepeat(onTap),
        onTapUp: (_) => _cancelRepeat(),
        onTapCancel: _cancelRepeat,
        child: Container(
          decoration: BoxDecoration(
            color: Colors.white.withValues(alpha: 0.08),
            borderRadius: BorderRadius.circular(8),
          ),
          child: Icon(icon, color: Colors.white70, size: 24),
        ),
      ),
    );
  }

  // ─── Side panels ────────────────────────────────────────────────

  Widget _holdPanel(double cellSize) {
    final c = (cellSize * 0.45).clamp(6.0, 16.0);
    return _Column(
      children: [
        _label('HOLD'),
        const SizedBox(height: 4),
        _miniature(_state.holdPiece, _state.canHold, c),
      ],
    );
  }

  Widget _infoPanel(double cellSize) {
    final c = (cellSize * 0.45).clamp(6.0, 16.0);
    return _Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _stat('SCORE', '${_state.score}'),
        const SizedBox(height: 4),
        _stat('LEVEL', '${_state.level}'),
        const SizedBox(height: 4),
        _stat('LINES', '${_state.lines}'),
        const SizedBox(height: 8),
        _label('NEXT'),
        const SizedBox(height: 4),
        ...List.generate(
          _state.nextQueue.length.clamp(0, 5),
          (i) => Padding(
            padding: const EdgeInsets.only(bottom: 2),
            child: _miniature(_state.nextQueue[i], true, c),
          ),
        ),
      ],
    );
  }

  Widget _label(String s) =>
      Text(s, style: const TextStyle(color: Colors.white38, fontSize: 10));

  Widget _stat(String label, String value) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label,
              style: const TextStyle(color: Colors.white38, fontSize: 10)),
          Text(value,
              style: const TextStyle(
                  color: Colors.white, fontSize: 16, fontWeight: FontWeight.bold)),
        ],
      );

  Widget _miniature(CellType? piece, bool active, double c) {
    if (piece == null || !active || c < 4) {
      return Container(
        width: 4 * c,
        height: 3 * c,
        decoration: BoxDecoration(
            color: Colors.white.withValues(alpha: 0.03),
            borderRadius: BorderRadius.circular(2)),
      );
    }
    final off = _pieceCells(piece, 0);
    final cells = <Widget>[];
    for (int r = 0; r < 3; r++) {
      for (int col = 0; col < 4; col++) {
        final filled = off.any((o) => o.$1 == col && o.$2 == r);
        cells.add(Positioned(
          left: col * c,
          top: r * c,
          child: Container(
            width: c,
            height: c,
            decoration: BoxDecoration(
              color: filled ? cellTypeToColor(piece) : Colors.transparent,
              border: filled
                  ? Border.all(
                      color: Colors.white.withValues(alpha: 0.2), width: 0.5)
                  : null,
            ),
          ),
        ));
      }
    }
    return Container(
      decoration: BoxDecoration(
          color: Colors.white.withValues(alpha: 0.03),
          borderRadius: BorderRadius.circular(2)),
      child: SizedBox(
          width: 4 * c, height: 3 * c, child: Stack(children: cells)),
    );
  }
}

class _Column extends StatelessWidget {
  final List<Widget> children;
  final CrossAxisAlignment crossAxisAlignment;
  const _Column(
      {required this.children, this.crossAxisAlignment = CrossAxisAlignment.center});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: crossAxisAlignment,
      children: children,
    );
  }
}

List<(int, int)> _pieceCells(CellType piece, int rotation) {
  return switch ((piece, rotation)) {
    (CellType.i, 0) => [(0, 1), (1, 1), (2, 1), (3, 1)],
    (CellType.i, 1) => [(2, 0), (2, 1), (2, 2), (2, 3)],
    (CellType.i, 2) => [(0, 2), (1, 2), (2, 2), (3, 2)],
    (CellType.i, 3) => [(1, 0), (1, 1), (1, 2), (1, 3)],
    (CellType.o, _) => [(0, 0), (1, 0), (0, 1), (1, 1)],
    (CellType.t, 0) => [(1, 0), (0, 1), (1, 1), (2, 1)],
    (CellType.t, 1) => [(1, 0), (1, 1), (2, 1), (1, 2)],
    (CellType.t, 2) => [(0, 1), (1, 1), (2, 1), (1, 2)],
    (CellType.t, 3) => [(1, 0), (0, 1), (1, 1), (1, 2)],
    (CellType.s, 0) => [(1, 0), (2, 0), (0, 1), (1, 1)],
    (CellType.s, 1) => [(1, 0), (1, 1), (2, 1), (2, 2)],
    (CellType.s, 2) => [(1, 1), (2, 1), (0, 2), (1, 2)],
    (CellType.s, 3) => [(0, 0), (0, 1), (1, 1), (1, 2)],
    (CellType.z, 0) => [(0, 0), (1, 0), (1, 1), (2, 1)],
    (CellType.z, 1) => [(2, 0), (1, 1), (2, 1), (1, 2)],
    (CellType.z, 2) => [(0, 1), (1, 1), (1, 2), (2, 2)],
    (CellType.z, 3) => [(1, 0), (0, 1), (1, 1), (0, 2)],
    (CellType.j, 0) => [(0, 0), (0, 1), (1, 1), (2, 1)],
    (CellType.j, 1) => [(1, 0), (2, 0), (1, 1), (1, 2)],
    (CellType.j, 2) => [(0, 1), (1, 1), (2, 1), (2, 2)],
    (CellType.j, 3) => [(1, 0), (1, 1), (0, 2), (1, 2)],
    (CellType.l, 0) => [(2, 0), (0, 1), (1, 1), (2, 1)],
    (CellType.l, 1) => [(1, 0), (1, 1), (1, 2), (2, 2)],
    (CellType.l, 2) => [(0, 1), (1, 1), (2, 1), (0, 2)],
    (CellType.l, 3) => [(0, 0), (1, 0), (1, 1), (1, 2)],
    _ => [(0, 0), (0, 0), (0, 0), (0, 0)],
  };
}
