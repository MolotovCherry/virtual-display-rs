// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'driver_client.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$AddModeError {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int id) monitorNotFound,
    required TResult Function(int monitorId, int width, int height) modeExists,
    required TResult Function(
            int monitorId, int width, int height, int refreshRate)
        refreshRateExists,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int id)? monitorNotFound,
    TResult? Function(int monitorId, int width, int height)? modeExists,
    TResult? Function(int monitorId, int width, int height, int refreshRate)?
        refreshRateExists,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int id)? monitorNotFound,
    TResult Function(int monitorId, int width, int height)? modeExists,
    TResult Function(int monitorId, int width, int height, int refreshRate)?
        refreshRateExists,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AddModeError_MonitorNotFound value)
        monitorNotFound,
    required TResult Function(AddModeError_ModeExists value) modeExists,
    required TResult Function(AddModeError_RefreshRateExists value)
        refreshRateExists,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AddModeError_MonitorNotFound value)? monitorNotFound,
    TResult? Function(AddModeError_ModeExists value)? modeExists,
    TResult? Function(AddModeError_RefreshRateExists value)? refreshRateExists,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AddModeError_MonitorNotFound value)? monitorNotFound,
    TResult Function(AddModeError_ModeExists value)? modeExists,
    TResult Function(AddModeError_RefreshRateExists value)? refreshRateExists,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AddModeErrorCopyWith<$Res> {
  factory $AddModeErrorCopyWith(
          AddModeError value, $Res Function(AddModeError) then) =
      _$AddModeErrorCopyWithImpl<$Res, AddModeError>;
}

/// @nodoc
class _$AddModeErrorCopyWithImpl<$Res, $Val extends AddModeError>
    implements $AddModeErrorCopyWith<$Res> {
  _$AddModeErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$AddModeError_MonitorNotFoundImplCopyWith<$Res> {
  factory _$$AddModeError_MonitorNotFoundImplCopyWith(
          _$AddModeError_MonitorNotFoundImpl value,
          $Res Function(_$AddModeError_MonitorNotFoundImpl) then) =
      __$$AddModeError_MonitorNotFoundImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int id});
}

/// @nodoc
class __$$AddModeError_MonitorNotFoundImplCopyWithImpl<$Res>
    extends _$AddModeErrorCopyWithImpl<$Res, _$AddModeError_MonitorNotFoundImpl>
    implements _$$AddModeError_MonitorNotFoundImplCopyWith<$Res> {
  __$$AddModeError_MonitorNotFoundImplCopyWithImpl(
      _$AddModeError_MonitorNotFoundImpl _value,
      $Res Function(_$AddModeError_MonitorNotFoundImpl) _then)
      : super(_value, _then);

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
  }) {
    return _then(_$AddModeError_MonitorNotFoundImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$AddModeError_MonitorNotFoundImpl extends AddModeError_MonitorNotFound {
  const _$AddModeError_MonitorNotFoundImpl({required this.id}) : super._();

  @override
  final int id;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AddModeError_MonitorNotFoundImpl &&
            (identical(other.id, id) || other.id == id));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id);

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AddModeError_MonitorNotFoundImplCopyWith<
          _$AddModeError_MonitorNotFoundImpl>
      get copyWith => __$$AddModeError_MonitorNotFoundImplCopyWithImpl<
          _$AddModeError_MonitorNotFoundImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int id) monitorNotFound,
    required TResult Function(int monitorId, int width, int height) modeExists,
    required TResult Function(
            int monitorId, int width, int height, int refreshRate)
        refreshRateExists,
  }) {
    return monitorNotFound(id);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int id)? monitorNotFound,
    TResult? Function(int monitorId, int width, int height)? modeExists,
    TResult? Function(int monitorId, int width, int height, int refreshRate)?
        refreshRateExists,
  }) {
    return monitorNotFound?.call(id);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int id)? monitorNotFound,
    TResult Function(int monitorId, int width, int height)? modeExists,
    TResult Function(int monitorId, int width, int height, int refreshRate)?
        refreshRateExists,
    required TResult orElse(),
  }) {
    if (monitorNotFound != null) {
      return monitorNotFound(id);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AddModeError_MonitorNotFound value)
        monitorNotFound,
    required TResult Function(AddModeError_ModeExists value) modeExists,
    required TResult Function(AddModeError_RefreshRateExists value)
        refreshRateExists,
  }) {
    return monitorNotFound(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AddModeError_MonitorNotFound value)? monitorNotFound,
    TResult? Function(AddModeError_ModeExists value)? modeExists,
    TResult? Function(AddModeError_RefreshRateExists value)? refreshRateExists,
  }) {
    return monitorNotFound?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AddModeError_MonitorNotFound value)? monitorNotFound,
    TResult Function(AddModeError_ModeExists value)? modeExists,
    TResult Function(AddModeError_RefreshRateExists value)? refreshRateExists,
    required TResult orElse(),
  }) {
    if (monitorNotFound != null) {
      return monitorNotFound(this);
    }
    return orElse();
  }
}

