import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:vdd/errors.dart' as vdd;
import 'package:vdd/vdd.dart' as vdd;

void main() async {
  await vdd.init();

  try {
    final client = await vdd.Client.connect();

    client.receiveEvents().listen((monitors) {
      print('Driver changed state: $monitors');
    });

    await client.notify(monitors: [
      vdd.Monitor(
        id: 0,
        enabled: true,
        modes: [
          vdd.Mode(
            width: 1920,
            height: 1080,
            refreshRates: Uint32List.fromList([60]),
          ),
        ],
      ),
    ]);

    await Future.delayed(Duration(seconds: 5));

    await client.removeAll();
  } on vdd.ConnectionError catch (e) {
    print('Error: $e');
  }
}
