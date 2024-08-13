import 'dart:typed_data';

import 'package:vdd/errors.dart';
import 'package:vdd/test.dart';
import 'package:vdd/vdd.dart';
import 'package:test/test.dart';

void main() {
  final monitor = Monitor(
    id: 0,
    enabled: true,
    modes: [
      Mode(
        width: 1920,
        height: 1080,
        refreshRates: Uint32List.fromList([60]),
      ),
    ],
  );

  setUpAll(() async {
    await init();
  });

  group('Client', () {
    test('Client.connect throws correct error', () async {
      try {
        await Client.connect(pipeName: "nonexisting");
        fail("Expected ConnectionError");
      } catch (e) {
        expect(e, isA<ConnectionError_Failed>());
      }
    });

    test('Client can connect', () async {
      final server = await MockServer.create(pipeName: "client_can_connect");

      await Client.connect(pipeName: server.pipeName);
    });

    test('Client can notify', () async {
      final server = await MockServer.create(pipeName: "client_can_notify");

      final client = await Client.connect(pipeName: server.pipeName);

      await client.notify(monitors: [monitor]);

      await server.pump();

      expect(server.state, [monitor]);

      await client.notify(monitors: []);

      await server.pump();

      expect(server.state, []);
    });

    test('Client can remove', () async {
      final server = await MockServer.create(pipeName: "client_can_remove");

      final client = await Client.connect(pipeName: server.pipeName);

      await client.notify(
          monitors: List.generate(5, (i) => monitor.copyWith(id: i)));

      await server.pump();

      expect(server.state, List.generate(5, (i) => monitor.copyWith(id: i)));

      await client.remove(ids: [2, 3]);

      await server.pump();

      expect(server.state, [
        monitor.copyWith(id: 0),
        monitor.copyWith(id: 1),
        monitor.copyWith(id: 4),
      ]);

      await client.removeAll();

      await server.pump();

      expect(server.state, []);
    });

    test('Client can receive events', () async {
      final server = await MockServer.create(pipeName: "client_can_receive");

      final client = await Client.connect(pipeName: server.pipeName);

      final stream = client.receiveEvents();

      await client.notify(monitors: [monitor]);

      await server.pump();

      await client.notify(monitors: [monitor.copyWith(enabled: false)]);

      await server.pump();

      await client.notify(monitors: [monitor.copyWith(id: 1)]);

      await server.pump();

      expect(await stream.take(3).toList(), [
        [monitor],
        [monitor.copyWith(enabled: false)],
        [monitor.copyWith(id: 1)],
      ]);
    });

    test('Client can request state', () async {
      final server = await MockServer.create(pipeName: "client_can_request");

      final client = await Client.connect(pipeName: server.pipeName);

      await Future.wait([
        Future(() async {
          expect(await client.requestState(), []);
        }),
        server.pump(),
      ]);

      await client.notify(monitors: [monitor]);

      await server.pump();

      await Future.wait([
        Future(() async {
          expect(await client.requestState(), [monitor]);
        }),
        server.pump(),
      ]);
    });
  });

  group('DriverClient', () {
    Future<(MockServer, DriverClient)> createDeps(String pipeName) async {
      final server = await MockServer.create(pipeName: pipeName);

      final client = DriverClient.connect(pipeName: pipeName);

      await server.pump();

      return (server, await client);
    }

    test('DriverClient.connect throws correct error', () async {
      try {
        await DriverClient.connect(pipeName: "nonexisting");
        fail("Expected ConnectionError");
      } on InitError_Connect catch (e) {
        expect(e.inner, isA<ConnectionError_Failed>());
      } catch (e) {
        fail("Expected InitError_Connect, got $e");
      }
    });

    test('DiverClient can connect', () async {
      final (_, client) = await createDeps("driver_client_can_connect");

      expect(client.state, []);
    });

    test('DriverClient can notify', () async {
      final (server, client) = await createDeps("driver_client_can_notify");

      client.add(monitor: monitor);

      expect(client.state, [monitor]);
      expect(server.state, []);

      await client.notify();

      await server.pump();

      expect(client.state, [monitor]);
      expect(server.state, [monitor]);
    });

    test('DriverClient can refresh its state', () async {
      final (server, client) = await createDeps("driver_client_can_refresh");

      expect(client.state, []);

      await server.setState(state: [monitor]);
      expect(server.state, [monitor]);

      expect(client.state, []);

      expect(await client.refreshState(), [monitor]);

      expect(client.state, [monitor]);
    });

    test('Adding existing monitor throws correct error', () async {
      final (_, client) = await createDeps("driver_client_add_existing");

      client.add(monitor: monitor);

      try {
        client.add(monitor: monitor);
        fail("Expected DuplicateError_Monitor");
      } on DuplicateError_Monitor catch (e) {
        expect(e.id, monitor.id);
      } catch (e) {
        fail("Expected DuplicateError_Monitor, got $e");
      }
    });

    test('Setting duplicate monitors throws correct error', () async {
      final (_, client) =
          await createDeps("driver_client_set_duplicate_monitors");

      try {
        client.setMonitors(monitors: [monitor, monitor]);
        fail("Expected DuplicateError_Monitor");
      } on DuplicateError_Monitor catch (e) {
        expect(e.id, monitor.id);
      } catch (e) {
        fail("Expected DuplicateError_Monitor, got $e");
      }

      try {
        client.setMonitors(monitors: [
          monitor.copyWith(modes: [
            Mode(
              width: 1920,
              height: 1080,
              refreshRates: Uint32List.fromList([60]),
            ),
            Mode(
              width: 1920,
              height: 1080,
              refreshRates: Uint32List.fromList([120]),
            ),
          ])
        ]);
        fail("Expected DuplicateError_Mode");
      } on DuplicateError_Mode catch (e) {
        expect(e.monitorId, monitor.id);
        expect(e.width, 1920);
        expect(e.height, 1080);
      } catch (e) {
        fail("Expected DuplicateError_Mode, got $e");
      }

      try {
        client.setMonitors(monitors: [
          monitor.copyWith(modes: [
            Mode(
              width: 1920,
              height: 1080,
              refreshRates: Uint32List.fromList([60, 60]),
            ),
          ])
        ]);
        fail("Expected DuplicateError_RefreshRate");
      } on DuplicateError_RefreshRate catch (e) {
        expect(e.monitorId, monitor.id);
        expect(e.width, 1920);
        expect(e.height, 1080);
        expect(e.refreshRate, 60);
      } catch (e) {
        fail("Expected DuplicateError_RefreshRate, got $e");
      }
    });

    test('Adding duplicate mode throws correct error', () async {
      final (_, client) = await createDeps("driver_client_add_duplicate_mode");

      try {
        client.addMode(
          id: monitor.id,
          mode: monitor.modes.first,
        );
        fail("Expected AddModeError_MonitorNotFound");
      } on AddModeError_MonitorNotFound catch (e) {
        expect(e.id, monitor.id);
      } catch (e) {
        fail("Expected AddModeError_MonitorNotFound, got $e");
      }

      client.add(monitor: monitor);

      try {
        client.addMode(
          id: monitor.id,
          mode: monitor.modes.first,
        );
        fail("Expected AddModeError_ModeExists");
      } on AddModeError_ModeExists catch (e) {
        expect(e.monitorId, monitor.id);
        expect(e.width, 1920);
        expect(e.height, 1080);
      } catch (e) {
        fail("Expected AddModeError_ModeExists, got $e");
      }

      try {
        client.addMode(
          id: monitor.id,
          mode: monitor.modes.first
              .copyWith(refreshRates: Uint32List.fromList([60, 60])),
        );
        fail("Expected AddModeError_RefreshRateExists");
      } on AddModeError_RefreshRateExists catch (e) {
        expect(e.monitorId, monitor.id);
        expect(e.width, 1920);
        expect(e.height, 1080);
        expect(e.refreshRate, 60);
      } catch (e) {
        fail("Expected AddModeError_RefreshRateExists, got $e");
      }
    });

    test('DriverClient can modify its state', () async {
      final (server, client) = await createDeps("driver_client_can_modify");

      expect(client.state, []);

      client.add(monitor: monitor);

      expect(client.state, [monitor]);

      final newId = client.newId();
      expect(newId, 1);

      client.add(monitor: monitor.copyWith(id: newId!));

      expect(client.state, [monitor, monitor.copyWith(id: newId)]);

      client.replaceMonitor(
          monitor: monitor.copyWith(id: newId, enabled: false));

      expect(
          client.state, [monitor, monitor.copyWith(id: newId, enabled: false)]);

      client.setEnabled(ids: [newId], enabled: true);

      expect(
          client.state, [monitor, monitor.copyWith(id: newId, enabled: true)]);

      client.setMonitors(monitors: [
        monitor,
        monitor.copyWith(id: 2),
        monitor.copyWith(id: 3),
        monitor.copyWith(id: 4),
        monitor.copyWith(id: 5),
      ]);

      expect(client.state, [
        monitor,
        monitor.copyWith(id: 2),
        monitor.copyWith(id: 3),
        monitor.copyWith(id: 4),
        monitor.copyWith(id: 5),
      ]);

      client.addMode(
        id: 3,
        mode: Mode(
          width: 2560,
          height: 1440,
          refreshRates: Uint32List.fromList([60, 120]),
        ),
      );

      expect(
        client.findMonitor(id: 3),
        monitor.copyWith(id: 3, modes: [
          Mode(
            width: 1920,
            height: 1080,
            refreshRates: Uint32List.fromList([60]),
          ),
          Mode(
            width: 2560,
            height: 1440,
            refreshRates: Uint32List.fromList([60, 120]),
          ),
        ]),
      );

      client.removeMode(id: 3, resolution: (1920, 1080));

      expect(
        client.findMonitor(id: 3),
        monitor.copyWith(id: 3, modes: [
          Mode(
            width: 2560,
            height: 1440,
            refreshRates: Uint32List.fromList([60, 120]),
          ),
        ]),
      );

      expect(client.newId(preferredId: 3), null);
      expect(client.newId(preferredId: 0), null);
      expect(client.newId(preferredId: 1), 1);
      expect(client.newId(), 1);

      expect(server.state, []);

      await client.notify();

      await server.pump();

      expect(server.state, client.state);

      client.remove(ids: [2, 3]);

      expect(client.state, [
        monitor,
        monitor.copyWith(id: 4),
        monitor.copyWith(id: 5),
      ]);

      client.removeAll();

      expect(client.state, []);
    });
  });
}
