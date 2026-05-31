import 'package:flutter/material.dart';
import 'package:tetris/screens/game_screen.dart';
import 'package:tetris/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const TetrisApp());
}

class TetrisApp extends StatelessWidget {
  const TetrisApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Tetris',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark(),
      home: const GameScreen(),
    );
  }
}
