// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.2.0.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

import 'api.dart';
import 'api/client.dart';
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
  int get rustContentHash => 2054341763;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
    stem: 'dart_vdd',
    ioDirectory: 'rust/target/release/',
    webPrefix: 'pkg/',
  );
}

abstract class RustLibApi extends BaseApi {
  Future<Client> crateApiClientClientConnect({String? pipeName});

  Future<void> crateApiClientClientNotify(
      {required Client that, required List<Monitor> monitors});

  Stream<List<Monitor>> crateApiClientClientReceiveEvents(
      {required Client that});

  Future<void> crateApiClientClientRemove(
      {required Client that, required List<int> ids});

  Future<void> crateApiClientClientRemoveAll({required Client that});

  Future<List<Monitor>> crateApiClientClientRequestState(
      {required Client that});

  RustArcIncrementStrongCountFnType get rust_arc_increment_strong_count_Client;

  RustArcDecrementStrongCountFnType get rust_arc_decrement_strong_count_Client;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_ClientPtr;
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  Future<Client> crateApiClientClientConnect({String? pipeName}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_opt_String(pipeName, serializer);
        pdeCallFfi(generalizedFrbRustBinding, serializer,
            funcId: 1, port: port_);
      },
      codec: SseCodec(
        decodeSuccessData:
            sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient,
        decodeErrorData: sse_decode_connection_error,
      ),
      constMeta: kCrateApiClientClientConnectConstMeta,
      argValues: [pipeName],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiClientClientConnectConstMeta =>
      const TaskConstMeta(
        debugName: "Client_connect",
        argNames: ["pipeName"],
      );

