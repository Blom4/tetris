import 'package:flutter_test/flutter_test.dart';
import 'package:tetris/main.dart';
import 'package:tetris/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());
  testWidgets('App launches and shows game board', (WidgetTester tester) async {
    await tester.pumpWidget(const TetrisApp());
    await tester.pump();
    expect(find.text('SCORE'), findsOneWidget);
  });
}
