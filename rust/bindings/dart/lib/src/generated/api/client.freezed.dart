// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'client.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$ConnectionError {
  String get message => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) failed,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? failed,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? failed,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Failed value) failed,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Failed value)? failed,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Failed value)? failed,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Create a copy of ConnectionError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $ConnectionErrorCopyWith<ConnectionError> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ConnectionErrorCopyWith<$Res> {
  factory $ConnectionErrorCopyWith(
          ConnectionError value, $Res Function(ConnectionError) then) =
      _$ConnectionErrorCopyWithImpl<$Res, ConnectionError>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class _$ConnectionErrorCopyWithImpl<$Res, $Val extends ConnectionError>
    implements $ConnectionErrorCopyWith<$Res> {
  _$ConnectionErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ConnectionError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_value.copyWith(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ConnectionError_FailedImplCopyWith<$Res>
    implements $ConnectionErrorCopyWith<$Res> {
  factory _$$ConnectionError_FailedImplCopyWith(
          _$ConnectionError_FailedImpl value,
          $Res Function(_$ConnectionError_FailedImpl) then) =
      __$$ConnectionError_FailedImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$ConnectionError_FailedImplCopyWithImpl<$Res>
    extends _$ConnectionErrorCopyWithImpl<$Res, _$ConnectionError_FailedImpl>
    implements _$$ConnectionError_FailedImplCopyWith<$Res> {
  __$$ConnectionError_FailedImplCopyWithImpl(
      _$ConnectionError_FailedImpl _value,
      $Res Function(_$ConnectionError_FailedImpl) _then)
      : super(_value, _then);

  /// Create a copy of ConnectionError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$ConnectionError_FailedImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$ConnectionError_FailedImpl extends ConnectionError_Failed {
  const _$ConnectionError_FailedImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConnectionError_FailedImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of ConnectionError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ConnectionError_FailedImplCopyWith<_$ConnectionError_FailedImpl>
      get copyWith => __$$ConnectionError_FailedImplCopyWithImpl<
          _$ConnectionError_FailedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) failed,
  }) {
    return failed(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? failed,
  }) {
    return failed?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? failed,
    required TResult orElse(),
  }) {
    if (failed != null) {
      return failed(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConnectionError_Failed value) failed,
  }) {
    return failed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError_Failed value)? failed,
  }) {
    return failed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConnectionError_Failed value)? failed,
    required TResult orElse(),
  }) {
    if (failed != null) {
      return failed(this);
    }
    return orElse();
  }
}

abstract class ConnectionError_Failed extends ConnectionError {
  const factory ConnectionError_Failed({required final String message}) =
      _$ConnectionError_FailedImpl;
  const ConnectionError_Failed._() : super._();

  @override
  String get message;

