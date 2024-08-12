// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.2.0.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api.dart';
import 'api/sub_module.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'frb_generated.io.dart'
    if (dart.library.js_interop) 'frb_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
  }) async {
    await instance.initImpl(
      api: api,
      handler: handler,
      externalLibrary: externalLibrary,
    );
  }

  /// Dispose flutter_rust_bridge
  ///
  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
  /// is automatically disposed when the app stops.
  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  Future<void> executeRustInitializers() async {}

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      kDefaultExternalLibraryLoaderConfig;

  @override
  String get codegenVersion => '2.2.0';

  @override
  int get rustContentHash => 952525661;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
    stem: 'dart_vdd',
    ioDirectory: 'rust/target/release/',
    webPrefix: 'pkg/',
  );
}

abstract class RustLibApi extends BaseApi {
  TestRustApi crateApiTestRustApiNew();

  Future<void> crateApiTestRustApiTest({required TestRustApi that});

  AnotherTestRustApi crateApiSubModuleAnotherTestRustApiNew();

  Future<void> crateApiSubModuleAnotherTestRustApiTest(
      {required AnotherTestRustApi that});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  TestRustApi crateApiTestRustApiNew() {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 1)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_test_rust_api,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiTestRustApiNewConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTestRustApiNewConstMeta => const TaskConstMeta(
        debugName: "test_rust_api_new",
        argNames: [],
      );

  @override
  Future<void> crateApiTestRustApiTest({required TestRustApi that}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_box_autoadd_test_rust_api(that, serializer);
        pdeCallFfi(generalizedFrbRustBinding, serializer,
            funcId: 2, port: port_);
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiTestRustApiTestConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiTestRustApiTestConstMeta => const TaskConstMeta(
        debugName: "test_rust_api_test",
        argNames: ["that"],
      );

  @override
  AnotherTestRustApi crateApiSubModuleAnotherTestRustApiNew() {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 3)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_another_test_rust_api,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiSubModuleAnotherTestRustApiNewConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiSubModuleAnotherTestRustApiNewConstMeta =>
      const TaskConstMeta(
        debugName: "another_test_rust_api_new",
        argNames: [],
      );

  @override
  Future<void> crateApiSubModuleAnotherTestRustApiTest(
      {required AnotherTestRustApi that}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_box_autoadd_another_test_rust_api(that, serializer);
        pdeCallFfi(generalizedFrbRustBinding, serializer,
            funcId: 4, port: port_);
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateApiSubModuleAnotherTestRustApiTestConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiSubModuleAnotherTestRustApiTestConstMeta =>
      const TaskConstMeta(
        debugName: "another_test_rust_api_test",
        argNames: ["that"],
      );

  @protected
  AnotherTestRustApi dco_decode_another_test_rust_api(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    final arr = raw as List<dynamic>;
    if (arr.length != 0)
      throw Exception('unexpected arr length: expect 0 but see ${arr.length}');
    return AnotherTestRustApi.raw();
  }

  @protected
  AnotherTestRustApi dco_decode_box_autoadd_another_test_rust_api(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dco_decode_another_test_rust_api(raw);
  }

  @protected
  TestRustApi dco_decode_box_autoadd_test_rust_api(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dco_decode_test_rust_api(raw);
  }

  @protected
  TestRustApi dco_decode_test_rust_api(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    final arr = raw as List<dynamic>;
    if (arr.length != 0)
      throw Exception('unexpected arr length: expect 0 but see ${arr.length}');
    return TestRustApi.raw();
  }

  @protected
  void dco_decode_unit(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return;
  }

  @protected
  AnotherTestRustApi sse_decode_another_test_rust_api(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return AnotherTestRustApi.raw();
  }

  @protected
  AnotherTestRustApi sse_decode_box_autoadd_another_test_rust_api(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return (sse_decode_another_test_rust_api(deserializer));
  }

  @protected
  TestRustApi sse_decode_box_autoadd_test_rust_api(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return (sse_decode_test_rust_api(deserializer));
  }

  @protected
  TestRustApi sse_decode_test_rust_api(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return TestRustApi.raw();
  }

  @protected
  void sse_decode_unit(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getInt32();
  }

  @protected
  bool sse_decode_bool(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8() != 0;
  }

  @protected
  void sse_encode_another_test_rust_api(
      AnotherTestRustApi self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  void sse_encode_box_autoadd_another_test_rust_api(
      AnotherTestRustApi self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_another_test_rust_api(self, serializer);
  }

  @protected
  void sse_encode_box_autoadd_test_rust_api(
      TestRustApi self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_test_rust_api(self, serializer);
  }

  @protected
  void sse_encode_test_rust_api(TestRustApi self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  void sse_encode_unit(void self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putInt32(self);
  }

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self ? 1 : 0);
  }
}