abstract class AddModeError_MonitorNotFound extends AddModeError {
  const factory AddModeError_MonitorNotFound({required final int id}) =
      _$AddModeError_MonitorNotFoundImpl;
  const AddModeError_MonitorNotFound._() : super._();

  int get id;

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AddModeError_MonitorNotFoundImplCopyWith<
          _$AddModeError_MonitorNotFoundImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AddModeError_ModeExistsImplCopyWith<$Res> {
  factory _$$AddModeError_ModeExistsImplCopyWith(
          _$AddModeError_ModeExistsImpl value,
          $Res Function(_$AddModeError_ModeExistsImpl) then) =
      __$$AddModeError_ModeExistsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int monitorId, int width, int height});
}

/// @nodoc
class __$$AddModeError_ModeExistsImplCopyWithImpl<$Res>
    extends _$AddModeErrorCopyWithImpl<$Res, _$AddModeError_ModeExistsImpl>
    implements _$$AddModeError_ModeExistsImplCopyWith<$Res> {
  __$$AddModeError_ModeExistsImplCopyWithImpl(
      _$AddModeError_ModeExistsImpl _value,
      $Res Function(_$AddModeError_ModeExistsImpl) _then)
      : super(_value, _then);

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? monitorId = null,
    Object? width = null,
    Object? height = null,
  }) {
    return _then(_$AddModeError_ModeExistsImpl(
      monitorId: null == monitorId
          ? _value.monitorId
          : monitorId // ignore: cast_nullable_to_non_nullable
              as int,
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$AddModeError_ModeExistsImpl extends AddModeError_ModeExists {
  const _$AddModeError_ModeExistsImpl(
      {required this.monitorId, required this.width, required this.height})
      : super._();

  @override
  final int monitorId;
  @override
  final int width;
  @override
  final int height;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AddModeError_ModeExistsImpl &&
            (identical(other.monitorId, monitorId) ||
                other.monitorId == monitorId) &&
            (identical(other.width, width) || other.width == width) &&
            (identical(other.height, height) || other.height == height));
  }

  @override
  int get hashCode => Object.hash(runtimeType, monitorId, width, height);

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AddModeError_ModeExistsImplCopyWith<_$AddModeError_ModeExistsImpl>
      get copyWith => __$$AddModeError_ModeExistsImplCopyWithImpl<
          _$AddModeError_ModeExistsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int id) monitorNotFound,
    required TResult Function(int monitorId, int width, int height) modeExists,
    required TResult Function(
            int monitorId, int width, int height, int refreshRate)
        refreshRateExists,
  }) {
    return modeExists(monitorId, width, height);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int id)? monitorNotFound,
    TResult? Function(int monitorId, int width, int height)? modeExists,
    TResult? Function(int monitorId, int width, int height, int refreshRate)?
        refreshRateExists,
  }) {
    return modeExists?.call(monitorId, width, height);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int id)? monitorNotFound,
    TResult Function(int monitorId, int width, int height)? modeExists,
    TResult Function(int monitorId, int width, int height, int refreshRate)?
        refreshRateExists,
    required TResult orElse(),
  }) {
    if (modeExists != null) {
      return modeExists(monitorId, width, height);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AddModeError_MonitorNotFound value)
        monitorNotFound,
    required TResult Function(AddModeError_ModeExists value) modeExists,
    required TResult Function(AddModeError_RefreshRateExists value)
        refreshRateExists,
  }) {
    return modeExists(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AddModeError_MonitorNotFound value)? monitorNotFound,
    TResult? Function(AddModeError_ModeExists value)? modeExists,
    TResult? Function(AddModeError_RefreshRateExists value)? refreshRateExists,
  }) {
    return modeExists?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AddModeError_MonitorNotFound value)? monitorNotFound,
    TResult Function(AddModeError_ModeExists value)? modeExists,
    TResult Function(AddModeError_RefreshRateExists value)? refreshRateExists,
    required TResult orElse(),
  }) {
    if (modeExists != null) {
      return modeExists(this);
    }
    return orElse();
  }
}

