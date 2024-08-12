library;

import 'dart:ffi';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:vdd/src/generated/frb_generated.dart';

export 'src/generated/api.dart';

Future<void> init() async {
  // There is currently no way to locate a native asset in dart.

  // HACK: Force dart to load the dll into the process
  frb_get_rust_content_hash();

  await RustLib.init(externalLibrary: ExternalLibrary.process(iKnowHowToUseIt: true));
}

// HACK: Only used to force dart to load the dll into the process
@Native<Int32 Function()>(assetId: "package:vdd/vdd_lib")
// ignore: non_constant_identifier_names
external int frb_get_rust_content_hash();