  @override
  Future<void> crateApiClientClientNotify(
      {required Client that, required List<Monitor> monitors}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
            that, serializer);
        sse_encode_list_monitor(monitors, serializer);
        pdeCallFfi(generalizedFrbRustBinding, serializer,
            funcId: 2, port: port_);
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: sse_decode_send_error,
      ),
      constMeta: kCrateApiClientClientNotifyConstMeta,
      argValues: [that, monitors],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiClientClientNotifyConstMeta => const TaskConstMeta(
        debugName: "Client_notify",
        argNames: ["that", "monitors"],
      );

  @override
  Stream<List<Monitor>> crateApiClientClientReceiveEvents(
      {required Client that}) {
    final sink = RustStreamSink<List<Monitor>>();
    unawaited(handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
            that, serializer);
        sse_encode_StreamSink_list_monitor_Sse(sink, serializer);
        pdeCallFfi(generalizedFrbRustBinding, serializer,
            funcId: 3, port: port_);
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: sse_decode_receive_error,
      ),
      constMeta: kCrateApiClientClientReceiveEventsConstMeta,
      argValues: [that, sink],
      apiImpl: this,
    )));
    return sink.stream;
  }

  TaskConstMeta get kCrateApiClientClientReceiveEventsConstMeta =>
      const TaskConstMeta(
        debugName: "Client_receive_events",
        argNames: ["that", "sink"],
      );

  @override
  Future<void> crateApiClientClientRemove(
      {required Client that, required List<int> ids}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
            that, serializer);
        sse_encode_list_prim_u_32_loose(ids, serializer);
        pdeCallFfi(generalizedFrbRustBinding, serializer,
            funcId: 4, port: port_);
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: sse_decode_send_error,
      ),
      constMeta: kCrateApiClientClientRemoveConstMeta,
      argValues: [that, ids],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiClientClientRemoveConstMeta => const TaskConstMeta(
        debugName: "Client_remove",
        argNames: ["that", "ids"],
      );

  @override
  Future<void> crateApiClientClientRemoveAll({required Client that}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
            that, serializer);
        pdeCallFfi(generalizedFrbRustBinding, serializer,
            funcId: 5, port: port_);
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: sse_decode_send_error,
      ),
      constMeta: kCrateApiClientClientRemoveAllConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiClientClientRemoveAllConstMeta =>
      const TaskConstMeta(
        debugName: "Client_remove_all",
        argNames: ["that"],
      );

  @override
  Future<List<Monitor>> crateApiClientClientRequestState(
      {required Client that}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
            that, serializer);
        pdeCallFfi(generalizedFrbRustBinding, serializer,
            funcId: 6, port: port_);
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_list_monitor,
        decodeErrorData: sse_decode_request_error,
      ),
      constMeta: kCrateApiClientClientRequestStateConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateApiClientClientRequestStateConstMeta =>
      const TaskConstMeta(
        debugName: "Client_request_state",
        argNames: ["that"],
      );

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_Client => wire
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_Client => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return AnyhowException(raw as String);
  }

  @protected
  Client
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return ClientImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  Client
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return ClientImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  Duration dco_decode_Chrono_Duration(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dcoDecodeDuration(dco_decode_i_64(raw).toInt());
  }

  @protected
  Client
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return ClientImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  RustStreamSink<List<Monitor>> dco_decode_StreamSink_list_monitor_Sse(
      dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    throw UnimplementedError();
  }

  @protected
  String dco_decode_String(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as String;
  }

  @protected
  bool dco_decode_bool(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as bool;
  }

  @protected
  ConnectionError dco_decode_connection_error(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    switch (raw[0]) {
      case 0:
        return ConnectionError_Failed(
          message: dco_decode_String(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  @protected
  PlatformInt64 dco_decode_i_64(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dcoDecodeI64(raw);
  }

  @protected
  List<Mode> dco_decode_list_mode(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return (raw as List<dynamic>).map(dco_decode_mode).toList();
  }

  @protected
  List<Monitor> dco_decode_list_monitor(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return (raw as List<dynamic>).map(dco_decode_monitor).toList();
  }

  @protected
  List<int> dco_decode_list_prim_u_32_loose(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as List<int>;
  }

  @protected
  Uint32List dco_decode_list_prim_u_32_strict(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as Uint32List;
  }

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as Uint8List;
  }

  @protected
  Mode dco_decode_mode(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return Mode(
      width: dco_decode_u_32(arr[0]),
      height: dco_decode_u_32(arr[1]),
      refreshRates: dco_decode_list_prim_u_32_strict(arr[2]),
    );
  }

  @protected
  Monitor dco_decode_monitor(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    final arr = raw as List<dynamic>;
    if (arr.length != 4)
      throw Exception('unexpected arr length: expect 4 but see ${arr.length}');
    return Monitor(
      id: dco_decode_u_32(arr[0]),
      name: dco_decode_opt_String(arr[1]),
      enabled: dco_decode_bool(arr[2]),
      modes: dco_decode_list_mode(arr[3]),
    );
  }

  @protected
  String? dco_decode_opt_String(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw == null ? null : dco_decode_String(raw);
  }

  @protected
  ReceiveError dco_decode_receive_error(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    final arr = raw as List<dynamic>;
    if (arr.length != 1)
      throw Exception('unexpected arr length: expect 1 but see ${arr.length}');
    return ReceiveError(
      message: dco_decode_String(arr[0]),
    );
  }

  @protected
  RequestError dco_decode_request_error(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    switch (raw[0]) {
      case 0:
        return RequestError_Send(
          message: dco_decode_String(raw[1]),
        );
      case 1:
        return RequestError_Receive(
          message: dco_decode_String(raw[1]),
        );
      case 2:
        return RequestError_Timeout(
          duration: dco_decode_Chrono_Duration(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  @protected
  SendError dco_decode_send_error(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    switch (raw[0]) {
      case 0:
        return SendError_PipeBroken(
          message: dco_decode_String(raw[1]),
        );
      default:
        throw Exception("unreachable");
    }
  }

  @protected
  int dco_decode_u_32(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as int;
  }

  @protected
  int dco_decode_u_8(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as int;
  }

  @protected
  void dco_decode_unit(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return;
  }

  @protected
  BigInt dco_decode_usize(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dcoDecodeU64(raw);
  }

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_String(deserializer);
    return AnyhowException(inner);
  }

  @protected
  Client
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return ClientImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  Client
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return ClientImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  Duration sse_decode_Chrono_Duration(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_i_64(deserializer);
    return Duration(microseconds: inner.toInt());
  }

  @protected
  Client
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return ClientImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  RustStreamSink<List<Monitor>> sse_decode_StreamSink_list_monitor_Sse(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    throw UnimplementedError('Unreachable ()');
  }

  @protected
  String sse_decode_String(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_list_prim_u_8_strict(deserializer);
    return utf8.decoder.convert(inner);
  }

  @protected
  bool sse_decode_bool(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8() != 0;
  }

  @protected
  ConnectionError sse_decode_connection_error(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    var tag_ = sse_decode_i_32(deserializer);
    switch (tag_) {
      case 0:
        var var_message = sse_decode_String(deserializer);
        return ConnectionError_Failed(message: var_message);
      default:
        throw UnimplementedError('');
    }
  }

  @protected
  PlatformInt64 sse_decode_i_64(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getPlatformInt64();
  }

  @protected
  List<Mode> sse_decode_list_mode(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <Mode>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_mode(deserializer));
    }
    return ans_;
  }

  @protected
  List<Monitor> sse_decode_list_monitor(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <Monitor>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_monitor(deserializer));
    }
    return ans_;
  }

  @protected
  List<int> sse_decode_list_prim_u_32_loose(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint32List(len_);
  }

  @protected
  Uint32List sse_decode_list_prim_u_32_strict(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint32List(len_);
  }

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
  }

  @protected
  Mode sse_decode_mode(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var var_width = sse_decode_u_32(deserializer);
    var var_height = sse_decode_u_32(deserializer);
    var var_refreshRates = sse_decode_list_prim_u_32_strict(deserializer);
    return Mode(
        width: var_width, height: var_height, refreshRates: var_refreshRates);
  }

  @protected
  Monitor sse_decode_monitor(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var var_id = sse_decode_u_32(deserializer);
    var var_name = sse_decode_opt_String(deserializer);
    var var_enabled = sse_decode_bool(deserializer);
    var var_modes = sse_decode_list_mode(deserializer);
    return Monitor(
        id: var_id, name: var_name, enabled: var_enabled, modes: var_modes);
  }

  @protected
  String? sse_decode_opt_String(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    if (sse_decode_bool(deserializer)) {
      return (sse_decode_String(deserializer));
    } else {
      return null;
    }
  }

  @protected
  ReceiveError sse_decode_receive_error(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var var_message = sse_decode_String(deserializer);
    return ReceiveError(message: var_message);
  }

  @protected
  RequestError sse_decode_request_error(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    var tag_ = sse_decode_i_32(deserializer);
    switch (tag_) {
      case 0:
        var var_message = sse_decode_String(deserializer);
        return RequestError_Send(message: var_message);
      case 1:
        var var_message = sse_decode_String(deserializer);
        return RequestError_Receive(message: var_message);
      case 2:
        var var_duration = sse_decode_Chrono_Duration(deserializer);
        return RequestError_Timeout(duration: var_duration);
      default:
        throw UnimplementedError('');
    }
  }

  @protected
  SendError sse_decode_send_error(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    var tag_ = sse_decode_i_32(deserializer);
    switch (tag_) {
      case 0:
        var var_message = sse_decode_String(deserializer);
        return SendError_PipeBroken(message: var_message);
      default:
        throw UnimplementedError('');
    }
  }

  @protected
  int sse_decode_u_32(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint32();
  }

  @protected
  int sse_decode_u_8(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8();
  }

  @protected
  void sse_decode_unit(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  BigInt sse_decode_usize(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getBigUint64();
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getInt32();
  }

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_String(self.message, serializer);
  }

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          Client self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as ClientImpl).frbInternalSseEncode(move: true), serializer);
  }

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          Client self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as ClientImpl).frbInternalSseEncode(move: false), serializer);
  }

  @protected
  void sse_encode_Chrono_Duration(Duration self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_64(PlatformInt64Util.from(self.inMicroseconds), serializer);
  }

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerClient(
          Client self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as ClientImpl).frbInternalSseEncode(move: null), serializer);
  }

  @protected
  void sse_encode_StreamSink_list_monitor_Sse(
      RustStreamSink<List<Monitor>> self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_String(
        self.setupAndSerialize(
            codec: SseCodec(
          decodeSuccessData: sse_decode_list_monitor,
          decodeErrorData: sse_decode_AnyhowException,
        )),
        serializer);
  }

  @protected
  void sse_encode_String(String self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_list_prim_u_8_strict(utf8.encoder.convert(self), serializer);
  }

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self ? 1 : 0);
  }

  @protected
  void sse_encode_connection_error(
      ConnectionError self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    switch (self) {
      case ConnectionError_Failed(message: final message):
        sse_encode_i_32(0, serializer);
        sse_encode_String(message, serializer);
      default:
        throw UnimplementedError('');
    }
  }

  @protected
  void sse_encode_i_64(PlatformInt64 self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putPlatformInt64(self);
  }

  @protected
  void sse_encode_list_mode(List<Mode> self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_mode(item, serializer);
    }
  }

  @protected
  void sse_encode_list_monitor(List<Monitor> self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_monitor(item, serializer);
    }
  }

  @protected
  void sse_encode_list_prim_u_32_loose(
      List<int> self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    serializer.buffer
        .putUint32List(self is Uint32List ? self : Uint32List.fromList(self));
  }

  @protected
  void sse_encode_list_prim_u_32_strict(
      Uint32List self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    serializer.buffer.putUint32List(self);
  }

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    serializer.buffer.putUint8List(self);
  }

  @protected
  void sse_encode_mode(Mode self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_u_32(self.width, serializer);
    sse_encode_u_32(self.height, serializer);
    sse_encode_list_prim_u_32_strict(self.refreshRates, serializer);
  }

  @protected
  void sse_encode_monitor(Monitor self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_u_32(self.id, serializer);
    sse_encode_opt_String(self.name, serializer);
    sse_encode_bool(self.enabled, serializer);
    sse_encode_list_mode(self.modes, serializer);
  }

  @protected
  void sse_encode_opt_String(String? self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    sse_encode_bool(self != null, serializer);
    if (self != null) {
      sse_encode_String(self, serializer);
    }
  }

  @protected
  void sse_encode_receive_error(ReceiveError self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_String(self.message, serializer);
  }

  @protected
  void sse_encode_request_error(RequestError self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    switch (self) {
      case RequestError_Send(message: final message):
        sse_encode_i_32(0, serializer);
        sse_encode_String(message, serializer);
      case RequestError_Receive(message: final message):
        sse_encode_i_32(1, serializer);
        sse_encode_String(message, serializer);
      case RequestError_Timeout(duration: final duration):
        sse_encode_i_32(2, serializer);
        sse_encode_Chrono_Duration(duration, serializer);
      default:
        throw UnimplementedError('');
    }
  }

  @protected
  void sse_encode_send_error(SendError self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    switch (self) {
      case SendError_PipeBroken(message: final message):
        sse_encode_i_32(0, serializer);
        sse_encode_String(message, serializer);
      default:
        throw UnimplementedError('');
    }
  }

  @protected
  void sse_encode_u_32(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint32(self);
  }

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self);
  }

  @protected
  void sse_encode_unit(void self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  void sse_encode_usize(BigInt self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putBigUint64(self);
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putInt32(self);
  }
}

