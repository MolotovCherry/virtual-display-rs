import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/run_command.dart';
import 'package:native_assets_cli/native_assets_cli.dart';

void main(List<String> args) async {
  final buildConfig = await BuildConfig.fromArgs(args);
  final buildOutput = BuildOutput();

  final rustCrateDir = buildConfig.packageRoot.resolve('rust');

  final rustflags = Platform.environment['RUSTFLAGS'];

  await runCommand(
    'cargo',
    [
      'build',
      '--release',
      '--out-dir',
      buildConfig.outDir.toFilePath(),
      '-Z',
      'unstable-options',
    ],
    pwd: rustCrateDir.toFilePath(),
    printCommandInStderr: true,
    env: {
      // Though runCommand auto pass environment variable to commands,
      // we do this to explicitly show this important flag
      if (rustflags != null) 'RUSTFLAGS': rustflags,
    },
  );

  buildOutput.dependencies.dependencies.addAll({
    rustCrateDir,
    buildConfig.packageRoot.resolve('build.rs'),
  });
  buildOutput.assets.add(Asset(
    id: "package:dart_vdd/dart-vdd-lib.dart",
    linkMode: LinkMode.dynamic,
    target: Target.current,
    path: AssetAbsolutePath(buildConfig.outDir.resolve('dart_vdd.dll')),
  ));

  await buildOutput.writeToFile(outDir: buildConfig.outDir);
}
