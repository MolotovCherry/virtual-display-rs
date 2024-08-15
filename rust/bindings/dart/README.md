Control
[`virtual-display-rs`](https://github.com/MolotovCherry/virtual-display-rs)
using Dart, to create and manage virtual monitors on Windows.

`virtual-display-rs` uses windows named pipes to connect to the driver.

> Note: This package uses [native
> assets](https://github.com/dart-lang/sdk/issues/50565), which are not yet
> stable. To use this package, you must enable it using
> `--enable-experiment=native-assets` as the first flag on every dart command,
> or, when using flutter, enable it in the flutter config using `flutter config
> --enable-native-assets`.

## Features

- Add/Remove/Change virtual monitors
- Add/Remove/Change monitor resolutions and framerate
- Persist driver state across restarts
- Get continues state updates from the driver, no matter why it changed

## Getting started

1. [Install the driver](https://github.com/MolotovCherry/virtual-display-rs?tab=readme-ov-file#how-to-install)
2. Install Rust using [rustup](https://www.rust-lang.org/learn/get-started)
3. Add dependency to `pubspec.yaml`:

```yaml
dependencies:
  vdd:
    git:
      url: https://github.com/MolotovCherry/virtual-display-rs.git
      ref: master
      path: rust/bindings/dart
```

## Usage

Import the relevant packages:

```dart
import 'package:vdd/vdd.dart' as vdd;

// Imports all errors, that might be thrown by dart_vdd
import 'package:vdd/errors.dart' as vdd;
```

### Initialization

Before using the driver, `vdd` must be initialized:

```dart
await vdd.init();
```

There are two clients with a different level of abstraction.

### `Client`

This client does not manage its own state.

```dart
// Connect to driver using the default named pipe
final client = await vdd.Client.connect();

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
```

### `DriverClient`

This client manages its own state, separate from the driver. This state might
become stale. To refresh it, call `DriverClient.refreshState`. To apply state
changes to the driver, call `DriverClient.notify`.

```dart
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

// Apply changes to the driver
await client.notify();

// Make changes persistent across restarts
await client.persist();
```

## Testing

`vdd` provides a mock server, simulating the driver on the other end of the
named pipe.

Additionally, import `package:vdd/test.dart` to get access to the mock server.

```dart
import 'package:vdd/vdd.dart' as vdd;
import 'package:vdd/test.dart' as vdd;
```

```dart
setUpAll(() async {
  await vdd.init();
});

test('test', () async {
  // Pass in a unique pipe name
  final server = await vdd.MockServer.create(pipeName: "my_pipe_name");
  final client = await vdd.Client.connect(pipeName: server.pipeName);

  // Use the client
  await client.notify(...);

  // Pump the server, to handle exactly one request
  await server.pump();

  expect(server.state, ...);

  await server.setState(...);

  expect(client.requestState(), ...);
});
```

For more examples, see the
[tests](https://github.com/MolotovCherry/virtual-display-rs/tree/master/rust/bindings/dart/test/vdd_test.dart)
in `test/`.

## Maintaining

To regenerate bindings, run:

```bash
cargo make generate
```

or

```bash
cargo make generate-watch
```

To run unit tests, run:

```bash
cargo make test
```

To run examples, run:

```bash
dart --enable-experiment=native-assets run example/<example>.dart
```