abstract class AddModeError_ModeExists extends AddModeError {
  const factory AddModeError_ModeExists(
      {required final int monitorId,
      required final int width,
      required final int height}) = _$AddModeError_ModeExistsImpl;
  const AddModeError_ModeExists._() : super._();

  int get monitorId;
  int get width;
  int get height;

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AddModeError_ModeExistsImplCopyWith<_$AddModeError_ModeExistsImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AddModeError_RefreshRateExistsImplCopyWith<$Res> {
  factory _$$AddModeError_RefreshRateExistsImplCopyWith(
          _$AddModeError_RefreshRateExistsImpl value,
          $Res Function(_$AddModeError_RefreshRateExistsImpl) then) =
      __$$AddModeError_RefreshRateExistsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int monitorId, int width, int height, int refreshRate});
}

/// @nodoc
class __$$AddModeError_RefreshRateExistsImplCopyWithImpl<$Res>
    extends _$AddModeErrorCopyWithImpl<$Res,
        _$AddModeError_RefreshRateExistsImpl>
    implements _$$AddModeError_RefreshRateExistsImplCopyWith<$Res> {
  __$$AddModeError_RefreshRateExistsImplCopyWithImpl(
      _$AddModeError_RefreshRateExistsImpl _value,
      $Res Function(_$AddModeError_RefreshRateExistsImpl) _then)
      : super(_value, _then);

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? monitorId = null,
    Object? width = null,
    Object? height = null,
    Object? refreshRate = null,
  }) {
    return _then(_$AddModeError_RefreshRateExistsImpl(
      monitorId: null == monitorId
          ? _value.monitorId
          : monitorId // ignore: cast_nullable_to_non_nullable
              as int,
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
      refreshRate: null == refreshRate
          ? _value.refreshRate
          : refreshRate // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$AddModeError_RefreshRateExistsImpl
    extends AddModeError_RefreshRateExists {
  const _$AddModeError_RefreshRateExistsImpl(
      {required this.monitorId,
      required this.width,
      required this.height,
      required this.refreshRate})
      : super._();

  @override
  final int monitorId;
  @override
  final int width;
  @override
  final int height;
  @override
  final int refreshRate;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AddModeError_RefreshRateExistsImpl &&
            (identical(other.monitorId, monitorId) ||
                other.monitorId == monitorId) &&
            (identical(other.width, width) || other.width == width) &&
            (identical(other.height, height) || other.height == height) &&
            (identical(other.refreshRate, refreshRate) ||
                other.refreshRate == refreshRate));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, monitorId, width, height, refreshRate);

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AddModeError_RefreshRateExistsImplCopyWith<
          _$AddModeError_RefreshRateExistsImpl>
      get copyWith => __$$AddModeError_RefreshRateExistsImplCopyWithImpl<
          _$AddModeError_RefreshRateExistsImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int id) monitorNotFound,
    required TResult Function(int monitorId, int width, int height) modeExists,
    required TResult Function(
            int monitorId, int width, int height, int refreshRate)
        refreshRateExists,
  }) {
    return refreshRateExists(monitorId, width, height, refreshRate);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int id)? monitorNotFound,
    TResult? Function(int monitorId, int width, int height)? modeExists,
    TResult? Function(int monitorId, int width, int height, int refreshRate)?
        refreshRateExists,
  }) {
    return refreshRateExists?.call(monitorId, width, height, refreshRate);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int id)? monitorNotFound,
    TResult Function(int monitorId, int width, int height)? modeExists,
    TResult Function(int monitorId, int width, int height, int refreshRate)?
        refreshRateExists,
    required TResult orElse(),
  }) {
    if (refreshRateExists != null) {
      return refreshRateExists(monitorId, width, height, refreshRate);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AddModeError_MonitorNotFound value)
        monitorNotFound,
    required TResult Function(AddModeError_ModeExists value) modeExists,
    required TResult Function(AddModeError_RefreshRateExists value)
        refreshRateExists,
  }) {
    return refreshRateExists(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AddModeError_MonitorNotFound value)? monitorNotFound,
    TResult? Function(AddModeError_ModeExists value)? modeExists,
    TResult? Function(AddModeError_RefreshRateExists value)? refreshRateExists,
  }) {
    return refreshRateExists?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AddModeError_MonitorNotFound value)? monitorNotFound,
    TResult Function(AddModeError_ModeExists value)? modeExists,
    TResult Function(AddModeError_RefreshRateExists value)? refreshRateExists,
    required TResult orElse(),
  }) {
    if (refreshRateExists != null) {
      return refreshRateExists(this);
    }
    return orElse();
  }
}

