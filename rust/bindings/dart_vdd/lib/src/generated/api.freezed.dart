// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$IpcError {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) serDe,
    required TResult Function(String field0) io,
    required TResult Function(String field0) win,
    required TResult Function(String field0) client,
    required TResult Function() requestState,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? serDe,
    TResult? Function(String field0)? io,
    TResult? Function(String field0)? win,
    TResult? Function(String field0)? client,
    TResult? Function()? requestState,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? serDe,
    TResult Function(String field0)? io,
    TResult Function(String field0)? win,
    TResult Function(String field0)? client,
    TResult Function()? requestState,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(IpcError_SerDe value) serDe,
    required TResult Function(IpcError_Io value) io,
    required TResult Function(IpcError_Win value) win,
    required TResult Function(IpcError_Client value) client,
    required TResult Function(IpcError_RequestState value) requestState,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(IpcError_SerDe value)? serDe,
    TResult? Function(IpcError_Io value)? io,
    TResult? Function(IpcError_Win value)? win,
    TResult? Function(IpcError_Client value)? client,
    TResult? Function(IpcError_RequestState value)? requestState,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(IpcError_SerDe value)? serDe,
    TResult Function(IpcError_Io value)? io,
    TResult Function(IpcError_Win value)? win,
    TResult Function(IpcError_Client value)? client,
    TResult Function(IpcError_RequestState value)? requestState,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $IpcErrorCopyWith<$Res> {
  factory $IpcErrorCopyWith(IpcError value, $Res Function(IpcError) then) =
      _$IpcErrorCopyWithImpl<$Res, IpcError>;
}

