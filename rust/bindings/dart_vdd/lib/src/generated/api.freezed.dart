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
