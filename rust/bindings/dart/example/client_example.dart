import 'dart:io';
import 'dart:typed_data';

import 'package:vdd/errors.dart' as vdd;
import 'package:vdd/vdd.dart' as vdd;

void main(List<String> args) async {
  await vdd.init();

  // Connect to driver using the default named pipe
  final client = await vdd.Client.connect();

  try {
    print("Current state: ${await client.requestState()}");
  } on vdd.RequestError catch (e) {
    print("Did you forget to install the driver?\n$e");
    return;
  }

  // Listen to all state changes
  client.receiveEvents().listen((monitors) {
    print("Driver state changed: $monitors");
  });

  final monitors = [
    vdd.Monitor(
      id: 0,
      enabled: true,
      modes: [
        vdd.Mode(
          width: 1920,
          height: 1080,
          refreshRates: Uint32List.fromList([60, 120]),
        ),
      ],
    ),
  ];

  // Override driver state with new monitor
  await client.notify(monitors: monitors);

  // Make this state persistent across restarts
  await vdd.Client.persist(monitors: monitors);

  await Future.delayed(Duration(seconds: 5));

  // Remove all monitors
  await client.removeAll();

  await vdd.Client.persist(monitors: []);

  exit(0);
}
