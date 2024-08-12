import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:native_assets_cli/native_assets_cli.dart';

void main(List<String> args) async {
  await build(args, (config, output) async {
    final rustRoot = config.packageRoot.resolve('rust');

    final rustFlags = Platform.environment['RUSTFLAGS'];

    await runCommand(
      'cargo',
      [
        'build',
        '--release',
        '--artifact-dir',
        config.outputDirectory.toFilePath(),
        '-Z',
        'unstable-options',
      ],
      pwd: rustRoot.toFilePath(),
      printCommandInStderr: true,
      env: {
        if (rustFlags != null) 'RUSTFLAGS': rustFlags,
      },
    );

    output.addDependencies(await Directory.fromUri(rustRoot)
        .list(recursive: true)
        .where((f) => f is File)
        .map((e) => e.uri)
        .toList());

    output.addAsset(NativeCodeAsset(
      package: "vdd",
      linkMode: DynamicLoadingBundled(),
      name: "vdd_lib",
      os: OS.windows,
      architecture: Architecture.current,
      file: config.outputDirectory.resolve('dart_vdd.dll'),
    ));
  });
}