  /// Create a copy of ConnectionError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ConnectionError_FailedImplCopyWith<_$ConnectionError_FailedImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$PersistError {
  String get message => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) open,
    required TResult Function(String message) set_,
    required TResult Function(String message) serialize,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? open,
    TResult? Function(String message)? set_,
    TResult? Function(String message)? serialize,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? open,
    TResult Function(String message)? set_,
    TResult Function(String message)? serialize,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PersistError_Open value) open,
    required TResult Function(PersistError_Set value) set_,
    required TResult Function(PersistError_Serialize value) serialize,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PersistError_Open value)? open,
    TResult? Function(PersistError_Set value)? set_,
    TResult? Function(PersistError_Serialize value)? serialize,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PersistError_Open value)? open,
    TResult Function(PersistError_Set value)? set_,
    TResult Function(PersistError_Serialize value)? serialize,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $PersistErrorCopyWith<PersistError> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PersistErrorCopyWith<$Res> {
  factory $PersistErrorCopyWith(
          PersistError value, $Res Function(PersistError) then) =
      _$PersistErrorCopyWithImpl<$Res, PersistError>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class _$PersistErrorCopyWithImpl<$Res, $Val extends PersistError>
    implements $PersistErrorCopyWith<$Res> {
  _$PersistErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_value.copyWith(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$PersistError_OpenImplCopyWith<$Res>
    implements $PersistErrorCopyWith<$Res> {
  factory _$$PersistError_OpenImplCopyWith(_$PersistError_OpenImpl value,
          $Res Function(_$PersistError_OpenImpl) then) =
      __$$PersistError_OpenImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$PersistError_OpenImplCopyWithImpl<$Res>
    extends _$PersistErrorCopyWithImpl<$Res, _$PersistError_OpenImpl>
    implements _$$PersistError_OpenImplCopyWith<$Res> {
  __$$PersistError_OpenImplCopyWithImpl(_$PersistError_OpenImpl _value,
      $Res Function(_$PersistError_OpenImpl) _then)
      : super(_value, _then);

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$PersistError_OpenImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PersistError_OpenImpl extends PersistError_Open {
  const _$PersistError_OpenImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PersistError_OpenImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PersistError_OpenImplCopyWith<_$PersistError_OpenImpl> get copyWith =>
      __$$PersistError_OpenImplCopyWithImpl<_$PersistError_OpenImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) open,
    required TResult Function(String message) set_,
    required TResult Function(String message) serialize,
  }) {
    return open(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? open,
    TResult? Function(String message)? set_,
    TResult? Function(String message)? serialize,
  }) {
    return open?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? open,
    TResult Function(String message)? set_,
    TResult Function(String message)? serialize,
    required TResult orElse(),
  }) {
    if (open != null) {
      return open(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PersistError_Open value) open,
    required TResult Function(PersistError_Set value) set_,
    required TResult Function(PersistError_Serialize value) serialize,
  }) {
    return open(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PersistError_Open value)? open,
    TResult? Function(PersistError_Set value)? set_,
    TResult? Function(PersistError_Serialize value)? serialize,
  }) {
    return open?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PersistError_Open value)? open,
    TResult Function(PersistError_Set value)? set_,
    TResult Function(PersistError_Serialize value)? serialize,
    required TResult orElse(),
  }) {
    if (open != null) {
      return open(this);
    }
    return orElse();
  }
}

abstract class PersistError_Open extends PersistError {
  const factory PersistError_Open({required final String message}) =
      _$PersistError_OpenImpl;
  const PersistError_Open._() : super._();

  @override
  String get message;

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PersistError_OpenImplCopyWith<_$PersistError_OpenImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PersistError_SetImplCopyWith<$Res>
    implements $PersistErrorCopyWith<$Res> {
  factory _$$PersistError_SetImplCopyWith(_$PersistError_SetImpl value,
          $Res Function(_$PersistError_SetImpl) then) =
      __$$PersistError_SetImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$PersistError_SetImplCopyWithImpl<$Res>
    extends _$PersistErrorCopyWithImpl<$Res, _$PersistError_SetImpl>
    implements _$$PersistError_SetImplCopyWith<$Res> {
  __$$PersistError_SetImplCopyWithImpl(_$PersistError_SetImpl _value,
      $Res Function(_$PersistError_SetImpl) _then)
      : super(_value, _then);

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$PersistError_SetImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PersistError_SetImpl extends PersistError_Set {
  const _$PersistError_SetImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PersistError_SetImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PersistError_SetImplCopyWith<_$PersistError_SetImpl> get copyWith =>
      __$$PersistError_SetImplCopyWithImpl<_$PersistError_SetImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) open,
    required TResult Function(String message) set_,
    required TResult Function(String message) serialize,
  }) {
    return set_(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? open,
    TResult? Function(String message)? set_,
    TResult? Function(String message)? serialize,
  }) {
    return set_?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? open,
    TResult Function(String message)? set_,
    TResult Function(String message)? serialize,
    required TResult orElse(),
  }) {
    if (set_ != null) {
      return set_(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PersistError_Open value) open,
    required TResult Function(PersistError_Set value) set_,
    required TResult Function(PersistError_Serialize value) serialize,
  }) {
    return set_(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PersistError_Open value)? open,
    TResult? Function(PersistError_Set value)? set_,
    TResult? Function(PersistError_Serialize value)? serialize,
  }) {
    return set_?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PersistError_Open value)? open,
    TResult Function(PersistError_Set value)? set_,
    TResult Function(PersistError_Serialize value)? serialize,
    required TResult orElse(),
  }) {
    if (set_ != null) {
      return set_(this);
    }
    return orElse();
  }
}

