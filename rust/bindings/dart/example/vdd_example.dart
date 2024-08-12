import 'package:vdd/vdd.dart' as vdd;

void main() async {
  await vdd.init();

  vdd.TestRustApi().test();
}
