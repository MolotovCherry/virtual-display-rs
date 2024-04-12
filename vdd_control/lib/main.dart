import 'dart:typed_data';

import 'package:dart_vdd/dart_vdd.dart';
import 'package:flutter/material.dart';

/// Requires flutter main channel currently, change with `flutter channel main`
void main() async {
  await initVdd();

  final driver = VirtualDisplayDriver();

  Future(() async {
    await for (final monitors in driver.stream) {
      print('monitors: $monitors');
    }
  });

  driver.addMonitor(enabled: true, modes: [
    Mode(width: 1920, height: 1080, refreshRates: Uint32List.fromList([60])),
  ]);

  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: Scaffold(
        body: Center(
          child: Text('Hello World!'),
        ),
      ),
    );
  }
}