@sealed
class ClientImpl extends RustOpaque implements Client {
  // Not to be used by end users
  ClientImpl.frbInternalDcoDecode(List<dynamic> wire)
      : super.frbInternalDcoDecode(wire, _kStaticData);

  // Not to be used by end users
  ClientImpl.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative)
      : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_Client,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_Client,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_ClientPtr,
  );

  /// Send new state to the driver.
  Future<void> notify({required List<Monitor> monitors}) => RustLib.instance.api
      .crateApiClientClientNotify(that: this, monitors: monitors);

  /// Receive continuous events from the driver.
  ///
  /// Only new events after calling this method are received.
  ///
  /// May be called multiple times.
  Stream<List<Monitor>> receiveEvents() =>
      RustLib.instance.api.crateApiClientClientReceiveEvents(
        that: this,
      );

  /// Remove all monitors with the specified IDs.
  Future<void> remove({required List<int> ids}) =>
      RustLib.instance.api.crateApiClientClientRemove(that: this, ids: ids);

  /// Remove all monitors.
  Future<void> removeAll() =>
      RustLib.instance.api.crateApiClientClientRemoveAll(
        that: this,
      );

  /// Request the current state of the driver.
  ///
  /// Throws [RequestError.timeout] if the driver does not respond within 5
  /// seconds.
  Future<List<Monitor>> requestState() =>
      RustLib.instance.api.crateApiClientClientRequestState(
        that: this,
      );
}