abstract class AddModeError_RefreshRateExists extends AddModeError {
  const factory AddModeError_RefreshRateExists(
      {required final int monitorId,
      required final int width,
      required final int height,
      required final int refreshRate}) = _$AddModeError_RefreshRateExistsImpl;
  const AddModeError_RefreshRateExists._() : super._();

  int get monitorId;
  int get width;
  int get height;
  int get refreshRate;

  /// Create a copy of AddModeError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AddModeError_RefreshRateExistsImplCopyWith<
          _$AddModeError_RefreshRateExistsImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$DuplicateError {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int id) monitor,
    required TResult Function(int monitorId, int width, int height) mode,
    required TResult Function(
            int monitorId, int width, int height, int refreshRate)
        refreshRate,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int id)? monitor,
    TResult? Function(int monitorId, int width, int height)? mode,
    TResult? Function(int monitorId, int width, int height, int refreshRate)?
        refreshRate,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int id)? monitor,
    TResult Function(int monitorId, int width, int height)? mode,
    TResult Function(int monitorId, int width, int height, int refreshRate)?
        refreshRate,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DuplicateError_Monitor value) monitor,
    required TResult Function(DuplicateError_Mode value) mode,
    required TResult Function(DuplicateError_RefreshRate value) refreshRate,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DuplicateError_Monitor value)? monitor,
    TResult? Function(DuplicateError_Mode value)? mode,
    TResult? Function(DuplicateError_RefreshRate value)? refreshRate,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DuplicateError_Monitor value)? monitor,
    TResult Function(DuplicateError_Mode value)? mode,
    TResult Function(DuplicateError_RefreshRate value)? refreshRate,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DuplicateErrorCopyWith<$Res> {
  factory $DuplicateErrorCopyWith(
          DuplicateError value, $Res Function(DuplicateError) then) =
      _$DuplicateErrorCopyWithImpl<$Res, DuplicateError>;
}

/// @nodoc
class _$DuplicateErrorCopyWithImpl<$Res, $Val extends DuplicateError>
    implements $DuplicateErrorCopyWith<$Res> {
  _$DuplicateErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$DuplicateError_MonitorImplCopyWith<$Res> {
  factory _$$DuplicateError_MonitorImplCopyWith(
          _$DuplicateError_MonitorImpl value,
          $Res Function(_$DuplicateError_MonitorImpl) then) =
      __$$DuplicateError_MonitorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int id});
}

