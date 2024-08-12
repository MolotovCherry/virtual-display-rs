import 'package:vdd/vdd.dart';
import 'package:test/test.dart';

void main() {
  setUpAll(() async {
    await init();
  });

  test('TestRustApi', () async {
    await TestRustApi().test();
  });
}