abstract class PersistError_Set extends PersistError {
  const factory PersistError_Set({required final String message}) =
      _$PersistError_SetImpl;
  const PersistError_Set._() : super._();

  @override
  String get message;

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PersistError_SetImplCopyWith<_$PersistError_SetImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PersistError_SerializeImplCopyWith<$Res>
    implements $PersistErrorCopyWith<$Res> {
  factory _$$PersistError_SerializeImplCopyWith(
          _$PersistError_SerializeImpl value,
          $Res Function(_$PersistError_SerializeImpl) then) =
      __$$PersistError_SerializeImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$PersistError_SerializeImplCopyWithImpl<$Res>
    extends _$PersistErrorCopyWithImpl<$Res, _$PersistError_SerializeImpl>
    implements _$$PersistError_SerializeImplCopyWith<$Res> {
  __$$PersistError_SerializeImplCopyWithImpl(
      _$PersistError_SerializeImpl _value,
      $Res Function(_$PersistError_SerializeImpl) _then)
      : super(_value, _then);

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$PersistError_SerializeImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$PersistError_SerializeImpl extends PersistError_Serialize {
  const _$PersistError_SerializeImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PersistError_SerializeImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PersistError_SerializeImplCopyWith<_$PersistError_SerializeImpl>
      get copyWith => __$$PersistError_SerializeImplCopyWithImpl<
          _$PersistError_SerializeImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) open,
    required TResult Function(String message) set_,
    required TResult Function(String message) serialize,
  }) {
    return serialize(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? open,
    TResult? Function(String message)? set_,
    TResult? Function(String message)? serialize,
  }) {
    return serialize?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? open,
    TResult Function(String message)? set_,
    TResult Function(String message)? serialize,
    required TResult orElse(),
  }) {
    if (serialize != null) {
      return serialize(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PersistError_Open value) open,
    required TResult Function(PersistError_Set value) set_,
    required TResult Function(PersistError_Serialize value) serialize,
  }) {
    return serialize(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PersistError_Open value)? open,
    TResult? Function(PersistError_Set value)? set_,
    TResult? Function(PersistError_Serialize value)? serialize,
  }) {
    return serialize?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PersistError_Open value)? open,
    TResult Function(PersistError_Set value)? set_,
    TResult Function(PersistError_Serialize value)? serialize,
    required TResult orElse(),
  }) {
    if (serialize != null) {
      return serialize(this);
    }
    return orElse();
  }
}

abstract class PersistError_Serialize extends PersistError {
  const factory PersistError_Serialize({required final String message}) =
      _$PersistError_SerializeImpl;
  const PersistError_Serialize._() : super._();

  @override
  String get message;

  /// Create a copy of PersistError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PersistError_SerializeImplCopyWith<_$PersistError_SerializeImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$RequestError {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) send,
    required TResult Function(String message) receive,
    required TResult Function(Duration duration) timeout,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? send,
    TResult? Function(String message)? receive,
    TResult? Function(Duration duration)? timeout,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? send,
    TResult Function(String message)? receive,
    TResult Function(Duration duration)? timeout,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RequestError_Send value) send,
    required TResult Function(RequestError_Receive value) receive,
    required TResult Function(RequestError_Timeout value) timeout,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RequestError_Send value)? send,
    TResult? Function(RequestError_Receive value)? receive,
    TResult? Function(RequestError_Timeout value)? timeout,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RequestError_Send value)? send,
    TResult Function(RequestError_Receive value)? receive,
    TResult Function(RequestError_Timeout value)? timeout,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $RequestErrorCopyWith<$Res> {
  factory $RequestErrorCopyWith(
          RequestError value, $Res Function(RequestError) then) =
      _$RequestErrorCopyWithImpl<$Res, RequestError>;
}