/// @nodoc
class __$$DuplicateError_MonitorImplCopyWithImpl<$Res>
    extends _$DuplicateErrorCopyWithImpl<$Res, _$DuplicateError_MonitorImpl>
    implements _$$DuplicateError_MonitorImplCopyWith<$Res> {
  __$$DuplicateError_MonitorImplCopyWithImpl(
      _$DuplicateError_MonitorImpl _value,
      $Res Function(_$DuplicateError_MonitorImpl) _then)
      : super(_value, _then);

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
  }) {
    return _then(_$DuplicateError_MonitorImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$DuplicateError_MonitorImpl extends DuplicateError_Monitor {
  const _$DuplicateError_MonitorImpl({required this.id}) : super._();

  @override
  final int id;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DuplicateError_MonitorImpl &&
            (identical(other.id, id) || other.id == id));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id);

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DuplicateError_MonitorImplCopyWith<_$DuplicateError_MonitorImpl>
      get copyWith => __$$DuplicateError_MonitorImplCopyWithImpl<
          _$DuplicateError_MonitorImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int id) monitor,
    required TResult Function(int monitorId, int width, int height) mode,
    required TResult Function(
            int monitorId, int width, int height, int refreshRate)
        refreshRate,
  }) {
    return monitor(id);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int id)? monitor,
    TResult? Function(int monitorId, int width, int height)? mode,
    TResult? Function(int monitorId, int width, int height, int refreshRate)?
        refreshRate,
  }) {
    return monitor?.call(id);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int id)? monitor,
    TResult Function(int monitorId, int width, int height)? mode,
    TResult Function(int monitorId, int width, int height, int refreshRate)?
        refreshRate,
    required TResult orElse(),
  }) {
    if (monitor != null) {
      return monitor(id);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DuplicateError_Monitor value) monitor,
    required TResult Function(DuplicateError_Mode value) mode,
    required TResult Function(DuplicateError_RefreshRate value) refreshRate,
  }) {
    return monitor(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DuplicateError_Monitor value)? monitor,
    TResult? Function(DuplicateError_Mode value)? mode,
    TResult? Function(DuplicateError_RefreshRate value)? refreshRate,
  }) {
    return monitor?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DuplicateError_Monitor value)? monitor,
    TResult Function(DuplicateError_Mode value)? mode,
    TResult Function(DuplicateError_RefreshRate value)? refreshRate,
    required TResult orElse(),
  }) {
    if (monitor != null) {
      return monitor(this);
    }
    return orElse();
  }
}

abstract class DuplicateError_Monitor extends DuplicateError {
  const factory DuplicateError_Monitor({required final int id}) =
      _$DuplicateError_MonitorImpl;
  const DuplicateError_Monitor._() : super._();

  int get id;

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DuplicateError_MonitorImplCopyWith<_$DuplicateError_MonitorImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DuplicateError_ModeImplCopyWith<$Res> {
  factory _$$DuplicateError_ModeImplCopyWith(_$DuplicateError_ModeImpl value,
          $Res Function(_$DuplicateError_ModeImpl) then) =
      __$$DuplicateError_ModeImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int monitorId, int width, int height});
}

/// @nodoc
class __$$DuplicateError_ModeImplCopyWithImpl<$Res>
    extends _$DuplicateErrorCopyWithImpl<$Res, _$DuplicateError_ModeImpl>
    implements _$$DuplicateError_ModeImplCopyWith<$Res> {
  __$$DuplicateError_ModeImplCopyWithImpl(_$DuplicateError_ModeImpl _value,
      $Res Function(_$DuplicateError_ModeImpl) _then)
      : super(_value, _then);

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? monitorId = null,
    Object? width = null,
    Object? height = null,
  }) {
    return _then(_$DuplicateError_ModeImpl(
      monitorId: null == monitorId
          ? _value.monitorId
          : monitorId // ignore: cast_nullable_to_non_nullable
              as int,
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$DuplicateError_ModeImpl extends DuplicateError_Mode {
  const _$DuplicateError_ModeImpl(
      {required this.monitorId, required this.width, required this.height})
      : super._();

  @override
  final int monitorId;
  @override
  final int width;
  @override
  final int height;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DuplicateError_ModeImpl &&
            (identical(other.monitorId, monitorId) ||
                other.monitorId == monitorId) &&
            (identical(other.width, width) || other.width == width) &&
            (identical(other.height, height) || other.height == height));
  }

  @override
  int get hashCode => Object.hash(runtimeType, monitorId, width, height);

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DuplicateError_ModeImplCopyWith<_$DuplicateError_ModeImpl> get copyWith =>
      __$$DuplicateError_ModeImplCopyWithImpl<_$DuplicateError_ModeImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int id) monitor,
    required TResult Function(int monitorId, int width, int height) mode,
    required TResult Function(
            int monitorId, int width, int height, int refreshRate)
        refreshRate,
  }) {
    return mode(monitorId, width, height);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int id)? monitor,
    TResult? Function(int monitorId, int width, int height)? mode,
    TResult? Function(int monitorId, int width, int height, int refreshRate)?
        refreshRate,
  }) {
    return mode?.call(monitorId, width, height);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int id)? monitor,
    TResult Function(int monitorId, int width, int height)? mode,
    TResult Function(int monitorId, int width, int height, int refreshRate)?
        refreshRate,
    required TResult orElse(),
  }) {
    if (mode != null) {
      return mode(monitorId, width, height);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DuplicateError_Monitor value) monitor,
    required TResult Function(DuplicateError_Mode value) mode,
    required TResult Function(DuplicateError_RefreshRate value) refreshRate,
  }) {
    return mode(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DuplicateError_Monitor value)? monitor,
    TResult? Function(DuplicateError_Mode value)? mode,
    TResult? Function(DuplicateError_RefreshRate value)? refreshRate,
  }) {
    return mode?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DuplicateError_Monitor value)? monitor,
    TResult Function(DuplicateError_Mode value)? mode,
    TResult Function(DuplicateError_RefreshRate value)? refreshRate,
    required TResult orElse(),
  }) {
    if (mode != null) {
      return mode(this);
    }
    return orElse();
  }
}

