import 'package:dart_vdd/dart_vdd.dart';
import 'package:dart_vdd/src/generated/frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Can be run from package root using `dart --enable-experiment=native-assets
/// run .\example\dart_vdd_example.dart`
void main() async {
  await RustLib.init(
    // There is currently no way to accurately resolve the shared library in
    // every szenario
    externalLibrary: ExternalLibrary.open("../target/release/dart_vdd.dll"),

    // This works with `dart build`
    // externalLibrary: ExternalLibrary.open("dart_vdd.dll"),
  );

  final driver = VirtualDisplayDriver();

  Future(() async {
    await for (final monitors in driver.stream) {
      print('monitors: $monitors');
    }
  });

  driver.addMonitor(enabled: true, modes: [
    Mode(width: 1920, height: 1080, refreshRates: Uint32List.fromList([60])),
  ]);
}