/// @nodoc
class _$RequestErrorCopyWithImpl<$Res, $Val extends RequestError>
    implements $RequestErrorCopyWith<$Res> {
  _$RequestErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$RequestError_SendImplCopyWith<$Res> {
  factory _$$RequestError_SendImplCopyWith(_$RequestError_SendImpl value,
          $Res Function(_$RequestError_SendImpl) then) =
      __$$RequestError_SendImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$RequestError_SendImplCopyWithImpl<$Res>
    extends _$RequestErrorCopyWithImpl<$Res, _$RequestError_SendImpl>
    implements _$$RequestError_SendImplCopyWith<$Res> {
  __$$RequestError_SendImplCopyWithImpl(_$RequestError_SendImpl _value,
      $Res Function(_$RequestError_SendImpl) _then)
      : super(_value, _then);

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$RequestError_SendImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$RequestError_SendImpl extends RequestError_Send {
  const _$RequestError_SendImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RequestError_SendImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$RequestError_SendImplCopyWith<_$RequestError_SendImpl> get copyWith =>
      __$$RequestError_SendImplCopyWithImpl<_$RequestError_SendImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) send,
    required TResult Function(String message) receive,
    required TResult Function(Duration duration) timeout,
  }) {
    return send(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? send,
    TResult? Function(String message)? receive,
    TResult? Function(Duration duration)? timeout,
  }) {
    return send?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? send,
    TResult Function(String message)? receive,
    TResult Function(Duration duration)? timeout,
    required TResult orElse(),
  }) {
    if (send != null) {
      return send(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RequestError_Send value) send,
    required TResult Function(RequestError_Receive value) receive,
    required TResult Function(RequestError_Timeout value) timeout,
  }) {
    return send(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RequestError_Send value)? send,
    TResult? Function(RequestError_Receive value)? receive,
    TResult? Function(RequestError_Timeout value)? timeout,
  }) {
    return send?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RequestError_Send value)? send,
    TResult Function(RequestError_Receive value)? receive,
    TResult Function(RequestError_Timeout value)? timeout,
    required TResult orElse(),
  }) {
    if (send != null) {
      return send(this);
    }
    return orElse();
  }
}

abstract class RequestError_Send extends RequestError {
  const factory RequestError_Send({required final String message}) =
      _$RequestError_SendImpl;
  const RequestError_Send._() : super._();