/// @nodoc
class _$IpcErrorCopyWithImpl<$Res, $Val extends IpcError>
    implements $IpcErrorCopyWith<$Res> {
  _$IpcErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$IpcError_SerDeImplCopyWith<$Res> {
  factory _$$IpcError_SerDeImplCopyWith(_$IpcError_SerDeImpl value,
          $Res Function(_$IpcError_SerDeImpl) then) =
      __$$IpcError_SerDeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$IpcError_SerDeImplCopyWithImpl<$Res>
    extends _$IpcErrorCopyWithImpl<$Res, _$IpcError_SerDeImpl>
    implements _$$IpcError_SerDeImplCopyWith<$Res> {
  __$$IpcError_SerDeImplCopyWithImpl(
      _$IpcError_SerDeImpl _value, $Res Function(_$IpcError_SerDeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$IpcError_SerDeImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$IpcError_SerDeImpl extends IpcError_SerDe {
  const _$IpcError_SerDeImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'IpcError.serDe(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$IpcError_SerDeImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$IpcError_SerDeImplCopyWith<_$IpcError_SerDeImpl> get copyWith =>
      __$$IpcError_SerDeImplCopyWithImpl<_$IpcError_SerDeImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) serDe,
    required TResult Function(String field0) io,
    required TResult Function(String field0) win,
    required TResult Function(String field0) client,
    required TResult Function() requestState,
  }) {
    return serDe(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? serDe,
    TResult? Function(String field0)? io,
    TResult? Function(String field0)? win,
    TResult? Function(String field0)? client,
    TResult? Function()? requestState,
  }) {
    return serDe?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? serDe,
    TResult Function(String field0)? io,
    TResult Function(String field0)? win,
    TResult Function(String field0)? client,
    TResult Function()? requestState,
    required TResult orElse(),
  }) {
    if (serDe != null) {
      return serDe(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(IpcError_SerDe value) serDe,
    required TResult Function(IpcError_Io value) io,
    required TResult Function(IpcError_Win value) win,
    required TResult Function(IpcError_Client value) client,
    required TResult Function(IpcError_RequestState value) requestState,
  }) {
    return serDe(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(IpcError_SerDe value)? serDe,
    TResult? Function(IpcError_Io value)? io,
    TResult? Function(IpcError_Win value)? win,
    TResult? Function(IpcError_Client value)? client,
    TResult? Function(IpcError_RequestState value)? requestState,
  }) {
    return serDe?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(IpcError_SerDe value)? serDe,
    TResult Function(IpcError_Io value)? io,
    TResult Function(IpcError_Win value)? win,
    TResult Function(IpcError_Client value)? client,
    TResult Function(IpcError_RequestState value)? requestState,
    required TResult orElse(),
  }) {
    if (serDe != null) {
      return serDe(this);
    }
    return orElse();
  }
}

abstract class IpcError_SerDe extends IpcError {
  const factory IpcError_SerDe(final String field0) = _$IpcError_SerDeImpl;
  const IpcError_SerDe._() : super._();

  String get field0;
  @JsonKey(ignore: true)
  _$$IpcError_SerDeImplCopyWith<_$IpcError_SerDeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$IpcError_IoImplCopyWith<$Res> {
  factory _$$IpcError_IoImplCopyWith(
          _$IpcError_IoImpl value, $Res Function(_$IpcError_IoImpl) then) =
      __$$IpcError_IoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$IpcError_IoImplCopyWithImpl<$Res>
    extends _$IpcErrorCopyWithImpl<$Res, _$IpcError_IoImpl>
    implements _$$IpcError_IoImplCopyWith<$Res> {
  __$$IpcError_IoImplCopyWithImpl(
      _$IpcError_IoImpl _value, $Res Function(_$IpcError_IoImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$IpcError_IoImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$IpcError_IoImpl extends IpcError_Io {
  const _$IpcError_IoImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'IpcError.io(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$IpcError_IoImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$IpcError_IoImplCopyWith<_$IpcError_IoImpl> get copyWith =>
      __$$IpcError_IoImplCopyWithImpl<_$IpcError_IoImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) serDe,
    required TResult Function(String field0) io,
    required TResult Function(String field0) win,
    required TResult Function(String field0) client,
    required TResult Function() requestState,
  }) {
    return io(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? serDe,
    TResult? Function(String field0)? io,
    TResult? Function(String field0)? win,
    TResult? Function(String field0)? client,
    TResult? Function()? requestState,
  }) {
    return io?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? serDe,
    TResult Function(String field0)? io,
    TResult Function(String field0)? win,
    TResult Function(String field0)? client,
    TResult Function()? requestState,
    required TResult orElse(),
  }) {
    if (io != null) {
      return io(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(IpcError_SerDe value) serDe,
    required TResult Function(IpcError_Io value) io,
    required TResult Function(IpcError_Win value) win,
    required TResult Function(IpcError_Client value) client,
    required TResult Function(IpcError_RequestState value) requestState,
  }) {
    return io(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(IpcError_SerDe value)? serDe,
    TResult? Function(IpcError_Io value)? io,
    TResult? Function(IpcError_Win value)? win,
    TResult? Function(IpcError_Client value)? client,
    TResult? Function(IpcError_RequestState value)? requestState,
  }) {
    return io?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(IpcError_SerDe value)? serDe,
    TResult Function(IpcError_Io value)? io,
    TResult Function(IpcError_Win value)? win,
    TResult Function(IpcError_Client value)? client,
    TResult Function(IpcError_RequestState value)? requestState,
    required TResult orElse(),
  }) {
    if (io != null) {
      return io(this);
    }
    return orElse();
  }
}

abstract class IpcError_Io extends IpcError {
  const factory IpcError_Io(final String field0) = _$IpcError_IoImpl;
  const IpcError_Io._() : super._();

  String get field0;
  @JsonKey(ignore: true)
  _$$IpcError_IoImplCopyWith<_$IpcError_IoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$IpcError_WinImplCopyWith<$Res> {
  factory _$$IpcError_WinImplCopyWith(
          _$IpcError_WinImpl value, $Res Function(_$IpcError_WinImpl) then) =
      __$$IpcError_WinImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$IpcError_WinImplCopyWithImpl<$Res>
    extends _$IpcErrorCopyWithImpl<$Res, _$IpcError_WinImpl>
    implements _$$IpcError_WinImplCopyWith<$Res> {
  __$$IpcError_WinImplCopyWithImpl(
      _$IpcError_WinImpl _value, $Res Function(_$IpcError_WinImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$IpcError_WinImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$IpcError_WinImpl extends IpcError_Win {
  const _$IpcError_WinImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'IpcError.win(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$IpcError_WinImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$IpcError_WinImplCopyWith<_$IpcError_WinImpl> get copyWith =>
      __$$IpcError_WinImplCopyWithImpl<_$IpcError_WinImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) serDe,
    required TResult Function(String field0) io,
    required TResult Function(String field0) win,
    required TResult Function(String field0) client,
    required TResult Function() requestState,
  }) {
    return win(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? serDe,
    TResult? Function(String field0)? io,
    TResult? Function(String field0)? win,
    TResult? Function(String field0)? client,
    TResult? Function()? requestState,
  }) {
    return win?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? serDe,
    TResult Function(String field0)? io,
    TResult Function(String field0)? win,
    TResult Function(String field0)? client,
    TResult Function()? requestState,
    required TResult orElse(),
  }) {
    if (win != null) {
      return win(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(IpcError_SerDe value) serDe,
    required TResult Function(IpcError_Io value) io,
    required TResult Function(IpcError_Win value) win,
    required TResult Function(IpcError_Client value) client,
    required TResult Function(IpcError_RequestState value) requestState,
  }) {
    return win(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(IpcError_SerDe value)? serDe,
    TResult? Function(IpcError_Io value)? io,
    TResult? Function(IpcError_Win value)? win,
    TResult? Function(IpcError_Client value)? client,
    TResult? Function(IpcError_RequestState value)? requestState,
  }) {
    return win?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(IpcError_SerDe value)? serDe,
    TResult Function(IpcError_Io value)? io,
    TResult Function(IpcError_Win value)? win,
    TResult Function(IpcError_Client value)? client,
    TResult Function(IpcError_RequestState value)? requestState,
    required TResult orElse(),
  }) {
    if (win != null) {
      return win(this);
    }
    return orElse();
  }
}

abstract class IpcError_Win extends IpcError {
  const factory IpcError_Win(final String field0) = _$IpcError_WinImpl;
  const IpcError_Win._() : super._();

  String get field0;
  @JsonKey(ignore: true)
  _$$IpcError_WinImplCopyWith<_$IpcError_WinImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$IpcError_ClientImplCopyWith<$Res> {
  factory _$$IpcError_ClientImplCopyWith(_$IpcError_ClientImpl value,
          $Res Function(_$IpcError_ClientImpl) then) =
      __$$IpcError_ClientImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String field0});
}

/// @nodoc
class __$$IpcError_ClientImplCopyWithImpl<$Res>
    extends _$IpcErrorCopyWithImpl<$Res, _$IpcError_ClientImpl>
    implements _$$IpcError_ClientImplCopyWith<$Res> {
  __$$IpcError_ClientImplCopyWithImpl(
      _$IpcError_ClientImpl _value, $Res Function(_$IpcError_ClientImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$IpcError_ClientImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$IpcError_ClientImpl extends IpcError_Client {
  const _$IpcError_ClientImpl(this.field0) : super._();

  @override
  final String field0;

  @override
  String toString() {
    return 'IpcError.client(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$IpcError_ClientImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$IpcError_ClientImplCopyWith<_$IpcError_ClientImpl> get copyWith =>
      __$$IpcError_ClientImplCopyWithImpl<_$IpcError_ClientImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) serDe,
    required TResult Function(String field0) io,
    required TResult Function(String field0) win,
    required TResult Function(String field0) client,
    required TResult Function() requestState,
  }) {
    return client(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? serDe,
    TResult? Function(String field0)? io,
    TResult? Function(String field0)? win,
    TResult? Function(String field0)? client,
    TResult? Function()? requestState,
  }) {
    return client?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? serDe,
    TResult Function(String field0)? io,
    TResult Function(String field0)? win,
    TResult Function(String field0)? client,
    TResult Function()? requestState,
    required TResult orElse(),
  }) {
    if (client != null) {
      return client(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(IpcError_SerDe value) serDe,
    required TResult Function(IpcError_Io value) io,
    required TResult Function(IpcError_Win value) win,
    required TResult Function(IpcError_Client value) client,
    required TResult Function(IpcError_RequestState value) requestState,
  }) {
    return client(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(IpcError_SerDe value)? serDe,
    TResult? Function(IpcError_Io value)? io,
    TResult? Function(IpcError_Win value)? win,
    TResult? Function(IpcError_Client value)? client,
    TResult? Function(IpcError_RequestState value)? requestState,
  }) {
    return client?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(IpcError_SerDe value)? serDe,
    TResult Function(IpcError_Io value)? io,
    TResult Function(IpcError_Win value)? win,
    TResult Function(IpcError_Client value)? client,
    TResult Function(IpcError_RequestState value)? requestState,
    required TResult orElse(),
  }) {
    if (client != null) {
      return client(this);
    }
    return orElse();
  }
}

abstract class IpcError_Client extends IpcError {
  const factory IpcError_Client(final String field0) = _$IpcError_ClientImpl;
  const IpcError_Client._() : super._();

  String get field0;
  @JsonKey(ignore: true)
  _$$IpcError_ClientImplCopyWith<_$IpcError_ClientImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$IpcError_RequestStateImplCopyWith<$Res> {
  factory _$$IpcError_RequestStateImplCopyWith(
          _$IpcError_RequestStateImpl value,
          $Res Function(_$IpcError_RequestStateImpl) then) =
      __$$IpcError_RequestStateImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$IpcError_RequestStateImplCopyWithImpl<$Res>
    extends _$IpcErrorCopyWithImpl<$Res, _$IpcError_RequestStateImpl>
    implements _$$IpcError_RequestStateImplCopyWith<$Res> {
  __$$IpcError_RequestStateImplCopyWithImpl(_$IpcError_RequestStateImpl _value,
      $Res Function(_$IpcError_RequestStateImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$IpcError_RequestStateImpl extends IpcError_RequestState {
  const _$IpcError_RequestStateImpl() : super._();

  @override
  String toString() {
    return 'IpcError.requestState()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$IpcError_RequestStateImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String field0) serDe,
    required TResult Function(String field0) io,
    required TResult Function(String field0) win,
    required TResult Function(String field0) client,
    required TResult Function() requestState,
  }) {
    return requestState();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String field0)? serDe,
    TResult? Function(String field0)? io,
    TResult? Function(String field0)? win,
    TResult? Function(String field0)? client,
    TResult? Function()? requestState,
  }) {
    return requestState?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String field0)? serDe,
    TResult Function(String field0)? io,
    TResult Function(String field0)? win,
    TResult Function(String field0)? client,
    TResult Function()? requestState,
    required TResult orElse(),
  }) {
    if (requestState != null) {
      return requestState();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(IpcError_SerDe value) serDe,
    required TResult Function(IpcError_Io value) io,
    required TResult Function(IpcError_Win value) win,
    required TResult Function(IpcError_Client value) client,
    required TResult Function(IpcError_RequestState value) requestState,
  }) {
    return requestState(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(IpcError_SerDe value)? serDe,
    TResult? Function(IpcError_Io value)? io,
    TResult? Function(IpcError_Win value)? win,
    TResult? Function(IpcError_Client value)? client,
    TResult? Function(IpcError_RequestState value)? requestState,
  }) {
    return requestState?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(IpcError_SerDe value)? serDe,
    TResult Function(IpcError_Io value)? io,
    TResult Function(IpcError_Win value)? win,
    TResult Function(IpcError_Client value)? client,
    TResult Function(IpcError_RequestState value)? requestState,
    required TResult orElse(),
  }) {
    if (requestState != null) {
      return requestState(this);
    }
    return orElse();
  }
}

abstract class IpcError_RequestState extends IpcError {
  const factory IpcError_RequestState() = _$IpcError_RequestStateImpl;
  const IpcError_RequestState._() : super._();
}

/// @nodoc
mixin _$Mode {
  int get width => throw _privateConstructorUsedError;
  int get height => throw _privateConstructorUsedError;
  Uint32List get refreshRates => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $ModeCopyWith<Mode> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ModeCopyWith<$Res> {
  factory $ModeCopyWith(Mode value, $Res Function(Mode) then) =
      _$ModeCopyWithImpl<$Res, Mode>;
  @useResult
  $Res call({int width, int height, Uint32List refreshRates});
}

/// @nodoc
class _$ModeCopyWithImpl<$Res, $Val extends Mode>
    implements $ModeCopyWith<$Res> {
  _$ModeCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? width = null,
    Object? height = null,
    Object? refreshRates = null,
  }) {
    return _then(_value.copyWith(
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
      refreshRates: null == refreshRates
          ? _value.refreshRates
          : refreshRates // ignore: cast_nullable_to_non_nullable
              as Uint32List,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ModeImplCopyWith<$Res> implements $ModeCopyWith<$Res> {
  factory _$$ModeImplCopyWith(
          _$ModeImpl value, $Res Function(_$ModeImpl) then) =
      __$$ModeImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int width, int height, Uint32List refreshRates});
}

/// @nodoc
class __$$ModeImplCopyWithImpl<$Res>
    extends _$ModeCopyWithImpl<$Res, _$ModeImpl>
    implements _$$ModeImplCopyWith<$Res> {
  __$$ModeImplCopyWithImpl(_$ModeImpl _value, $Res Function(_$ModeImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? width = null,
    Object? height = null,
    Object? refreshRates = null,
  }) {
    return _then(_$ModeImpl(
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
      refreshRates: null == refreshRates
          ? _value.refreshRates
          : refreshRates // ignore: cast_nullable_to_non_nullable
              as Uint32List,
    ));
  }
}

/// @nodoc

class _$ModeImpl implements _Mode {
  const _$ModeImpl(
      {required this.width, required this.height, required this.refreshRates});

  @override
  final int width;
  @override
  final int height;
  @override
  final Uint32List refreshRates;

  @override
  String toString() {
    return 'Mode(width: $width, height: $height, refreshRates: $refreshRates)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ModeImpl &&
            (identical(other.width, width) || other.width == width) &&
            (identical(other.height, height) || other.height == height) &&
            const DeepCollectionEquality()
                .equals(other.refreshRates, refreshRates));
  }

  @override
  int get hashCode => Object.hash(runtimeType, width, height,
      const DeepCollectionEquality().hash(refreshRates));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$ModeImplCopyWith<_$ModeImpl> get copyWith =>
      __$$ModeImplCopyWithImpl<_$ModeImpl>(this, _$identity);
}

abstract class _Mode implements Mode {
  const factory _Mode(
      {required final int width,
      required final int height,
      required final Uint32List refreshRates}) = _$ModeImpl;

  @override
  int get width;
  @override
  int get height;
  @override
  Uint32List get refreshRates;
  @override
  @JsonKey(ignore: true)
  _$$ModeImplCopyWith<_$ModeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$Monitor {
  int get id => throw _privateConstructorUsedError;
  String? get name => throw _privateConstructorUsedError;
  bool get enabled => throw _privateConstructorUsedError;
  List<Mode> get modes => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $MonitorCopyWith<Monitor> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MonitorCopyWith<$Res> {
  factory $MonitorCopyWith(Monitor value, $Res Function(Monitor) then) =
      _$MonitorCopyWithImpl<$Res, Monitor>;
  @useResult
  $Res call({int id, String? name, bool enabled, List<Mode> modes});
}

/// @nodoc
class _$MonitorCopyWithImpl<$Res, $Val extends Monitor>
    implements $MonitorCopyWith<$Res> {
  _$MonitorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = freezed,
    Object? enabled = null,
    Object? modes = null,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as int,
      name: freezed == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String?,
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      modes: null == modes
          ? _value.modes
          : modes // ignore: cast_nullable_to_non_nullable
              as List<Mode>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$MonitorImplCopyWith<$Res> implements $MonitorCopyWith<$Res> {
  factory _$$MonitorImplCopyWith(
          _$MonitorImpl value, $Res Function(_$MonitorImpl) then) =
      __$$MonitorImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int id, String? name, bool enabled, List<Mode> modes});
}

/// @nodoc
class __$$MonitorImplCopyWithImpl<$Res>
    extends _$MonitorCopyWithImpl<$Res, _$MonitorImpl>
    implements _$$MonitorImplCopyWith<$Res> {
  __$$MonitorImplCopyWithImpl(
      _$MonitorImpl _value, $Res Function(_$MonitorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = freezed,
    Object? enabled = null,
    Object? modes = null,
  }) {
    return _then(_$MonitorImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as int,
      name: freezed == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String?,
      enabled: null == enabled
          ? _value.enabled
          : enabled // ignore: cast_nullable_to_non_nullable
              as bool,
      modes: null == modes
          ? _value._modes
          : modes // ignore: cast_nullable_to_non_nullable
              as List<Mode>,
    ));
  }
}

/// @nodoc

class _$MonitorImpl implements _Monitor {
  const _$MonitorImpl(
      {required this.id,
      this.name,
      required this.enabled,
      required final List<Mode> modes})
      : _modes = modes;

  @override
  final int id;
  @override
  final String? name;
  @override
  final bool enabled;
  final List<Mode> _modes;
  @override
  List<Mode> get modes {
    if (_modes is EqualUnmodifiableListView) return _modes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_modes);
  }

  @override
  String toString() {
    return 'Monitor(id: $id, name: $name, enabled: $enabled, modes: $modes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MonitorImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.enabled, enabled) || other.enabled == enabled) &&
            const DeepCollectionEquality().equals(other._modes, _modes));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, name, enabled,
      const DeepCollectionEquality().hash(_modes));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MonitorImplCopyWith<_$MonitorImpl> get copyWith =>
      __$$MonitorImplCopyWithImpl<_$MonitorImpl>(this, _$identity);
}

abstract class _Monitor implements Monitor {
  const factory _Monitor(
      {required final int id,
      final String? name,
      required final bool enabled,
      required final List<Mode> modes}) = _$MonitorImpl;

  @override
  int get id;
  @override
  String? get name;
  @override
  bool get enabled;
  @override
  List<Mode> get modes;
  @override
  @JsonKey(ignore: true)
  _$$MonitorImplCopyWith<_$MonitorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