abstract class DuplicateError_Mode extends DuplicateError {
  const factory DuplicateError_Mode(
      {required final int monitorId,
      required final int width,
      required final int height}) = _$DuplicateError_ModeImpl;
  const DuplicateError_Mode._() : super._();

  int get monitorId;
  int get width;
  int get height;

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DuplicateError_ModeImplCopyWith<_$DuplicateError_ModeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DuplicateError_RefreshRateImplCopyWith<$Res> {
  factory _$$DuplicateError_RefreshRateImplCopyWith(
          _$DuplicateError_RefreshRateImpl value,
          $Res Function(_$DuplicateError_RefreshRateImpl) then) =
      __$$DuplicateError_RefreshRateImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int monitorId, int width, int height, int refreshRate});
}

/// @nodoc
class __$$DuplicateError_RefreshRateImplCopyWithImpl<$Res>
    extends _$DuplicateErrorCopyWithImpl<$Res, _$DuplicateError_RefreshRateImpl>
    implements _$$DuplicateError_RefreshRateImplCopyWith<$Res> {
  __$$DuplicateError_RefreshRateImplCopyWithImpl(
      _$DuplicateError_RefreshRateImpl _value,
      $Res Function(_$DuplicateError_RefreshRateImpl) _then)
      : super(_value, _then);

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? monitorId = null,
    Object? width = null,
    Object? height = null,
    Object? refreshRate = null,
  }) {
    return _then(_$DuplicateError_RefreshRateImpl(
      monitorId: null == monitorId
          ? _value.monitorId
          : monitorId // ignore: cast_nullable_to_non_nullable
              as int,
      width: null == width
          ? _value.width
          : width // ignore: cast_nullable_to_non_nullable
              as int,
      height: null == height
          ? _value.height
          : height // ignore: cast_nullable_to_non_nullable
              as int,
      refreshRate: null == refreshRate
          ? _value.refreshRate
          : refreshRate // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$DuplicateError_RefreshRateImpl extends DuplicateError_RefreshRate {
  const _$DuplicateError_RefreshRateImpl(
      {required this.monitorId,
      required this.width,
      required this.height,
      required this.refreshRate})
      : super._();

  @override
  final int monitorId;
  @override
  final int width;
  @override
  final int height;
  @override
  final int refreshRate;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DuplicateError_RefreshRateImpl &&
            (identical(other.monitorId, monitorId) ||
                other.monitorId == monitorId) &&
            (identical(other.width, width) || other.width == width) &&
            (identical(other.height, height) || other.height == height) &&
            (identical(other.refreshRate, refreshRate) ||
                other.refreshRate == refreshRate));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, monitorId, width, height, refreshRate);

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DuplicateError_RefreshRateImplCopyWith<_$DuplicateError_RefreshRateImpl>
      get copyWith => __$$DuplicateError_RefreshRateImplCopyWithImpl<
          _$DuplicateError_RefreshRateImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int id) monitor,
    required TResult Function(int monitorId, int width, int height) mode,
    required TResult Function(
            int monitorId, int width, int height, int refreshRate)
        refreshRate,
  }) {
    return refreshRate(monitorId, width, height, this.refreshRate);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int id)? monitor,
    TResult? Function(int monitorId, int width, int height)? mode,
    TResult? Function(int monitorId, int width, int height, int refreshRate)?
        refreshRate,
  }) {
    return refreshRate?.call(monitorId, width, height, this.refreshRate);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int id)? monitor,
    TResult Function(int monitorId, int width, int height)? mode,
    TResult Function(int monitorId, int width, int height, int refreshRate)?
        refreshRate,
    required TResult orElse(),
  }) {
    if (refreshRate != null) {
      return refreshRate(monitorId, width, height, this.refreshRate);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(DuplicateError_Monitor value) monitor,
    required TResult Function(DuplicateError_Mode value) mode,
    required TResult Function(DuplicateError_RefreshRate value) refreshRate,
  }) {
    return refreshRate(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(DuplicateError_Monitor value)? monitor,
    TResult? Function(DuplicateError_Mode value)? mode,
    TResult? Function(DuplicateError_RefreshRate value)? refreshRate,
  }) {
    return refreshRate?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(DuplicateError_Monitor value)? monitor,
    TResult Function(DuplicateError_Mode value)? mode,
    TResult Function(DuplicateError_RefreshRate value)? refreshRate,
    required TResult orElse(),
  }) {
    if (refreshRate != null) {
      return refreshRate(this);
    }
    return orElse();
  }
}