  String get message;

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$RequestError_SendImplCopyWith<_$RequestError_SendImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RequestError_ReceiveImplCopyWith<$Res> {
  factory _$$RequestError_ReceiveImplCopyWith(_$RequestError_ReceiveImpl value,
          $Res Function(_$RequestError_ReceiveImpl) then) =
      __$$RequestError_ReceiveImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$RequestError_ReceiveImplCopyWithImpl<$Res>
    extends _$RequestErrorCopyWithImpl<$Res, _$RequestError_ReceiveImpl>
    implements _$$RequestError_ReceiveImplCopyWith<$Res> {
  __$$RequestError_ReceiveImplCopyWithImpl(_$RequestError_ReceiveImpl _value,
      $Res Function(_$RequestError_ReceiveImpl) _then)
      : super(_value, _then);

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$RequestError_ReceiveImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$RequestError_ReceiveImpl extends RequestError_Receive {
  const _$RequestError_ReceiveImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RequestError_ReceiveImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$RequestError_ReceiveImplCopyWith<_$RequestError_ReceiveImpl>
      get copyWith =>
          __$$RequestError_ReceiveImplCopyWithImpl<_$RequestError_ReceiveImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) send,
    required TResult Function(String message) receive,
    required TResult Function(Duration duration) timeout,
  }) {
    return receive(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? send,
    TResult? Function(String message)? receive,
    TResult? Function(Duration duration)? timeout,
  }) {
    return receive?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? send,
    TResult Function(String message)? receive,
    TResult Function(Duration duration)? timeout,
    required TResult orElse(),
  }) {
    if (receive != null) {
      return receive(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RequestError_Send value) send,
    required TResult Function(RequestError_Receive value) receive,
    required TResult Function(RequestError_Timeout value) timeout,
  }) {
    return receive(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RequestError_Send value)? send,
    TResult? Function(RequestError_Receive value)? receive,
    TResult? Function(RequestError_Timeout value)? timeout,
  }) {
    return receive?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RequestError_Send value)? send,
    TResult Function(RequestError_Receive value)? receive,
    TResult Function(RequestError_Timeout value)? timeout,
    required TResult orElse(),
  }) {
    if (receive != null) {
      return receive(this);
    }
    return orElse();
  }
}

abstract class RequestError_Receive extends RequestError {
  const factory RequestError_Receive({required final String message}) =
      _$RequestError_ReceiveImpl;
  const RequestError_Receive._() : super._();

  String get message;

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$RequestError_ReceiveImplCopyWith<_$RequestError_ReceiveImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$RequestError_TimeoutImplCopyWith<$Res> {
  factory _$$RequestError_TimeoutImplCopyWith(_$RequestError_TimeoutImpl value,
          $Res Function(_$RequestError_TimeoutImpl) then) =
      __$$RequestError_TimeoutImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Duration duration});
}

/// @nodoc
class __$$RequestError_TimeoutImplCopyWithImpl<$Res>
    extends _$RequestErrorCopyWithImpl<$Res, _$RequestError_TimeoutImpl>
    implements _$$RequestError_TimeoutImplCopyWith<$Res> {
  __$$RequestError_TimeoutImplCopyWithImpl(_$RequestError_TimeoutImpl _value,
      $Res Function(_$RequestError_TimeoutImpl) _then)
      : super(_value, _then);

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? duration = null,
  }) {
    return _then(_$RequestError_TimeoutImpl(
      duration: null == duration
          ? _value.duration
          : duration // ignore: cast_nullable_to_non_nullable
              as Duration,
    ));
  }
}

/// @nodoc

class _$RequestError_TimeoutImpl extends RequestError_Timeout {
  const _$RequestError_TimeoutImpl({required this.duration}) : super._();

  @override
  final Duration duration;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$RequestError_TimeoutImpl &&
            (identical(other.duration, duration) ||
                other.duration == duration));
  }

  @override
  int get hashCode => Object.hash(runtimeType, duration);

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$RequestError_TimeoutImplCopyWith<_$RequestError_TimeoutImpl>
      get copyWith =>
          __$$RequestError_TimeoutImplCopyWithImpl<_$RequestError_TimeoutImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) send,
    required TResult Function(String message) receive,
    required TResult Function(Duration duration) timeout,
  }) {
    return timeout(duration);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? send,
    TResult? Function(String message)? receive,
    TResult? Function(Duration duration)? timeout,
  }) {
    return timeout?.call(duration);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? send,
    TResult Function(String message)? receive,
    TResult Function(Duration duration)? timeout,
    required TResult orElse(),
  }) {
    if (timeout != null) {
      return timeout(duration);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(RequestError_Send value) send,
    required TResult Function(RequestError_Receive value) receive,
    required TResult Function(RequestError_Timeout value) timeout,
  }) {
    return timeout(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(RequestError_Send value)? send,
    TResult? Function(RequestError_Receive value)? receive,
    TResult? Function(RequestError_Timeout value)? timeout,
  }) {
    return timeout?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(RequestError_Send value)? send,
    TResult Function(RequestError_Receive value)? receive,
    TResult Function(RequestError_Timeout value)? timeout,
    required TResult orElse(),
  }) {
    if (timeout != null) {
      return timeout(this);
    }
    return orElse();
  }
}

