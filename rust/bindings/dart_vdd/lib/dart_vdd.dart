/// Support for doing something awesome.
///
/// More dartdocs go here.
library;

import 'package:dart_vdd/src/generated/frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

export 'src/generated/api.dart';

Future<void> initVdd() async {
  await RustLib.init(
    // This works with `dart build`, `flutter build`, `flutter run`
    externalLibrary: ExternalLibrary.open("dart_vdd.dll"),
  );
}