abstract class DuplicateError_RefreshRate extends DuplicateError {
  const factory DuplicateError_RefreshRate(
      {required final int monitorId,
      required final int width,
      required final int height,
      required final int refreshRate}) = _$DuplicateError_RefreshRateImpl;
  const DuplicateError_RefreshRate._() : super._();

  int get monitorId;
  int get width;
  int get height;
  int get refreshRate;

  /// Create a copy of DuplicateError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DuplicateError_RefreshRateImplCopyWith<_$DuplicateError_RefreshRateImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$InitError {
  FrbException get inner => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ConnectionError inner) connect,
    required TResult Function(RequestError inner) requestState,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError inner)? connect,
    TResult? Function(RequestError inner)? requestState,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ConnectionError inner)? connect,
    TResult Function(RequestError inner)? requestState,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InitError_Connect value) connect,
    required TResult Function(InitError_RequestState value) requestState,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InitError_Connect value)? connect,
    TResult? Function(InitError_RequestState value)? requestState,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InitError_Connect value)? connect,
    TResult Function(InitError_RequestState value)? requestState,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $InitErrorCopyWith<$Res> {
  factory $InitErrorCopyWith(InitError value, $Res Function(InitError) then) =
      _$InitErrorCopyWithImpl<$Res, InitError>;
}

/// @nodoc
class _$InitErrorCopyWithImpl<$Res, $Val extends InitError>
    implements $InitErrorCopyWith<$Res> {
  _$InitErrorCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$InitError_ConnectImplCopyWith<$Res> {
  factory _$$InitError_ConnectImplCopyWith(_$InitError_ConnectImpl value,
          $Res Function(_$InitError_ConnectImpl) then) =
      __$$InitError_ConnectImplCopyWithImpl<$Res>;
  @useResult
  $Res call({ConnectionError inner});

  $ConnectionErrorCopyWith<$Res> get inner;
}

/// @nodoc
class __$$InitError_ConnectImplCopyWithImpl<$Res>
    extends _$InitErrorCopyWithImpl<$Res, _$InitError_ConnectImpl>
    implements _$$InitError_ConnectImplCopyWith<$Res> {
  __$$InitError_ConnectImplCopyWithImpl(_$InitError_ConnectImpl _value,
      $Res Function(_$InitError_ConnectImpl) _then)
      : super(_value, _then);

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? inner = null,
  }) {
    return _then(_$InitError_ConnectImpl(
      inner: null == inner
          ? _value.inner
          : inner // ignore: cast_nullable_to_non_nullable
              as ConnectionError,
    ));
  }

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $ConnectionErrorCopyWith<$Res> get inner {
    return $ConnectionErrorCopyWith<$Res>(_value.inner, (value) {
      return _then(_value.copyWith(inner: value));
    });
  }
}

/// @nodoc

class _$InitError_ConnectImpl extends InitError_Connect {
  const _$InitError_ConnectImpl({required this.inner}) : super._();

