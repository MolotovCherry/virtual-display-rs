import 'dart:typed_data';

import 'package:vdd/errors.dart' as vdd;
import 'package:vdd/vdd.dart' as vdd;

void main(List<String> args) async {
  await vdd.init();

  try {
    // Connect to the driver, using the default pipe name
    final client = await vdd.DriverClient.connect();

    // Listen to all state changes
    client.receiveEvents().listen((monitors) async {
      print("Driver state changed: $monitors");

      // Refresh local state when driver state changes
      await client.refreshState();
    });

    print("Current driver state: ${client.state}");

    // Get a free id for a new monitor
    final monitorId = client.newId()!;

    // Add a new monitor
    client.add(
      monitor: vdd.Monitor(
        id: monitorId,
        enabled: true,
        modes: [
          vdd.Mode(
            width: 1920,
            height: 1080,
            refreshRates: Uint32List.fromList([60, 120]),
          ),
        ],
      ),
    );

    client.addMode(
      id: monitorId,
      mode: vdd.Mode(
        width: 2560,
        height: 1440,
        refreshRates: Uint32List.fromList([60, 120]),
      ),
    );

    // Apply changes to the driver
    await client.notify();

    // Make changes persistent across restarts
    await client.persist();

    await Future.delayed(Duration(seconds: 5));

    // Remove all monitors
    client.removeAll();

    await client.notify();

    await client.persist();
  } on vdd.InitError catch (e) {
    print("Did you forget to install the driver?\n$e");
  }
}
