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
}