  @override
  final ConnectionError inner;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InitError_ConnectImpl &&
            (identical(other.inner, inner) || other.inner == inner));
  }

  @override
  int get hashCode => Object.hash(runtimeType, inner);

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$InitError_ConnectImplCopyWith<_$InitError_ConnectImpl> get copyWith =>
      __$$InitError_ConnectImplCopyWithImpl<_$InitError_ConnectImpl>(
          this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ConnectionError inner) connect,
    required TResult Function(RequestError inner) requestState,
  }) {
    return connect(inner);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError inner)? connect,
    TResult? Function(RequestError inner)? requestState,
  }) {
    return connect?.call(inner);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ConnectionError inner)? connect,
    TResult Function(RequestError inner)? requestState,
    required TResult orElse(),
  }) {
    if (connect != null) {
      return connect(inner);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InitError_Connect value) connect,
    required TResult Function(InitError_RequestState value) requestState,
  }) {
    return connect(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InitError_Connect value)? connect,
    TResult? Function(InitError_RequestState value)? requestState,
  }) {
    return connect?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InitError_Connect value)? connect,
    TResult Function(InitError_RequestState value)? requestState,
    required TResult orElse(),
  }) {
    if (connect != null) {
      return connect(this);
    }
    return orElse();
  }
}

abstract class InitError_Connect extends InitError {
  const factory InitError_Connect({required final ConnectionError inner}) =
      _$InitError_ConnectImpl;
  const InitError_Connect._() : super._();

  @override
  ConnectionError get inner;

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$InitError_ConnectImplCopyWith<_$InitError_ConnectImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InitError_RequestStateImplCopyWith<$Res> {
  factory _$$InitError_RequestStateImplCopyWith(
          _$InitError_RequestStateImpl value,
          $Res Function(_$InitError_RequestStateImpl) then) =
      __$$InitError_RequestStateImplCopyWithImpl<$Res>;
  @useResult
  $Res call({RequestError inner});

  $RequestErrorCopyWith<$Res> get inner;
}

/// @nodoc
class __$$InitError_RequestStateImplCopyWithImpl<$Res>
    extends _$InitErrorCopyWithImpl<$Res, _$InitError_RequestStateImpl>
    implements _$$InitError_RequestStateImplCopyWith<$Res> {
  __$$InitError_RequestStateImplCopyWithImpl(
      _$InitError_RequestStateImpl _value,
      $Res Function(_$InitError_RequestStateImpl) _then)
      : super(_value, _then);

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? inner = null,
  }) {
    return _then(_$InitError_RequestStateImpl(
      inner: null == inner
          ? _value.inner
          : inner // ignore: cast_nullable_to_non_nullable
              as RequestError,
    ));
  }

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $RequestErrorCopyWith<$Res> get inner {
    return $RequestErrorCopyWith<$Res>(_value.inner, (value) {
      return _then(_value.copyWith(inner: value));
    });
  }
}

/// @nodoc

class _$InitError_RequestStateImpl extends InitError_RequestState {
  const _$InitError_RequestStateImpl({required this.inner}) : super._();

  @override
  final RequestError inner;

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$InitError_RequestStateImpl &&
            (identical(other.inner, inner) || other.inner == inner));
  }

  @override
  int get hashCode => Object.hash(runtimeType, inner);

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$InitError_RequestStateImplCopyWith<_$InitError_RequestStateImpl>
      get copyWith => __$$InitError_RequestStateImplCopyWithImpl<
          _$InitError_RequestStateImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(ConnectionError inner) connect,
    required TResult Function(RequestError inner) requestState,
  }) {
    return requestState(inner);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(ConnectionError inner)? connect,
    TResult? Function(RequestError inner)? requestState,
  }) {
    return requestState?.call(inner);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(ConnectionError inner)? connect,
    TResult Function(RequestError inner)? requestState,
    required TResult orElse(),
  }) {
    if (requestState != null) {
      return requestState(inner);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(InitError_Connect value) connect,
    required TResult Function(InitError_RequestState value) requestState,
  }) {
    return requestState(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(InitError_Connect value)? connect,
    TResult? Function(InitError_RequestState value)? requestState,
  }) {
    return requestState?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InitError_Connect value)? connect,
    TResult Function(InitError_RequestState value)? requestState,
    required TResult orElse(),
  }) {
    if (requestState != null) {
      return requestState(this);
    }
    return orElse();
  }
}

abstract class InitError_RequestState extends InitError {
  const factory InitError_RequestState({required final RequestError inner}) =
      _$InitError_RequestStateImpl;
  const InitError_RequestState._() : super._();

  @override
  RequestError get inner;

  /// Create a copy of InitError
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$InitError_RequestStateImplCopyWith<_$InitError_RequestStateImpl>
      get copyWith => throw _privateConstructorUsedError;
}
