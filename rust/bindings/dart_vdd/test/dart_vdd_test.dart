import 'package:dart_vdd/dart_vdd.dart';
import 'package:dart_vdd/src/generated/frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:test/test.dart';

/// Can be run from package root using `dart --enable-experiment=native-assets
/// test`
void main() {
  setUpAll(() async {
    await RustLib.init(
      // There is currently no way to accurately resolve the shared library
      externalLibrary: ExternalLibrary.open("../target/release/dart_vdd.dll"),
    );
  });

  test('Test', () {
    final driver = VirtualDisplayDriver();

    expect(() => driver.removeAllMonitors(), throwsException);
  });
}