abstract class RequestError_Timeout extends RequestError {
  const factory RequestError_Timeout({required final Duration duration}) =
      _$RequestError_TimeoutImpl;
  const RequestError_Timeout._() : super._();

  Duration get duration;

  /// Create a copy of RequestError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$RequestError_TimeoutImplCopyWith<_$RequestError_TimeoutImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SendError {
  String get message => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) pipeBroken,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? pipeBroken,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? pipeBroken,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SendError_PipeBroken value) pipeBroken,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SendError_PipeBroken value)? pipeBroken,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SendError_PipeBroken value)? pipeBroken,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Create a copy of SendError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $SendErrorCopyWith<SendError> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $SendErrorCopyWith<$Res> {
  factory $SendErrorCopyWith(SendError value, $Res Function(SendError) then) =
      _$SendErrorCopyWithImpl<$Res, SendError>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class _$SendErrorCopyWithImpl<$Res, $Val extends SendError>
    implements $SendErrorCopyWith<$Res> {
  _$SendErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of SendError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_value.copyWith(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$SendError_PipeBrokenImplCopyWith<$Res>
    implements $SendErrorCopyWith<$Res> {
  factory _$$SendError_PipeBrokenImplCopyWith(_$SendError_PipeBrokenImpl value,
          $Res Function(_$SendError_PipeBrokenImpl) then) =
      __$$SendError_PipeBrokenImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$SendError_PipeBrokenImplCopyWithImpl<$Res>
    extends _$SendErrorCopyWithImpl<$Res, _$SendError_PipeBrokenImpl>
    implements _$$SendError_PipeBrokenImplCopyWith<$Res> {
  __$$SendError_PipeBrokenImplCopyWithImpl(_$SendError_PipeBrokenImpl _value,
      $Res Function(_$SendError_PipeBrokenImpl) _then)
      : super(_value, _then);

  /// Create a copy of SendError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? message = null,
  }) {
    return _then(_$SendError_PipeBrokenImpl(
      message: null == message
          ? _value.message
          : message // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$SendError_PipeBrokenImpl extends SendError_PipeBroken {
  const _$SendError_PipeBrokenImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SendError_PipeBrokenImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of SendError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$SendError_PipeBrokenImplCopyWith<_$SendError_PipeBrokenImpl>
      get copyWith =>
          __$$SendError_PipeBrokenImplCopyWithImpl<_$SendError_PipeBrokenImpl>(
              this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String message) pipeBroken,
  }) {
    return pipeBroken(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String message)? pipeBroken,
  }) {
    return pipeBroken?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String message)? pipeBroken,
    required TResult orElse(),
  }) {
    if (pipeBroken != null) {
      return pipeBroken(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(SendError_PipeBroken value) pipeBroken,
  }) {
    return pipeBroken(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(SendError_PipeBroken value)? pipeBroken,
  }) {
    return pipeBroken?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(SendError_PipeBroken value)? pipeBroken,
    required TResult orElse(),
  }) {
    if (pipeBroken != null) {
      return pipeBroken(this);
    }
    return orElse();
  }
}

abstract class SendError_PipeBroken extends SendError {
  const factory SendError_PipeBroken({required final String message}) =
      _$SendError_PipeBrokenImpl;
  const SendError_PipeBroken._() : super._();

  @override
  String get message;

  /// Create a copy of SendError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$SendError_PipeBrokenImplCopyWith<_$SendError_PipeBrokenImpl>
      get copyWith => throw _privateConstructorUsedError;
}
