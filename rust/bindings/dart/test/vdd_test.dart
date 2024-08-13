import 'package:vdd/errors.dart';
import 'package:vdd/vdd.dart';
import 'package:test/test.dart';

void main() {
  setUpAll(() async {
    await init();
  });

  test('Client.connect throws correct error', () async {
    try {
      await Client.connect(pipeName: "nonexisting");
      fail("Expected ConnectionError");
    } catch (e) {
      expect(e, isA<ConnectionError_Failed>());
    }
  });
}
