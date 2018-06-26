#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
extern crate libc;
#[macro_use]
extern crate log;

unsafe fn YGResolveValue(value: *const YGValue, parentSize: libc::c_float) -> libc::c_float {
    match (*value).unit {
        YGUnitPoint => (*value).value,
        YGUnitPercent => (*value).value * parentSize / 100.0f32,
        _ => YGUndefined,
    }
}

// TODO(anp): figure out wtf this is
type _IO_FILE_plus = libc::c_void;

extern "C" {
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __isnan(__value: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn fmodf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn __isnanf(__value: libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn fmaxf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fminf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn __isnanl(__value: libc::c_double) -> libc::c_int;
    #[no_mangle]
    static mut signgam: libc::c_int;
    #[no_mangle]
    static mut _LIB_VERSION: _LIB_VERSION_TYPE;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub const _IEEE_: _LIB_VERSION_TYPE = -1;
pub const YGOverflowScroll: YGOverflow = 2;
pub type YGFree = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub const YGEdgeRight: YGEdge_0 = 2;
pub const YGEdgeVertical: YGEdge_0 = 7;
pub type YGWrap = libc::c_uint;
pub const YGDirectionInherit: YGDirection = 0;
pub const YGAlignSpaceAround: YGAlign = 7;
pub const YGFlexDirectionColumnReverse: YGFlexDirection = 1;
pub const YGJustifyCenter: YGJustify_0 = 1;
pub type YGPositionType = YGPositionType_0;
pub type YGEdge = YGEdge_0;
pub type va_list = __builtin_va_list;
pub const YGEdgeStart: YGEdge_0 = 4;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGNodeList {
    pub capacity: uint32_t,
    pub count: uint32_t,
    pub items: *mut YGNodeRef,
}
pub type YGOverflow = libc::c_uint;
pub type YGValue = YGValue_0;
pub type YGAlign = libc::c_uint;
pub const YGEdgeLeft: YGEdge_0 = 0;
pub type YGNodeListRef = *mut YGNodeList;
pub const YGEdgeAll: YGEdge_0 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGValue_0 {
    pub value: libc::c_float,
    pub unit: YGUnit_0,
}

impl PartialEq for YGValue {
    fn eq(&self, other: &Self) -> bool {
        if a.unit != b.unit {
            return false;
        }

        if a.unit == YGUnitUndefined {
            return true;
        }

        return fabs(a.value - b.value) < 0.0001;
    }
}

pub const YGMeasureModeExactly: YGMeasureMode_0 = 1;
pub const YGWrapNoWrap: YGWrap = 0;
pub type YGWrap_0 = YGWrap;
pub type YGOverflow_0 = YGOverflow;
pub const YGAlignStretch: YGAlign = 4;
pub type YGDirection = libc::c_uint;
pub const YGWrapWrap: YGWrap = 1;
pub const YGEdgeTop: YGEdge_0 = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGLayout {
    pub position: [libc::c_float; 4],
    pub dimensions: [libc::c_float; 2],
    pub margin: [libc::c_float; 6],
    pub border: [libc::c_float; 6],
    pub padding: [libc::c_float; 6],
    pub direction: YGDirection_0,
    pub computedFlexBasisGeneration: uint32_t,
    pub computedFlexBasis: libc::c_float,
    pub hadOverflow: bool,
    pub generationCount: uint32_t,
    pub lastParentDirection: YGDirection_0,
    pub nextCachedMeasurementsIndex: uint32_t,
    pub cachedMeasurements: [YGCachedMeasurement_0; 16],
    pub measuredDimensions: [libc::c_float; 2],
    pub cachedLayout: YGCachedMeasurement_0,
}
pub type YGRealloc =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void>;
pub const YGOverflowHidden: YGOverflow = 1;
pub const YGUnitPercent: YGUnit = 2;
pub type YGJustify = YGJustify_0;
pub const YGUnitPoint: YGUnit = 1;
pub const YGDisplayFlex: YGDisplay_0 = 0;
pub type YGLayout_0 = YGLayout;
pub type YGDisplay = YGDisplay_0;
pub type YGAlign_0 = YGAlign;
pub const YGFlexDirectionRowReverse: YGFlexDirection = 3;
pub type YGMalloc = Option<unsafe extern "C" fn(_: size_t) -> *mut libc::c_void>;
pub type FILE = _IO_FILE;
pub type YGFlexDirection = libc::c_uint;
pub type size_t = libc::c_ulong;
pub const YGAlignAuto: YGAlign = 0;
pub const YGNodeTypeText: YGNodeType = 1;
pub const YGDimensionHeight: YGDimension_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGStringStream {
    pub str_0: *mut libc::c_char,
    pub length: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub const YGMeasureModeAtMost: YGMeasureMode_0 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGStyle {
    pub direction: YGDirection_0,
    pub flexDirection: YGFlexDirection_0,
    pub justifyContent: YGJustify,
    pub alignContent: YGAlign_0,
    pub alignItems: YGAlign_0,
    pub alignSelf: YGAlign_0,
    pub positionType: YGPositionType,
    pub flexWrap: YGWrap_0,
    pub overflow: YGOverflow_0,
    pub display: YGDisplay,
    pub flex: libc::c_float,
    pub flexGrow: libc::c_float,
    pub flexShrink: libc::c_float,
    pub flexBasis: YGValue,
    pub margin: [YGValue; 9],
    pub position: [YGValue; 9],
    pub padding: [YGValue; 9],
    pub border: [YGValue; 9],
    pub dimensions: [YGValue; 2],
    pub minDimensions: [YGValue; 2],
    pub maxDimensions: [YGValue; 2],
    pub aspectRatio: libc::c_float,
}
pub type YGDimension = YGDimension_0;
pub const YGAlignBaseline: YGAlign = 5;
pub type YGStringStream_0 = YGStringStream;
pub const YGEdgeEnd: YGEdge_0 = 5;
pub const _XOPEN_: _LIB_VERSION_TYPE = 1;
pub type __off_t = libc::c_long;
pub const _SVID_: _LIB_VERSION_TYPE = 0;
pub const YGUnitUndefined: YGUnit = 0;
pub type uint32_t = libc::c_uint;
pub type YGPositionType_0 = libc::c_uint;
pub type YGConfigRef = *mut YGConfig_0;
pub type YGEdge_0 = libc::c_uint;
pub const YGAlignFlexEnd: YGAlign = 3;
pub const YGOverflowVisible: YGOverflow = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGCachedMeasurement {
    pub availableWidth: libc::c_float,
    pub availableHeight: libc::c_float,
    pub widthMeasureMode: YGMeasureMode,
    pub heightMeasureMode: YGMeasureMode,
    pub computedWidth: libc::c_float,
    pub computedHeight: libc::c_float,
}
pub const _ISOC_: _LIB_VERSION_TYPE = 3;
pub type YGUnit = libc::c_uint;
pub type YGDirection_0 = YGDirection;
pub type YGMeasureMode = YGMeasureMode_0;
pub const YGUnitAuto: YGUnit = 3;
pub type YGJustify_0 = libc::c_uint;
pub const YGJustifySpaceBetween: YGJustify_0 = 3;
pub type YGConfig = YGConfig_0;
pub const YGExperimentalFeatureWebFlexBasis: YGExperimentalFeature = 0;
pub const YGDimensionWidth: YGDimension_0 = 0;
pub const YGDirectionRTL: YGDirection = 2;
pub type YGStyle_0 = YGStyle;
pub const YGJustifyFlexStart: YGJustify_0 = 0;
pub type YGNodeType = libc::c_uint;
pub type _LIB_VERSION_TYPE = libc::c_int;
pub type YGNodeType_0 = YGNodeType;
pub type YGExperimentalFeature = libc::c_uint;
pub type __builtin_va_list = [__va_list_tag; 1];
pub type YGNodeClonedFunc =
    Option<unsafe extern "C" fn(_: YGNodeRef, _: YGNodeRef, _: YGNodeRef, _: libc::c_int) -> ()>;
pub const YGAlignFlexStart: YGAlign = 1;
pub type int32_t = libc::c_int;
pub const YGJustifySpaceAround: YGJustify_0 = 4;
pub const YGAlignSpaceBetween: YGAlign = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGSize {
    pub width: libc::c_float,
    pub height: libc::c_float,
}
pub type YGDisplay_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const YGWrapWrapReverse: YGWrap = 2;
pub const YGMeasureModeUndefined: YGMeasureMode_0 = 0;
pub const YGDisplayNone: YGDisplay_0 = 1;
pub type YGCachedMeasurement_0 = YGCachedMeasurement;
pub type YGMeasureMode_0 = libc::c_uint;
pub type YGExperimentalFeature_0 = YGExperimentalFeature;
pub const YGFlexDirectionColumn: YGFlexDirection = 0;
pub type YGUnit_0 = YGUnit;
pub const YGNodeTypeDefault: YGNodeType = 0;
pub const YGJustifyFlexEnd: YGJustify_0 = 2;
pub type YGNodeRef = *mut YGNode_0;
pub type YGFlexDirection_0 = YGFlexDirection;
pub type _IO_lock_t = ();
pub type YGNode = YGNode_0;
pub type YGDimension_0 = libc::c_uint;
pub const YGFlexDirectionRow: YGFlexDirection = 2;
pub const _POSIX_: _LIB_VERSION_TYPE = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGNode_0 {
    pub style: YGStyle_0,
    pub layout: YGLayout_0,
    pub lineIndex: uint32_t,
    pub parent: YGNodeRef,
    pub children: YGNodeListRef,
    pub nextChild: *mut YGNode_0,
    pub measure: YGMeasureFunc,
    pub baseline: YGBaselineFunc,
    pub config: YGConfigRef,
    pub context: *mut libc::c_void,
    pub isDirty: bool,
    pub hasNewLayout: bool,
    pub nodeType: YGNodeType_0,
    pub resolvedDimensions: [*const YGValue; 2],
}
pub type YGBaselineFunc =
    Option<unsafe extern "C" fn(_: YGNodeRef, _: libc::c_float, _: libc::c_float) -> libc::c_float>;
pub type YGMeasureFunc = Option<
    unsafe extern "C" fn(
        _: YGNodeRef,
        _: libc::c_float,
        _: YGMeasureMode,
        _: libc::c_float,
        _: YGMeasureMode,
    ) -> YGSize_0,
>;
pub const YGPositionTypeAbsolute: YGPositionType_0 = 1;
pub const YGAlignCenter: YGAlign = 2;
pub const YGEdgeBottom: YGEdge_0 = 3;
pub const YGPositionTypeRelative: YGPositionType_0 = 0;
pub type __off64_t = libc::c_long;
pub const YGDirectionLTR: YGDirection = 1;
pub type YGCalloc = Option<unsafe extern "C" fn(_: size_t, _: size_t) -> *mut libc::c_void>;
pub type YGSize_0 = YGSize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGConfig_0 {
    pub experimentalFeatures: [bool; 2],
    pub useWebDefaults: bool,
    pub useLegacyStretchBehaviour: bool,
    pub pointScaleFactor: libc::c_float,
    pub cloneNodeCallback: YGNodeClonedFunc,
    pub context: *mut libc::c_void,
}
pub const YGEdgeHorizontal: YGEdge_0 = 6;
#[no_mangle]
pub unsafe extern "C" fn YGRoundValueToPixelGrid(
    value: libc::c_float,
    pointScaleFactor: libc::c_float,
    forceCeil: bool,
    forceFloor: bool,
) -> libc::c_float {
    let mut scaledValue: libc::c_float = value * pointScaleFactor;
    let mut fractial: libc::c_float = fmodf(scaledValue, 1.0f64 as libc::c_float);
    if YGFloatsEqual(fractial, 0i32 as libc::c_float) {
        scaledValue = scaledValue - fractial;
    } else {
        if YGFloatsEqual(fractial, 1.0f64 as libc::c_float) {
            scaledValue = ((scaledValue - fractial) as libc::c_double + 1.0f64) as libc::c_float;
        } else {
            if forceCeil {
                scaledValue = scaledValue - fractial + 1.0f32;
            } else {
                if forceFloor {
                    scaledValue = scaledValue - fractial;
                } else {
                    scaledValue = scaledValue - fractial
                        + if fractial > 0.5f32
                            || 0 != YGFloatsEqual(fractial, 0.5f32) as libc::c_int
                        {
                            1.0f32
                        } else {
                            0.0f32
                        };
                };
            };
        };
    };
    return scaledValue / pointScaleFactor;
}
unsafe extern "C" fn YGFloatsEqual(a: libc::c_float, b: libc::c_float) -> bool {
    if YGFloatIsUndefined(a) {
        return YGFloatIsUndefined(b);
    };
    return fabs((a - b) as libc::c_double) < 0.00009999999747378752f32 as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn YGFloatIsUndefined(value: libc::c_float) -> bool {
    return 0 != if ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        __isnanf(value)
    } else if ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        __isnan(value as libc::c_double)
    } else {
        __isnanl(value as libc::c_double)
    };
}
static mut YGValueUndefined: YGValue = unsafe {
    YGValue_0 {
        value: ::std::f32::NAN,
        unit: YGUnitUndefined,
    }
};
static mut YGValueAuto: YGValue = unsafe {
    YGValue_0 {
        value: ::std::f32::NAN,
        unit: YGUnitAuto,
    }
};
#[no_mangle]
pub unsafe extern "C" fn YGNodeNew() -> YGNodeRef {
    return YGNodeNewWithConfig(&mut gYGConfigDefaults as *mut YGConfig);
}
static mut gYGConfigDefaults: YGConfig = unsafe {
    YGConfig_0 {
        experimentalFeatures: [0 != 0i32, false],
        useWebDefaults: 0 != 0i32,
        useLegacyStretchBehaviour: false,
        pointScaleFactor: 1.0f32,
        cloneNodeCallback: None,
        context: 0 as *const libc::c_void as *mut libc::c_void,
    }
};
#[no_mangle]
pub unsafe extern "C" fn YGNodeNewWithConfig(config: YGConfigRef) -> YGNodeRef {
    let node: YGNodeRef = gYGMalloc.expect("non-null function pointer")(
        ::std::mem::size_of::<YGNode>() as libc::c_ulong,
    ) as YGNodeRef;
    YGAssertWithConfig(
        config,
        !node.is_null(),
        b"Could not allocate memory for node\x00" as *const u8 as *const libc::c_char,
    );
    gNodeInstanceCount += 1;
    memcpy(
        node as *mut libc::c_void,
        &gYGNodeDefaults as *const YGNode as *const libc::c_void,
        ::std::mem::size_of::<YGNode>() as libc::c_ulong,
    );
    if (*config).useWebDefaults {
        (*node).style.flexDirection = YGFlexDirectionRow;
        (*node).style.alignContent = YGAlignStretch;
    };
    (*node).config = config;
    return node;
}
#[no_mangle]
pub static mut gYGMalloc: YGMalloc = unsafe { Some(malloc) };
static mut gYGNodeDefaults: YGNode = unsafe {
    YGNode_0 {
        style: YGStyle {
            direction: YGDirectionInherit,
            flexDirection: YGFlexDirectionColumn,
            justifyContent: YGJustifyFlexStart,
            alignContent: YGAlignFlexStart,
            alignItems: YGAlignStretch,
            alignSelf: YGAlignAuto,
            positionType: YGPositionTypeRelative,
            flexWrap: YGWrapNoWrap,
            overflow: YGOverflowVisible,
            display: YGDisplayFlex,
            flex: ::std::f32::NAN,
            flexGrow: ::std::f32::NAN,
            flexShrink: ::std::f32::NAN,
            flexBasis: YGValue_0 {
                value: ::std::f32::NAN,
                unit: YGUnitAuto,
            },
            margin: [
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
            ],
            position: [
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
            ],
            padding: [
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
            ],
            border: [
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
            ],
            dimensions: [
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitAuto,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitAuto,
                },
            ],
            minDimensions: [
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
            ],
            maxDimensions: [
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
            ],
            aspectRatio: ::std::f32::NAN,
        },
        layout: YGLayout {
            position: [0.; 4],
            dimensions: [::std::f32::NAN, ::std::f32::NAN],
            margin: [0.; 6],
            border: [0.; 6],
            padding: [0.; 6],
            direction: YGDirectionInherit,
            computedFlexBasisGeneration: 0,
            computedFlexBasis: ::std::f32::NAN,
            hadOverflow: 0 != 0i32,
            generationCount: 0,
            lastParentDirection: 4294967295 as YGDirection_0,
            nextCachedMeasurementsIndex: 0i32 as uint32_t,
            cachedMeasurements: [YGCachedMeasurement {
                availableWidth: 0.,
                availableHeight: 0.,
                widthMeasureMode: YGMeasureModeUndefined,
                heightMeasureMode: YGMeasureModeUndefined,
                computedWidth: 0.,
                computedHeight: 0.,
            }; 16],
            measuredDimensions: [::std::f32::NAN, ::std::f32::NAN],
            cachedLayout: YGCachedMeasurement {
                availableWidth: 0.,
                availableHeight: 0.,
                widthMeasureMode: 4294967295 as YGMeasureMode,
                heightMeasureMode: 4294967295 as YGMeasureMode,
                computedWidth: -1i32 as libc::c_float,
                computedHeight: -1i32 as libc::c_float,
            },
        },
        lineIndex: 0,
        parent: 0 as *const YGNode_0 as YGNodeRef,
        children: 0 as *const YGNodeList as YGNodeListRef,
        nextChild: 0 as *const YGNode_0 as *mut YGNode_0,
        measure: None,
        baseline: None,
        config: 0 as *const YGConfig_0 as *mut YGConfig_0,
        context: 0 as *const libc::c_void as *mut libc::c_void,
        isDirty: 0 != 0i32,
        hasNewLayout: 0 != 1i32,
        nodeType: YGNodeTypeDefault,
        resolvedDimensions: [
            &YGValueUndefined as *const YGValue,
            &YGValueUndefined as *const YGValue,
        ],
    }
};
#[no_mangle]
pub static mut gNodeInstanceCount: int32_t = unsafe { 0i32 };
#[no_mangle]
pub unsafe extern "C" fn YGAssertWithConfig(
    config: YGConfigRef,
    condition: bool,
    mut message: *const libc::c_char,
) -> () {
    if !condition {
        error!("{} (config: {:?})", CStr::from(message).unwrap(), config);
    };
}

#[no_mangle]
pub unsafe extern "C" fn YGNodeClone(oldNode: YGNodeRef) -> YGNodeRef {
    let node: YGNodeRef = gYGMalloc.expect("non-null function pointer")(
        ::std::mem::size_of::<YGNode>() as libc::c_ulong,
    ) as YGNodeRef;
    YGAssertWithConfig(
        (*oldNode).config,
        !node.is_null(),
        b"Could not allocate memory for node\x00" as *const u8 as *const libc::c_char,
    );
    gNodeInstanceCount += 1;
    memcpy(
        node as *mut libc::c_void,
        oldNode as *const libc::c_void,
        ::std::mem::size_of::<YGNode>() as libc::c_ulong,
    );
    (*node).children = YGNodeListClone((*oldNode).children);
    (*node).parent = 0 as YGNodeRef;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListClone(oldList: YGNodeListRef) -> YGNodeListRef {
    if oldList.is_null() {
        return 0 as YGNodeListRef;
    };
    let count: uint32_t = (*oldList).count;
    if count == 0i32 as libc::c_uint {
        return 0 as YGNodeListRef;
    };
    let newList: YGNodeListRef = YGNodeListNew(count);
    memcpy(
        (*newList).items as *mut libc::c_void,
        (*oldList).items as *const libc::c_void,
        (::std::mem::size_of::<YGNodeRef>() as libc::c_ulong).wrapping_mul(count as libc::c_ulong),
    );
    (*newList).count = count;
    return newList;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListNew(initialCapacity: uint32_t) -> YGNodeListRef {
    let list: YGNodeListRef = gYGMalloc.expect("non-null function pointer")(::std::mem::size_of::<
        YGNodeList,
    >()
        as libc::c_ulong) as YGNodeListRef;
    YGAssert(
        !list.is_null(),
        b"Could not allocate memory for list\x00" as *const u8 as *const libc::c_char,
    );
    (*list).capacity = initialCapacity;
    (*list).count = 0i32 as uint32_t;
    (*list).items = gYGMalloc.expect("non-null function pointer")(
        (::std::mem::size_of::<YGNodeRef>() as libc::c_ulong)
            .wrapping_mul((*list).capacity as libc::c_ulong),
    ) as *mut YGNodeRef;
    YGAssert(
        !(*list).items.is_null(),
        b"Could not allocate memory for items\x00" as *const u8 as *const libc::c_char,
    );
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn YGAssert(condition: bool, mut message: *const libc::c_char) -> () {
    if !condition {
        error!("{}", CStr::from(message).unwrap());
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeFree(node: YGNodeRef) -> () {
    if !(*node).parent.is_null() {
        YGNodeListDelete((*(*node).parent).children, node);
        (*node).parent = 0 as YGNodeRef;
    };
    let childCount: uint32_t = YGNodeGetChildCount(node);
    {
        let mut i: uint32_t = 0i32 as uint32_t;
        while i < childCount {
            {
                let child: YGNodeRef = YGNodeGetChild(node, i);
                (*child).parent = 0 as YGNodeRef;
            }
            i = i.wrapping_add(1);
        }
    }
    YGNodeListFree((*node).children);
    gYGFree.expect("non-null function pointer")(node as *mut libc::c_void);
    gNodeInstanceCount -= 1;
}
#[no_mangle]
pub static mut gYGFree: YGFree = unsafe { Some(free) };
#[no_mangle]
pub unsafe extern "C" fn YGNodeListFree(list: YGNodeListRef) -> () {
    if !list.is_null() {
        gYGFree.expect("non-null function pointer")((*list).items as *mut libc::c_void);
        gYGFree.expect("non-null function pointer")(list as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetChild(node: YGNodeRef, index: uint32_t) -> YGNodeRef {
    return YGNodeListGet((*node).children, index);
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListGet(list: YGNodeListRef, index: uint32_t) -> YGNodeRef {
    if YGNodeListCount(list) > 0i32 as libc::c_uint {
        return *(*list).items.offset(index as isize);
    };
    return 0 as YGNodeRef;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListCount(list: YGNodeListRef) -> uint32_t {
    if !list.is_null() {
        return (*list).count;
    };
    return 0i32 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetChildCount(node: YGNodeRef) -> uint32_t {
    return YGNodeListCount((*node).children);
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListDelete(list: YGNodeListRef, node: YGNodeRef) -> YGNodeRef {
    {
        let mut i: uint32_t = 0i32 as uint32_t;
        while i < (*list).count {
            {
                if *(*list).items.offset(i as isize) == node {
                    return YGNodeListRemove(list, i);
                };
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as YGNodeRef;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListRemove(list: YGNodeListRef, index: uint32_t) -> YGNodeRef {
    let removed: YGNodeRef = *(*list).items.offset(index as isize);
    let ref mut fresh0 = *(*list).items.offset(index as isize);
    *fresh0 = 0 as YGNodeRef;
    {
        let mut i: uint32_t = index;
        while i < (*list).count.wrapping_sub(1i32 as libc::c_uint) {
            {
                let ref mut fresh1 = *(*list).items.offset(i as isize);
                *fresh1 = *(*list)
                    .items
                    .offset(i.wrapping_add(1i32 as libc::c_uint) as isize);
                let ref mut fresh2 = *(*list)
                    .items
                    .offset(i.wrapping_add(1i32 as libc::c_uint) as isize);
                *fresh2 = 0 as YGNodeRef;
            }
            i = i.wrapping_add(1);
        }
    }
    (*list).count = (*list).count.wrapping_sub(1);
    return removed;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeFreeRecursive(root: YGNodeRef) -> () {
    'loop0: while YGNodeGetChildCount(root) > 0i32 as libc::c_uint {
        let child: YGNodeRef = YGNodeGetChild(root, 0i32 as uint32_t);
        if (*child).parent != root {
            break 'loop0;
        };
        YGNodeRemoveChild(root, child);
        YGNodeFreeRecursive(child);
    }
    YGNodeFree(root);
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeRemoveChild(parent: YGNodeRef, excludedChild: YGNodeRef) -> () {
    let childCount: uint32_t = YGNodeGetChildCount(parent);
    if childCount == 0i32 as libc::c_uint {
        return;
    };
    let firstChild: YGNodeRef = YGNodeGetChild(parent, 0i32 as uint32_t);
    if (*firstChild).parent == parent {
        if !YGNodeListDelete((*parent).children, excludedChild).is_null() {
            (*excludedChild).layout = gYGNodeDefaults.layout;
            (*excludedChild).parent = 0 as YGNodeRef;
            YGNodeMarkDirtyInternal(parent);
        };
        return;
    };
    let cloneNodeCallback: YGNodeClonedFunc = (*(*parent).config).cloneNodeCallback;
    let children: YGNodeListRef = (*parent).children;
    let mut nextInsertIndex: uint32_t = 0i32 as uint32_t;
    {
        let mut i: uint32_t = 0i32 as uint32_t;
        while i < childCount {
            'body0: loop {
                {
                    let oldChild: YGNodeRef = YGNodeListGet(children, i);
                    if excludedChild == oldChild {
                        YGNodeMarkDirtyInternal(parent);
                        break 'body0;
                    };
                    let newChild: YGNodeRef = YGNodeClone(oldChild);
                    YGNodeListReplace(children, nextInsertIndex, newChild);
                    (*newChild).parent = parent;
                    if cloneNodeCallback.is_some() {
                        cloneNodeCallback.expect("non-null function pointer")(
                            oldChild,
                            newChild,
                            parent,
                            nextInsertIndex as libc::c_int,
                        );
                    };
                    nextInsertIndex = nextInsertIndex.wrapping_add(1);
                }
                break 'body0;
            }
            i = i.wrapping_add(1);
        }
    }
    while nextInsertIndex < childCount {
        YGNodeListRemove(children, nextInsertIndex);
        nextInsertIndex = nextInsertIndex.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListReplace(
    mut list: YGNodeListRef,
    index: uint32_t,
    newNode: YGNodeRef,
) -> () {
    let ref mut fresh3 = *(*list).items.offset(index as isize);
    *fresh3 = newNode;
}
unsafe extern "C" fn YGNodeMarkDirtyInternal(node: YGNodeRef) -> () {
    if !(*node).isDirty {
        (*node).isDirty = 0 != 1i32;
        (*node).layout.computedFlexBasis = ::std::f32::NAN;
        if !(*node).parent.is_null() {
            YGNodeMarkDirtyInternal((*node).parent);
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeReset(node: YGNodeRef) -> () {
    YGAssertWithNode(
        node,
        YGNodeGetChildCount(node) == 0i32 as libc::c_uint,
        b"Cannot reset a node which still has children attached\x00" as *const u8
            as *const libc::c_char,
    );
    YGAssertWithNode(
        node,
        (*node).parent.is_null(),
        b"Cannot reset a node still attached to a parent\x00" as *const u8 as *const libc::c_char,
    );
    YGNodeListFree((*node).children);
    let config: YGConfigRef = (*node).config;
    memcpy(
        node as *mut libc::c_void,
        &gYGNodeDefaults as *const YGNode as *const libc::c_void,
        ::std::mem::size_of::<YGNode>() as libc::c_ulong,
    );
    if (*config).useWebDefaults {
        (*node).style.flexDirection = YGFlexDirectionRow;
        (*node).style.alignContent = YGAlignStretch;
    };
    (*node).config = config;
}
#[no_mangle]
pub unsafe extern "C" fn YGAssertWithNode(
    node: YGNodeRef,
    condition: bool,
    mut message: *const libc::c_char,
) -> () {
    if !condition {
        error!("{} (node: {:?})", CStr::from(message).unwrap(), node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetInstanceCount() -> int32_t {
    return gNodeInstanceCount;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeInsertChild(
    node: YGNodeRef,
    child: YGNodeRef,
    index: uint32_t,
) -> () {
    YGAssertWithNode(
        node,
        (*child).parent.is_null(),
        b"Child already has a parent, it must be removed first.\x00" as *const u8
            as *const libc::c_char,
    );
    YGAssertWithNode(
        node,
        (*node).measure.is_none(),
        b"Cannot add child: Nodes with measure functions cannot have children.\x00" as *const u8
            as *const libc::c_char,
    );
    YGCloneChildrenIfNeeded(node);
    YGNodeListInsert(&mut (*node).children as *mut YGNodeListRef, child, index);
    (*child).parent = node;
    YGNodeMarkDirtyInternal(node);
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListInsert(
    mut listp: *mut YGNodeListRef,
    node: YGNodeRef,
    index: uint32_t,
) -> () {
    if (*listp).is_null() {
        *listp = YGNodeListNew(4i32 as uint32_t);
    };
    let mut list: YGNodeListRef = *listp;
    if (*list).count == (*list).capacity {
        (*list).capacity = ((*list).capacity as libc::c_uint).wrapping_mul(2i32 as libc::c_uint)
            as uint32_t as uint32_t;
        (*list).items = gYGRealloc.expect("non-null function pointer")(
            (*list).items as *mut libc::c_void,
            (::std::mem::size_of::<YGNodeRef>() as libc::c_ulong)
                .wrapping_mul((*list).capacity as libc::c_ulong),
        ) as *mut YGNodeRef;
        YGAssert(
            !(*list).items.is_null(),
            b"Could not extend allocation for items\x00" as *const u8 as *const libc::c_char,
        );
    };
    {
        let mut i: uint32_t = (*list).count;
        while i > index {
            {
                let ref mut fresh4 = *(*list).items.offset(i as isize);
                *fresh4 = *(*list)
                    .items
                    .offset(i.wrapping_sub(1i32 as libc::c_uint) as isize);
            }
            i = i.wrapping_sub(1);
        }
    }
    (*list).count = (*list).count.wrapping_add(1);
    let ref mut fresh5 = *(*list).items.offset(index as isize);
    *fresh5 = node;
}
#[no_mangle]
pub static mut gYGRealloc: YGRealloc = unsafe { Some(realloc) };
unsafe extern "C" fn YGCloneChildrenIfNeeded(parent: YGNodeRef) -> () {
    let childCount: uint32_t = YGNodeGetChildCount(parent);
    if childCount == 0i32 as libc::c_uint {
        return;
    };
    let firstChild: YGNodeRef = YGNodeGetChild(parent, 0i32 as uint32_t);
    if (*firstChild).parent == parent {
        return;
    };
    let cloneNodeCallback: YGNodeClonedFunc = (*(*parent).config).cloneNodeCallback;
    let children: YGNodeListRef = (*parent).children;
    {
        let mut i: uint32_t = 0i32 as uint32_t;
        while i < childCount {
            {
                let oldChild: YGNodeRef = YGNodeListGet(children, i);
                let newChild: YGNodeRef = YGNodeClone(oldChild);
                YGNodeListReplace(children, i, newChild);
                (*newChild).parent = parent;
                if cloneNodeCallback.is_some() {
                    cloneNodeCallback.expect("non-null function pointer")(
                        oldChild,
                        newChild,
                        parent,
                        i as libc::c_int,
                    );
                };
            }
            i = i.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeRemoveAllChildren(parent: YGNodeRef) -> () {
    let childCount: uint32_t = YGNodeGetChildCount(parent);
    if childCount == 0i32 as libc::c_uint {
        return;
    };
    let firstChild: YGNodeRef = YGNodeGetChild(parent, 0i32 as uint32_t);
    if (*firstChild).parent == parent {
        {
            let mut i: uint32_t = 0i32 as uint32_t;
            while i < childCount {
                {
                    let oldChild: YGNodeRef = YGNodeGetChild(parent, i);
                    (*oldChild).layout = gYGNodeDefaults.layout;
                    (*oldChild).parent = 0 as YGNodeRef;
                }
                i = i.wrapping_add(1);
            }
        }
        YGNodeListRemoveAll((*parent).children);
        YGNodeMarkDirtyInternal(parent);
        return;
    };
    (*parent).children = 0 as YGNodeListRef;
    YGNodeMarkDirtyInternal(parent);
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListRemoveAll(list: YGNodeListRef) -> () {
    {
        let mut i: uint32_t = 0i32 as uint32_t;
        while i < (*list).count {
            {
                let ref mut fresh6 = *(*list).items.offset(i as isize);
                *fresh6 = 0 as YGNodeRef;
            }
            i = i.wrapping_add(1);
        }
    }
    (*list).count = 0i32 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetParent(node: YGNodeRef) -> YGNodeRef {
    return (*node).parent;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeCalculateLayout(
    node: YGNodeRef,
    parentWidth: libc::c_float,
    parentHeight: libc::c_float,
    parentDirection: YGDirection_0,
) -> () {
    gCurrentGenerationCount = gCurrentGenerationCount.wrapping_add(1);
    YGResolveDimensions(node);
    let mut width: libc::c_float = ::std::f32::NAN;
    let mut widthMeasureMode: YGMeasureMode = YGMeasureModeUndefined;
    if YGNodeIsStyleDimDefined(node, YGFlexDirectionRow, parentWidth) {
        width = YGResolveValue(
            (*node).resolvedDimensions[dim[YGFlexDirectionRow as libc::c_int as usize] as usize],
            parentWidth,
        ) + YGNodeMarginForAxis(node, YGFlexDirectionRow, parentWidth);
        widthMeasureMode = YGMeasureModeExactly;
    } else {
        if YGResolveValue(
            &mut (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize]
                as *mut YGValue,
            parentWidth,
        ) >= 0.0f32
        {
            width = YGResolveValue(
                &mut (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize]
                    as *mut YGValue,
                parentWidth,
            );
            widthMeasureMode = YGMeasureModeAtMost;
        } else {
            width = parentWidth;
            widthMeasureMode = (if 0 != YGFloatIsUndefined(width) as libc::c_int {
                YGMeasureModeUndefined as libc::c_int
            } else {
                YGMeasureModeExactly as libc::c_int
            }) as YGMeasureMode;
        };
    };
    let mut height: libc::c_float = ::std::f32::NAN;
    let mut heightMeasureMode: YGMeasureMode = YGMeasureModeUndefined;
    if YGNodeIsStyleDimDefined(node, YGFlexDirectionColumn, parentHeight) {
        height = YGResolveValue(
            (*node).resolvedDimensions[dim[YGFlexDirectionColumn as libc::c_int as usize] as usize],
            parentHeight,
        ) + YGNodeMarginForAxis(node, YGFlexDirectionColumn, parentWidth);
        heightMeasureMode = YGMeasureModeExactly;
    } else {
        if YGResolveValue(
            &mut (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize]
                as *mut YGValue,
            parentHeight,
        ) >= 0.0f32
        {
            height = YGResolveValue(
                &mut (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize]
                    as *mut YGValue,
                parentHeight,
            );
            heightMeasureMode = YGMeasureModeAtMost;
        } else {
            height = parentHeight;
            heightMeasureMode = (if 0 != YGFloatIsUndefined(height) as libc::c_int {
                YGMeasureModeUndefined as libc::c_int
            } else {
                YGMeasureModeExactly as libc::c_int
            }) as YGMeasureMode;
        };
    };
    if YGLayoutNodeInternal(
        node,
        width,
        height,
        parentDirection,
        widthMeasureMode,
        heightMeasureMode,
        parentWidth,
        parentHeight,
        0 != 1i32,
        b"initial\x00" as *const u8 as *const libc::c_char,
        (*node).config,
    ) {
        YGNodeSetPosition(
            node,
            (*node).layout.direction,
            parentWidth,
            parentHeight,
            parentWidth,
        );
        YGRoundToPixelGrid(node, (*(*node).config).pointScaleFactor, 0.0f32, 0.0f32);
    };
}

unsafe extern "C" fn YGComputedEdgeValue(
    mut edges: *const YGValue,
    edge: YGEdge,
    defaultValue: *const YGValue,
) -> *const YGValue {
    if (*edges.offset(edge as isize)).unit as libc::c_uint
        != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return &*edges.offset(edge as isize) as *const YGValue;
    };
    if (edge as libc::c_uint == YGEdgeTop as libc::c_int as libc::c_uint
        || edge as libc::c_uint == YGEdgeBottom as libc::c_int as libc::c_uint)
        && (*edges.offset(YGEdgeVertical as libc::c_int as isize)).unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return &*edges.offset(YGEdgeVertical as libc::c_int as isize) as *const YGValue;
    };
    if (edge as libc::c_uint == YGEdgeLeft as libc::c_int as libc::c_uint
        || edge as libc::c_uint == YGEdgeRight as libc::c_int as libc::c_uint
        || edge as libc::c_uint == YGEdgeStart as libc::c_int as libc::c_uint
        || edge as libc::c_uint == YGEdgeEnd as libc::c_int as libc::c_uint)
        && (*edges.offset(YGEdgeHorizontal as libc::c_int as isize)).unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return &*edges.offset(YGEdgeHorizontal as libc::c_int as isize) as *const YGValue;
    };
    if (*edges.offset(YGEdgeAll as libc::c_int as isize)).unit as libc::c_uint
        != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return &*edges.offset(YGEdgeAll as libc::c_int as isize) as *const YGValue;
    };
    if edge as libc::c_uint == YGEdgeStart as libc::c_int as libc::c_uint
        || edge as libc::c_uint == YGEdgeEnd as libc::c_int as libc::c_uint
    {
        return &YGValueUndefined as *const YGValue;
    };
    return defaultValue;
}
unsafe extern "C" fn YGNodeSetPosition(
    node: YGNodeRef,
    direction: YGDirection_0,
    mainSize: libc::c_float,
    crossSize: libc::c_float,
    parentWidth: libc::c_float,
) -> () {
    let directionRespectingRoot: YGDirection_0 = (if !(*node).parent.is_null() {
        direction as libc::c_uint
    } else {
        YGDirectionLTR as libc::c_int as libc::c_uint
    }) as YGDirection_0;
    let mainAxis: YGFlexDirection_0 =
        YGResolveFlexDirection((*node).style.flexDirection, directionRespectingRoot);
    let crossAxis: YGFlexDirection_0 = YGFlexDirectionCross(mainAxis, directionRespectingRoot);
    let relativePositionMain: libc::c_float = YGNodeRelativePosition(node, mainAxis, mainSize);
    let relativePositionCross: libc::c_float = YGNodeRelativePosition(node, crossAxis, crossSize);
    (*node).layout.position[leading[mainAxis as usize] as usize] =
        YGNodeLeadingMargin(node, mainAxis, parentWidth) + relativePositionMain;
    (*node).layout.position[trailing[mainAxis as usize] as usize] =
        YGNodeTrailingMargin(node, mainAxis, parentWidth) + relativePositionMain;
    (*node).layout.position[leading[crossAxis as usize] as usize] =
        YGNodeLeadingMargin(node, crossAxis, parentWidth) + relativePositionCross;
    (*node).layout.position[trailing[crossAxis as usize] as usize] =
        YGNodeTrailingMargin(node, crossAxis, parentWidth) + relativePositionCross;
}
unsafe extern "C" fn YGResolveFlexDirection(
    flexDirection: YGFlexDirection_0,
    direction: YGDirection_0,
) -> YGFlexDirection_0 {
    if direction as libc::c_uint == YGDirectionRTL as libc::c_int as libc::c_uint {
        if flexDirection as libc::c_uint == YGFlexDirectionRow as libc::c_int as libc::c_uint {
            return YGFlexDirectionRowReverse;
        } else {
            if flexDirection as libc::c_uint
                == YGFlexDirectionRowReverse as libc::c_int as libc::c_uint
            {
                return YGFlexDirectionRow;
            };
        };
    };
    return flexDirection;
}
unsafe extern "C" fn YGFlexDirectionCross(
    flexDirection: YGFlexDirection_0,
    direction: YGDirection_0,
) -> YGFlexDirection_0 {
    return (if 0 != YGFlexDirectionIsColumn(flexDirection) as libc::c_int {
        YGResolveFlexDirection(YGFlexDirectionRow, direction) as libc::c_uint
    } else {
        YGFlexDirectionColumn as libc::c_int as libc::c_uint
    }) as YGFlexDirection_0;
}
unsafe extern "C" fn YGFlexDirectionIsColumn(flexDirection: YGFlexDirection_0) -> bool {
    return flexDirection as libc::c_uint == YGFlexDirectionColumn as libc::c_int as libc::c_uint
        || flexDirection as libc::c_uint
            == YGFlexDirectionColumnReverse as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn YGNodeRelativePosition(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    axisSize: libc::c_float,
) -> libc::c_float {
    return if 0 != YGNodeIsLeadingPosDefined(node, axis) as libc::c_int {
        YGNodeLeadingPosition(node, axis, axisSize)
    } else {
        -YGNodeTrailingPosition(node, axis, axisSize)
    };
}
unsafe extern "C" fn YGNodeTrailingPosition(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    axisSize: libc::c_float,
) -> libc::c_float {
    if YGFlexDirectionIsRow(axis) {
        let mut trailingPosition: *const YGValue = YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            YGEdgeEnd,
            &YGValueUndefined as *const YGValue,
        );
        if (*trailingPosition).unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
        {
            return YGResolveValue(trailingPosition, axisSize);
        };
    };
    let mut trailingPosition: *const YGValue = YGComputedEdgeValue(
        (*node).style.position.as_mut_ptr() as *const YGValue,
        trailing[axis as usize],
        &YGValueUndefined as *const YGValue,
    );
    return if (*trailingPosition).unit as libc::c_uint
        == YGUnitUndefined as libc::c_int as libc::c_uint
    {
        0.0f32
    } else {
        YGResolveValue(trailingPosition, axisSize)
    };
}
static mut trailing: [YGEdge; 4] = unsafe { [YGEdgeBottom, YGEdgeTop, YGEdgeRight, YGEdgeLeft] };
unsafe extern "C" fn YGFlexDirectionIsRow(flexDirection: YGFlexDirection_0) -> bool {
    return flexDirection as libc::c_uint == YGFlexDirectionRow as libc::c_int as libc::c_uint
        || flexDirection as libc::c_uint
            == YGFlexDirectionRowReverse as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn YGNodeLeadingPosition(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    axisSize: libc::c_float,
) -> libc::c_float {
    if YGFlexDirectionIsRow(axis) {
        let mut leadingPosition: *const YGValue = YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            YGEdgeStart,
            &YGValueUndefined as *const YGValue,
        );
        if (*leadingPosition).unit as libc::c_uint != YGUnitUndefined as libc::c_int as libc::c_uint
        {
            return YGResolveValue(leadingPosition, axisSize);
        };
    };
    let mut leadingPosition: *const YGValue = YGComputedEdgeValue(
        (*node).style.position.as_mut_ptr() as *const YGValue,
        leading[axis as usize],
        &YGValueUndefined as *const YGValue,
    );
    return if (*leadingPosition).unit as libc::c_uint
        == YGUnitUndefined as libc::c_int as libc::c_uint
    {
        0.0f32
    } else {
        YGResolveValue(leadingPosition, axisSize)
    };
}
static mut leading: [YGEdge; 4] = unsafe { [YGEdgeTop, YGEdgeBottom, YGEdgeLeft, YGEdgeRight] };
unsafe extern "C" fn YGNodeIsLeadingPosDefined(node: YGNodeRef, axis: YGFlexDirection_0) -> bool {
    return 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            YGEdgeStart,
            &YGValueUndefined as *const YGValue,
        )).unit as libc::c_uint != YGUnitUndefined as libc::c_int as libc::c_uint
        || (*YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            leading[axis as usize],
            &YGValueUndefined as *const YGValue,
        )).unit as libc::c_uint != YGUnitUndefined as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn YGNodeTrailingMargin(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    if 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*node).style.margin[YGEdgeEnd as libc::c_int as usize].unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return YGResolveValueMargin(
            &mut (*node).style.margin[YGEdgeEnd as libc::c_int as usize] as *mut YGValue,
            widthSize,
        );
    };
    return YGResolveValueMargin(
        YGComputedEdgeValue(
            (*node).style.margin.as_mut_ptr() as *const YGValue,
            trailing[axis as usize],
            &mut YGValueZero as *mut YGValue,
        ),
        widthSize,
    );
}
static mut YGValueZero: YGValue = unsafe {
    YGValue_0 {
        value: 0i32 as libc::c_float,
        unit: YGUnitPoint,
    }
};
unsafe extern "C" fn YGResolveValueMargin(
    value: *const YGValue,
    parentSize: libc::c_float,
) -> libc::c_float {
    return if (*value).unit as libc::c_uint == YGUnitAuto as libc::c_int as libc::c_uint {
        0i32 as libc::c_float
    } else {
        YGResolveValue(value, parentSize)
    };
}
unsafe extern "C" fn YGNodeLeadingMargin(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    if 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*node).style.margin[YGEdgeStart as libc::c_int as usize].unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return YGResolveValueMargin(
            &mut (*node).style.margin[YGEdgeStart as libc::c_int as usize] as *mut YGValue,
            widthSize,
        );
    };
    return YGResolveValueMargin(
        YGComputedEdgeValue(
            (*node).style.margin.as_mut_ptr() as *const YGValue,
            leading[axis as usize],
            &mut YGValueZero as *mut YGValue,
        ),
        widthSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn YGLayoutNodeInternal(
    node: YGNodeRef,
    availableWidth: libc::c_float,
    availableHeight: libc::c_float,
    parentDirection: YGDirection_0,
    widthMeasureMode: YGMeasureMode,
    heightMeasureMode: YGMeasureMode,
    parentWidth: libc::c_float,
    parentHeight: libc::c_float,
    performLayout: bool,
    mut reason: *const libc::c_char,
    config: YGConfigRef,
) -> bool {
    let mut layout: *mut YGLayout_0 = &mut (*node).layout as *mut YGLayout_0;
    gDepth = gDepth.wrapping_add(1);
    let needToVisitNode: bool = 0 != (*node).isDirty as libc::c_int
        && (*layout).generationCount != gCurrentGenerationCount
        || (*layout).lastParentDirection as libc::c_uint != parentDirection as libc::c_uint;
    if needToVisitNode {
        (*layout).nextCachedMeasurementsIndex = 0i32 as uint32_t;
        (*layout).cachedLayout.widthMeasureMode = 4294967295 as YGMeasureMode;
        (*layout).cachedLayout.heightMeasureMode = 4294967295 as YGMeasureMode;
        (*layout).cachedLayout.computedWidth = -1i32 as libc::c_float;
        (*layout).cachedLayout.computedHeight = -1i32 as libc::c_float;
    };
    let mut cachedResults: *mut YGCachedMeasurement_0 = 0 as *mut YGCachedMeasurement_0;
    if (*node).measure.is_some() {
        let marginAxisRow: libc::c_float =
            YGNodeMarginForAxis(node, YGFlexDirectionRow, parentWidth);
        let marginAxisColumn: libc::c_float =
            YGNodeMarginForAxis(node, YGFlexDirectionColumn, parentWidth);
        if YGNodeCanUseCachedMeasurement(
            widthMeasureMode,
            availableWidth,
            heightMeasureMode,
            availableHeight,
            (*layout).cachedLayout.widthMeasureMode,
            (*layout).cachedLayout.availableWidth,
            (*layout).cachedLayout.heightMeasureMode,
            (*layout).cachedLayout.availableHeight,
            (*layout).cachedLayout.computedWidth,
            (*layout).cachedLayout.computedHeight,
            marginAxisRow,
            marginAxisColumn,
            config,
        ) {
            cachedResults = &mut (*layout).cachedLayout as *mut YGCachedMeasurement_0;
        } else {
            let mut i: uint32_t = 0i32 as uint32_t;
            'loop1: while i < (*layout).nextCachedMeasurementsIndex {
                {
                    if YGNodeCanUseCachedMeasurement(
                        widthMeasureMode,
                        availableWidth,
                        heightMeasureMode,
                        availableHeight,
                        (*layout).cachedMeasurements[i as usize].widthMeasureMode,
                        (*layout).cachedMeasurements[i as usize].availableWidth,
                        (*layout).cachedMeasurements[i as usize].heightMeasureMode,
                        (*layout).cachedMeasurements[i as usize].availableHeight,
                        (*layout).cachedMeasurements[i as usize].computedWidth,
                        (*layout).cachedMeasurements[i as usize].computedHeight,
                        marginAxisRow,
                        marginAxisColumn,
                        config,
                    ) {
                        cachedResults = &mut (*layout).cachedMeasurements[i as usize]
                            as *mut YGCachedMeasurement_0;
                        break 'loop1;
                    };
                }
                i = i.wrapping_add(1);
            }
        };
    } else {
        if performLayout {
            if 0 != YGFloatsEqual((*layout).cachedLayout.availableWidth, availableWidth)
                as libc::c_int
                && 0 != YGFloatsEqual((*layout).cachedLayout.availableHeight, availableHeight)
                    as libc::c_int
                && (*layout).cachedLayout.widthMeasureMode as libc::c_uint
                    == widthMeasureMode as libc::c_uint
                && (*layout).cachedLayout.heightMeasureMode as libc::c_uint
                    == heightMeasureMode as libc::c_uint
            {
                cachedResults = &mut (*layout).cachedLayout as *mut YGCachedMeasurement_0;
            };
        } else {
            let mut i: uint32_t = 0i32 as uint32_t;
            'loop2: while i < (*layout).nextCachedMeasurementsIndex {
                {
                    if 0 != YGFloatsEqual(
                        (*layout).cachedMeasurements[i as usize].availableWidth,
                        availableWidth,
                    ) as libc::c_int
                        && 0 != YGFloatsEqual(
                            (*layout).cachedMeasurements[i as usize].availableHeight,
                            availableHeight,
                        ) as libc::c_int
                        && (*layout).cachedMeasurements[i as usize].widthMeasureMode as libc::c_uint
                            == widthMeasureMode as libc::c_uint
                        && (*layout).cachedMeasurements[i as usize].heightMeasureMode
                            as libc::c_uint
                            == heightMeasureMode as libc::c_uint
                    {
                        cachedResults = &mut (*layout).cachedMeasurements[i as usize]
                            as *mut YGCachedMeasurement_0;
                        break 'loop2;
                    };
                }
                i = i.wrapping_add(1);
            }
        };
    };
    if !needToVisitNode && !cachedResults.is_null() {
        (*layout).measuredDimensions[YGDimensionWidth as libc::c_int as usize] =
            (*cachedResults).computedWidth;
        (*layout).measuredDimensions[YGDimensionHeight as libc::c_int as usize] =
            (*cachedResults).computedHeight;
    } else {
        YGNodelayoutImpl(
            node,
            availableWidth,
            availableHeight,
            parentDirection,
            widthMeasureMode,
            heightMeasureMode,
            parentWidth,
            parentHeight,
            performLayout,
            config,
        );
        (*layout).lastParentDirection = parentDirection;
        if cachedResults.is_null() {
            if (*layout).nextCachedMeasurementsIndex == 16i32 as libc::c_uint {
                (*layout).nextCachedMeasurementsIndex = 0i32 as uint32_t;
            };
            let mut newCacheEntry: *mut YGCachedMeasurement_0 = 0 as *mut YGCachedMeasurement_0;
            if performLayout {
                newCacheEntry = &mut (*layout).cachedLayout as *mut YGCachedMeasurement_0;
            } else {
                newCacheEntry = &mut (*layout).cachedMeasurements
                    [(*layout).nextCachedMeasurementsIndex as usize]
                    as *mut YGCachedMeasurement_0;
                (*layout).nextCachedMeasurementsIndex =
                    (*layout).nextCachedMeasurementsIndex.wrapping_add(1);
            };
            (*newCacheEntry).availableWidth = availableWidth;
            (*newCacheEntry).availableHeight = availableHeight;
            (*newCacheEntry).widthMeasureMode = widthMeasureMode;
            (*newCacheEntry).heightMeasureMode = heightMeasureMode;
            (*newCacheEntry).computedWidth =
                (*layout).measuredDimensions[YGDimensionWidth as libc::c_int as usize];
            (*newCacheEntry).computedHeight =
                (*layout).measuredDimensions[YGDimensionHeight as libc::c_int as usize];
        };
    };
    if performLayout {
        (*node).layout.dimensions[YGDimensionWidth as libc::c_int as usize] =
            (*node).layout.measuredDimensions[YGDimensionWidth as libc::c_int as usize];
        (*node).layout.dimensions[YGDimensionHeight as libc::c_int as usize] =
            (*node).layout.measuredDimensions[YGDimensionHeight as libc::c_int as usize];
        (*node).hasNewLayout = 0 != 1i32;
        (*node).isDirty = 0 != 0i32;
    };
    gDepth = gDepth.wrapping_sub(1);
    (*layout).generationCount = gCurrentGenerationCount;
    return 0 != needToVisitNode as libc::c_int || cachedResults.is_null();
}
#[no_mangle]
pub static mut gCurrentGenerationCount: uint32_t = unsafe { 0i32 as uint32_t };
#[no_mangle]
pub static mut gDepth: uint32_t = unsafe { 0i32 as uint32_t };
unsafe extern "C" fn YGMeasureModeName(
    mode: YGMeasureMode,
    performLayout: bool,
) -> *const libc::c_char {
    let mut kMeasureModeNames: [*const libc::c_char; 3] = [
        b"UNDEFINED\x00" as *const u8 as *const libc::c_char,
        b"EXACTLY\x00" as *const u8 as *const libc::c_char,
        b"AT_MOST\x00" as *const u8 as *const libc::c_char,
    ];
    let mut kLayoutModeNames: [*const libc::c_char; 3] = [
        b"LAY_UNDEFINED\x00" as *const u8 as *const libc::c_char,
        b"LAY_EXACTLY\x00" as *const u8 as *const libc::c_char,
        b"LAY_AT_MOST\x00" as *const u8 as *const libc::c_char,
    ];
    if mode as libc::c_uint >= 3i32 as libc::c_uint {
        return b"\x00" as *const u8 as *const libc::c_char;
    };
    return if 0 != performLayout as libc::c_int {
        kLayoutModeNames[mode as usize]
    } else {
        kMeasureModeNames[mode as usize]
    };
}
unsafe extern "C" fn YGSpacer(level: libc::c_ulong) -> *const libc::c_char {
    let spacerLen: size_t = strlen(spacer);
    if level > spacerLen {
        return &*spacer.offset(0isize) as *const libc::c_char;
    } else {
        return &*spacer.offset(spacerLen.wrapping_sub(level) as isize) as *const libc::c_char;
    };
}
static mut spacer: *const libc::c_char = unsafe {
    b"                                                            \x00" as *const u8
        as *const libc::c_char
};
unsafe extern "C" fn YGNodeResolveDirection(
    node: YGNodeRef,
    parentDirection: YGDirection_0,
) -> YGDirection_0 {
    if (*node).style.direction as libc::c_uint == YGDirectionInherit as libc::c_int as libc::c_uint
    {
        return (if parentDirection as libc::c_uint
            > YGDirectionInherit as libc::c_int as libc::c_uint
        {
            parentDirection as libc::c_uint
        } else {
            YGDirectionLTR as libc::c_int as libc::c_uint
        }) as YGDirection_0;
    } else {
        return (*node).style.direction;
    };
}
unsafe extern "C" fn YGNodeSetChildTrailingPosition(
    node: YGNodeRef,
    child: YGNodeRef,
    axis: YGFlexDirection_0,
) -> () {
    let size: libc::c_float = (*child).layout.measuredDimensions[dim[axis as usize] as usize];
    (*child).layout.position[trailing[axis as usize] as usize] = (*node).layout.measuredDimensions
        [dim[axis as usize] as usize]
        - size
        - (*child).layout.position[pos[axis as usize] as usize];
}
static mut pos: [YGEdge; 4] = unsafe { [YGEdgeTop, YGEdgeBottom, YGEdgeLeft, YGEdgeRight] };
static mut dim: [YGDimension; 4] = unsafe {
    [
        YGDimensionHeight,
        YGDimensionHeight,
        YGDimensionWidth,
        YGDimensionWidth,
    ]
};
unsafe extern "C" fn YGNodePaddingAndBorderForAxis(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    return YGNodeLeadingPaddingAndBorder(node, axis, widthSize)
        + YGNodeTrailingPaddingAndBorder(node, axis, widthSize);
}
unsafe extern "C" fn YGNodeTrailingPaddingAndBorder(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    return YGNodeTrailingPadding(node, axis, widthSize) + YGNodeTrailingBorder(node, axis);
}
unsafe extern "C" fn YGNodeTrailingBorder(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
) -> libc::c_float {
    if 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*node).style.border[YGEdgeEnd as libc::c_int as usize].unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
        && (*node).style.border[YGEdgeEnd as libc::c_int as usize].value >= 0.0f32
    {
        return (*node).style.border[YGEdgeEnd as libc::c_int as usize].value;
    };
    return fmaxf(
        (*YGComputedEdgeValue(
            (*node).style.border.as_mut_ptr() as *const YGValue,
            trailing[axis as usize],
            &mut YGValueZero as *mut YGValue,
        )).value,
        0.0f32,
    );
}
unsafe extern "C" fn YGNodeTrailingPadding(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    if 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*node).style.padding[YGEdgeEnd as libc::c_int as usize].unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
        && YGResolveValue(
            &mut (*node).style.padding[YGEdgeEnd as libc::c_int as usize] as *mut YGValue,
            widthSize,
        ) >= 0.0f32
    {
        return YGResolveValue(
            &mut (*node).style.padding[YGEdgeEnd as libc::c_int as usize] as *mut YGValue,
            widthSize,
        );
    };
    return fmaxf(
        YGResolveValue(
            YGComputedEdgeValue(
                (*node).style.padding.as_mut_ptr() as *const YGValue,
                trailing[axis as usize],
                &mut YGValueZero as *mut YGValue,
            ),
            widthSize,
        ),
        0.0f32,
    );
}
unsafe extern "C" fn YGNodeLeadingPaddingAndBorder(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    return YGNodeLeadingPadding(node, axis, widthSize) + YGNodeLeadingBorder(node, axis);
}
unsafe extern "C" fn YGNodeLeadingBorder(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
) -> libc::c_float {
    if 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*node).style.border[YGEdgeStart as libc::c_int as usize].unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
        && (*node).style.border[YGEdgeStart as libc::c_int as usize].value >= 0.0f32
    {
        return (*node).style.border[YGEdgeStart as libc::c_int as usize].value;
    };
    return fmaxf(
        (*YGComputedEdgeValue(
            (*node).style.border.as_mut_ptr() as *const YGValue,
            leading[axis as usize],
            &mut YGValueZero as *mut YGValue,
        )).value,
        0.0f32,
    );
}
unsafe extern "C" fn YGNodeLeadingPadding(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    if 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*node).style.padding[YGEdgeStart as libc::c_int as usize].unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
        && YGResolveValue(
            &mut (*node).style.padding[YGEdgeStart as libc::c_int as usize] as *mut YGValue,
            widthSize,
        ) >= 0.0f32
    {
        return YGResolveValue(
            &mut (*node).style.padding[YGEdgeStart as libc::c_int as usize] as *mut YGValue,
            widthSize,
        );
    };
    return fmaxf(
        YGResolveValue(
            YGComputedEdgeValue(
                (*node).style.padding.as_mut_ptr() as *const YGValue,
                leading[axis as usize],
                &mut YGValueZero as *mut YGValue,
            ),
            widthSize,
        ),
        0.0f32,
    );
}
unsafe extern "C" fn YGNodeMarginForAxis(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    return YGNodeLeadingMargin(node, axis, widthSize) + YGNodeTrailingMargin(node, axis, widthSize);
}
unsafe extern "C" fn YGNodeAbsoluteLayoutChild(
    node: YGNodeRef,
    child: YGNodeRef,
    width: libc::c_float,
    widthMode: YGMeasureMode,
    height: libc::c_float,
    direction: YGDirection_0,
    config: YGConfigRef,
) -> () {
    let mainAxis: YGFlexDirection_0 =
        YGResolveFlexDirection((*node).style.flexDirection, direction);
    let crossAxis: YGFlexDirection_0 = YGFlexDirectionCross(mainAxis, direction);
    let isMainAxisRow: bool = YGFlexDirectionIsRow(mainAxis);
    let mut childWidth: libc::c_float = ::std::f32::NAN;
    let mut childHeight: libc::c_float = ::std::f32::NAN;
    let mut childWidthMeasureMode: YGMeasureMode = YGMeasureModeUndefined;
    let mut childHeightMeasureMode: YGMeasureMode = YGMeasureModeUndefined;
    let marginRow: libc::c_float = YGNodeMarginForAxis(child, YGFlexDirectionRow, width);
    let marginColumn: libc::c_float = YGNodeMarginForAxis(child, YGFlexDirectionColumn, width);
    if YGNodeIsStyleDimDefined(child, YGFlexDirectionRow, width) {
        childWidth = YGResolveValue(
            (*child).resolvedDimensions[YGDimensionWidth as libc::c_int as usize],
            width,
        ) + marginRow;
    } else {
        if 0 != YGNodeIsLeadingPosDefined(child, YGFlexDirectionRow) as libc::c_int
            && 0 != YGNodeIsTrailingPosDefined(child, YGFlexDirectionRow) as libc::c_int
        {
            childWidth = (*node).layout.measuredDimensions[YGDimensionWidth as libc::c_int as usize]
                - (YGNodeLeadingBorder(node, YGFlexDirectionRow)
                    + YGNodeTrailingBorder(node, YGFlexDirectionRow))
                - (YGNodeLeadingPosition(child, YGFlexDirectionRow, width)
                    + YGNodeTrailingPosition(child, YGFlexDirectionRow, width));
            childWidth = YGNodeBoundAxis(child, YGFlexDirectionRow, childWidth, width, width);
        };
    };
    if YGNodeIsStyleDimDefined(child, YGFlexDirectionColumn, height) {
        childHeight = YGResolveValue(
            (*child).resolvedDimensions[YGDimensionHeight as libc::c_int as usize],
            height,
        ) + marginColumn;
    } else {
        if 0 != YGNodeIsLeadingPosDefined(child, YGFlexDirectionColumn) as libc::c_int
            && 0 != YGNodeIsTrailingPosDefined(child, YGFlexDirectionColumn) as libc::c_int
        {
            childHeight = (*node).layout.measuredDimensions
                [YGDimensionHeight as libc::c_int as usize]
                - (YGNodeLeadingBorder(node, YGFlexDirectionColumn)
                    + YGNodeTrailingBorder(node, YGFlexDirectionColumn))
                - (YGNodeLeadingPosition(child, YGFlexDirectionColumn, height)
                    + YGNodeTrailingPosition(child, YGFlexDirectionColumn, height));
            childHeight = YGNodeBoundAxis(child, YGFlexDirectionColumn, childHeight, height, width);
        };
    };
    if 0 != YGFloatIsUndefined(childWidth) as libc::c_int
        ^ YGFloatIsUndefined(childHeight) as libc::c_int
    {
        if !YGFloatIsUndefined((*child).style.aspectRatio) {
            if YGFloatIsUndefined(childWidth) {
                childWidth = marginRow + (childHeight - marginColumn) * (*child).style.aspectRatio;
            } else {
                if YGFloatIsUndefined(childHeight) {
                    childHeight =
                        marginColumn + (childWidth - marginRow) / (*child).style.aspectRatio;
                };
            };
        };
    };
    if 0 != YGFloatIsUndefined(childWidth) as libc::c_int
        || 0 != YGFloatIsUndefined(childHeight) as libc::c_int
    {
        childWidthMeasureMode = (if 0 != YGFloatIsUndefined(childWidth) as libc::c_int {
            YGMeasureModeUndefined as libc::c_int
        } else {
            YGMeasureModeExactly as libc::c_int
        }) as YGMeasureMode;
        childHeightMeasureMode = (if 0 != YGFloatIsUndefined(childHeight) as libc::c_int {
            YGMeasureModeUndefined as libc::c_int
        } else {
            YGMeasureModeExactly as libc::c_int
        }) as YGMeasureMode;
        if !isMainAxisRow
            && 0 != YGFloatIsUndefined(childWidth) as libc::c_int
            && widthMode as libc::c_uint != YGMeasureModeUndefined as libc::c_int as libc::c_uint
            && width > 0i32 as libc::c_float
        {
            childWidth = width;
            childWidthMeasureMode = YGMeasureModeAtMost;
        };
        YGLayoutNodeInternal(
            child,
            childWidth,
            childHeight,
            direction,
            childWidthMeasureMode,
            childHeightMeasureMode,
            childWidth,
            childHeight,
            0 != 0i32,
            b"abs-measure\x00" as *const u8 as *const libc::c_char,
            config,
        );
        childWidth = (*child).layout.measuredDimensions[YGDimensionWidth as libc::c_int as usize]
            + YGNodeMarginForAxis(child, YGFlexDirectionRow, width);
        childHeight = (*child).layout.measuredDimensions[YGDimensionHeight as libc::c_int as usize]
            + YGNodeMarginForAxis(child, YGFlexDirectionColumn, width);
    };
    YGLayoutNodeInternal(
        child,
        childWidth,
        childHeight,
        direction,
        YGMeasureModeExactly,
        YGMeasureModeExactly,
        childWidth,
        childHeight,
        0 != 1i32,
        b"abs-layout\x00" as *const u8 as *const libc::c_char,
        config,
    );
    if 0 != YGNodeIsTrailingPosDefined(child, mainAxis) as libc::c_int
        && !YGNodeIsLeadingPosDefined(child, mainAxis)
    {
        (*child).layout.position[leading[mainAxis as usize] as usize] =
            (*node).layout.measuredDimensions[dim[mainAxis as usize] as usize]
                - (*child).layout.measuredDimensions[dim[mainAxis as usize] as usize]
                - YGNodeTrailingBorder(node, mainAxis)
                - YGNodeTrailingMargin(child, mainAxis, width)
                - YGNodeTrailingPosition(
                    child,
                    mainAxis,
                    if 0 != isMainAxisRow as libc::c_int {
                        width
                    } else {
                        height
                    },
                );
    } else {
        if !YGNodeIsLeadingPosDefined(child, mainAxis)
            && (*node).style.justifyContent as libc::c_uint
                == YGJustifyCenter as libc::c_int as libc::c_uint
        {
            (*child).layout.position[leading[mainAxis as usize] as usize] =
                ((*node).layout.measuredDimensions[dim[mainAxis as usize] as usize]
                    - (*child).layout.measuredDimensions[dim[mainAxis as usize] as usize])
                    / 2.0f32;
        } else {
            if !YGNodeIsLeadingPosDefined(child, mainAxis)
                && (*node).style.justifyContent as libc::c_uint
                    == YGJustifyFlexEnd as libc::c_int as libc::c_uint
            {
                (*child).layout.position[leading[mainAxis as usize] as usize] =
                    (*node).layout.measuredDimensions[dim[mainAxis as usize] as usize]
                        - (*child).layout.measuredDimensions[dim[mainAxis as usize] as usize];
            };
        };
    };
    if 0 != YGNodeIsTrailingPosDefined(child, crossAxis) as libc::c_int
        && !YGNodeIsLeadingPosDefined(child, crossAxis)
    {
        (*child).layout.position[leading[crossAxis as usize] as usize] =
            (*node).layout.measuredDimensions[dim[crossAxis as usize] as usize]
                - (*child).layout.measuredDimensions[dim[crossAxis as usize] as usize]
                - YGNodeTrailingBorder(node, crossAxis)
                - YGNodeTrailingMargin(child, crossAxis, width)
                - YGNodeTrailingPosition(
                    child,
                    crossAxis,
                    if 0 != isMainAxisRow as libc::c_int {
                        height
                    } else {
                        width
                    },
                );
    } else {
        if !YGNodeIsLeadingPosDefined(child, crossAxis)
            && YGNodeAlignItem(node, child) as libc::c_uint
                == YGAlignCenter as libc::c_int as libc::c_uint
        {
            (*child).layout.position[leading[crossAxis as usize] as usize] =
                ((*node).layout.measuredDimensions[dim[crossAxis as usize] as usize]
                    - (*child).layout.measuredDimensions[dim[crossAxis as usize] as usize])
                    / 2.0f32;
        } else {
            if !YGNodeIsLeadingPosDefined(child, crossAxis)
                && 0 != (YGNodeAlignItem(node, child) as libc::c_uint
                    == YGAlignFlexEnd as libc::c_int as libc::c_uint)
                    as libc::c_int
                    ^ ((*node).style.flexWrap as libc::c_uint
                        == YGWrapWrapReverse as libc::c_int as libc::c_uint)
                        as libc::c_int
            {
                (*child).layout.position[leading[crossAxis as usize] as usize] =
                    (*node).layout.measuredDimensions[dim[crossAxis as usize] as usize]
                        - (*child).layout.measuredDimensions[dim[crossAxis as usize] as usize];
            };
        };
    };
}
unsafe extern "C" fn YGNodeAlignItem(node: YGNodeRef, child: YGNodeRef) -> YGAlign_0 {
    let align: YGAlign_0 = (if (*child).style.alignSelf as libc::c_uint
        == YGAlignAuto as libc::c_int as libc::c_uint
    {
        (*node).style.alignItems as libc::c_uint
    } else {
        (*child).style.alignSelf as libc::c_uint
    }) as YGAlign_0;
    if align as libc::c_uint == YGAlignBaseline as libc::c_int as libc::c_uint
        && 0 != YGFlexDirectionIsColumn((*node).style.flexDirection) as libc::c_int
    {
        return YGAlignFlexStart;
    };
    return align;
}
unsafe extern "C" fn YGNodeIsTrailingPosDefined(node: YGNodeRef, axis: YGFlexDirection_0) -> bool {
    return 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            YGEdgeEnd,
            &YGValueUndefined as *const YGValue,
        )).unit as libc::c_uint != YGUnitUndefined as libc::c_int as libc::c_uint
        || (*YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            trailing[axis as usize],
            &YGValueUndefined as *const YGValue,
        )).unit as libc::c_uint != YGUnitUndefined as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn YGNodeBoundAxis(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    value: libc::c_float,
    axisSize: libc::c_float,
    widthSize: libc::c_float,
) -> libc::c_float {
    return fmaxf(
        YGNodeBoundAxisWithinMinAndMax(node, axis, value, axisSize),
        YGNodePaddingAndBorderForAxis(node, axis, widthSize),
    );
}
unsafe extern "C" fn YGNodeBoundAxisWithinMinAndMax(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    value: libc::c_float,
    axisSize: libc::c_float,
) -> libc::c_float {
    let mut min: libc::c_float = ::std::f32::NAN;
    let mut max: libc::c_float = ::std::f32::NAN;
    if YGFlexDirectionIsColumn(axis) {
        min = YGResolveValue(
            &mut (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize]
                as *mut YGValue,
            axisSize,
        );
        max = YGResolveValue(
            &mut (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize]
                as *mut YGValue,
            axisSize,
        );
    } else {
        if YGFlexDirectionIsRow(axis) {
            min = YGResolveValue(
                &mut (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize]
                    as *mut YGValue,
                axisSize,
            );
            max = YGResolveValue(
                &mut (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize]
                    as *mut YGValue,
                axisSize,
            );
        };
    };
    let mut boundValue: libc::c_float = value;
    if !YGFloatIsUndefined(max) && max >= 0.0f32 && boundValue > max {
        boundValue = max;
    };
    if !YGFloatIsUndefined(min) && min >= 0.0f32 && boundValue < min {
        boundValue = min;
    };
    return boundValue;
}
unsafe extern "C" fn YGNodeIsStyleDimDefined(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    parentSize: libc::c_float,
) -> bool {
    return !((*(*node).resolvedDimensions[dim[axis as usize] as usize]).unit as libc::c_uint
        == YGUnitAuto as libc::c_int as libc::c_uint
        || (*(*node).resolvedDimensions[dim[axis as usize] as usize]).unit as libc::c_uint
            == YGUnitUndefined as libc::c_int as libc::c_uint
        || (*(*node).resolvedDimensions[dim[axis as usize] as usize]).unit as libc::c_uint
            == YGUnitPoint as libc::c_int as libc::c_uint
            && (*(*node).resolvedDimensions[dim[axis as usize] as usize]).value < 0.0f32
        || (*(*node).resolvedDimensions[dim[axis as usize] as usize]).unit as libc::c_uint
            == YGUnitPercent as libc::c_int as libc::c_uint
            && ((*(*node).resolvedDimensions[dim[axis as usize] as usize]).value < 0.0f32
                || 0 != YGFloatIsUndefined(parentSize) as libc::c_int));
}
unsafe extern "C" fn YGBaseline(node: YGNodeRef) -> libc::c_float {
    if (*node).baseline.is_some() {
        let baseline: libc::c_float = (*node).baseline.expect("non-null function pointer")(
            node,
            (*node).layout.measuredDimensions[YGDimensionWidth as libc::c_int as usize],
            (*node).layout.measuredDimensions[YGDimensionHeight as libc::c_int as usize],
        );
        YGAssertWithNode(
            node,
            !YGFloatIsUndefined(baseline),
            b"Expect custom baseline function to not return NaN\x00" as *const u8
                as *const libc::c_char,
        );
        return baseline;
    };
    let mut baselineChild: YGNodeRef = 0 as YGNodeRef;
    let childCount: uint32_t = YGNodeGetChildCount(node);
    {
        let mut i: uint32_t = 0i32 as uint32_t;
        'loop5: while i < childCount {
            'body3: loop {
                {
                    let child: YGNodeRef = YGNodeGetChild(node, i);
                    if (*child).lineIndex > 0i32 as libc::c_uint {
                        break 'loop5;
                    };
                    if (*child).style.positionType as libc::c_uint
                        == YGPositionTypeAbsolute as libc::c_int as libc::c_uint
                    {
                        break 'body3;
                    };
                    if YGNodeAlignItem(node, child) as libc::c_uint
                        == YGAlignBaseline as libc::c_int as libc::c_uint
                    {
                        baselineChild = child;
                        break 'loop5;
                    };
                    if baselineChild.is_null() {
                        baselineChild = child;
                    };
                }
                break 'body3;
            }
            i = i.wrapping_add(1);
        }
    }
    if baselineChild.is_null() {
        return (*node).layout.measuredDimensions[YGDimensionHeight as libc::c_int as usize];
    };
    let baseline: libc::c_float = YGBaseline(baselineChild);
    return baseline + (*baselineChild).layout.position[YGEdgeTop as libc::c_int as usize];
}
unsafe extern "C" fn YGNodeIsLayoutDimDefined(node: YGNodeRef, axis: YGFlexDirection_0) -> bool {
    let value: libc::c_float = (*node).layout.measuredDimensions[dim[axis as usize] as usize];
    return !YGFloatIsUndefined(value) && value >= 0.0f32;
}
unsafe extern "C" fn YGIsBaselineLayout(node: YGNodeRef) -> bool {
    if YGFlexDirectionIsColumn((*node).style.flexDirection) {
        return 0 != 0i32;
    };
    if (*node).style.alignItems as libc::c_uint == YGAlignBaseline as libc::c_int as libc::c_uint {
        return 0 != 1i32;
    };
    let childCount: uint32_t = YGNodeGetChildCount(node);
    {
        let mut i: uint32_t = 0i32 as uint32_t;
        while i < childCount {
            {
                let child: YGNodeRef = YGNodeGetChild(node, i);
                if (*child).style.positionType as libc::c_uint
                    == YGPositionTypeRelative as libc::c_int as libc::c_uint
                    && (*child).style.alignSelf as libc::c_uint
                        == YGAlignBaseline as libc::c_int as libc::c_uint
                {
                    return 0 != 1i32;
                };
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 != 0i32;
}
unsafe extern "C" fn YGNodeDimWithMargin(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
    widthSize: libc::c_float,
) -> libc::c_float {
    return (*node).layout.measuredDimensions[dim[axis as usize] as usize]
        + YGNodeLeadingMargin(node, axis, widthSize)
        + YGNodeTrailingMargin(node, axis, widthSize);
}
unsafe extern "C" fn YGMarginLeadingValue(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
) -> *mut YGValue {
    if 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*node).style.margin[YGEdgeStart as libc::c_int as usize].unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return &mut (*node).style.margin[YGEdgeStart as libc::c_int as usize] as *mut YGValue;
    } else {
        return &mut (*node).style.margin[leading[axis as usize] as usize] as *mut YGValue;
    };
}
unsafe extern "C" fn YGMarginTrailingValue(
    node: YGNodeRef,
    axis: YGFlexDirection_0,
) -> *mut YGValue {
    if 0 != YGFlexDirectionIsRow(axis) as libc::c_int
        && (*node).style.margin[YGEdgeEnd as libc::c_int as usize].unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return &mut (*node).style.margin[YGEdgeEnd as libc::c_int as usize] as *mut YGValue;
    } else {
        return &mut (*node).style.margin[trailing[axis as usize] as usize] as *mut YGValue;
    };
}
unsafe extern "C" fn YGResolveFlexGrow(node: YGNodeRef) -> libc::c_float {
    if (*node).parent.is_null() {
        return 0.0f64 as libc::c_float;
    };
    if !YGFloatIsUndefined((*node).style.flexGrow) {
        return (*node).style.flexGrow;
    };
    if !YGFloatIsUndefined((*node).style.flex) && (*node).style.flex > 0.0f32 {
        return (*node).style.flex;
    };
    return kDefaultFlexGrow;
}
static mut kDefaultFlexGrow: libc::c_float = unsafe { 0.0f32 };
unsafe extern "C" fn YGNodeResolveFlexShrink(node: YGNodeRef) -> libc::c_float {
    if (*node).parent.is_null() {
        return 0.0f64 as libc::c_float;
    };
    if !YGFloatIsUndefined((*node).style.flexShrink) {
        return (*node).style.flexShrink;
    };
    if !(*(*node).config).useWebDefaults
        && !YGFloatIsUndefined((*node).style.flex)
        && (*node).style.flex < 0.0f32
    {
        return -(*node).style.flex;
    };
    return if 0 != (*(*node).config).useWebDefaults as libc::c_int {
        kWebDefaultFlexShrink
    } else {
        kDefaultFlexShrink
    };
}
static mut kDefaultFlexShrink: libc::c_float = unsafe { 0.0f32 };
static mut kWebDefaultFlexShrink: libc::c_float = unsafe { 1.0f32 };
unsafe extern "C" fn YGNodeIsFlex(node: YGNodeRef) -> bool {
    return (*node).style.positionType as libc::c_uint
        == YGPositionTypeRelative as libc::c_int as libc::c_uint
        && (YGResolveFlexGrow(node) != 0i32 as libc::c_float
            || YGNodeResolveFlexShrink(node) != 0i32 as libc::c_float);
}
unsafe extern "C" fn YGNodeComputeFlexBasisForChild(
    node: YGNodeRef,
    child: YGNodeRef,
    width: libc::c_float,
    widthMode: YGMeasureMode,
    height: libc::c_float,
    parentWidth: libc::c_float,
    parentHeight: libc::c_float,
    heightMode: YGMeasureMode,
    direction: YGDirection_0,
    config: YGConfigRef,
) -> () {
    let mainAxis: YGFlexDirection_0 =
        YGResolveFlexDirection((*node).style.flexDirection, direction);
    let isMainAxisRow: bool = YGFlexDirectionIsRow(mainAxis);
    let mainAxisSize: libc::c_float = if 0 != isMainAxisRow as libc::c_int {
        width
    } else {
        height
    };
    let mainAxisParentSize: libc::c_float = if 0 != isMainAxisRow as libc::c_int {
        parentWidth
    } else {
        parentHeight
    };
    let mut childWidth: libc::c_float = 0.;
    let mut childHeight: libc::c_float = 0.;
    let mut childWidthMeasureMode: YGMeasureMode = YGMeasureModeUndefined;
    let mut childHeightMeasureMode: YGMeasureMode = YGMeasureModeUndefined;
    let resolvedFlexBasis: libc::c_float =
        YGResolveValue(YGNodeResolveFlexBasisPtr(child), mainAxisParentSize);
    let isRowStyleDimDefined: bool =
        YGNodeIsStyleDimDefined(child, YGFlexDirectionRow, parentWidth);
    let isColumnStyleDimDefined: bool =
        YGNodeIsStyleDimDefined(child, YGFlexDirectionColumn, parentHeight);
    if !YGFloatIsUndefined(resolvedFlexBasis) && !YGFloatIsUndefined(mainAxisSize) {
        if 0 != YGFloatIsUndefined((*child).layout.computedFlexBasis) as libc::c_int
            || 0 != YGConfigIsExperimentalFeatureEnabled(
                (*child).config,
                YGExperimentalFeatureWebFlexBasis,
            ) as libc::c_int
                && (*child).layout.computedFlexBasisGeneration != gCurrentGenerationCount
        {
            (*child).layout.computedFlexBasis = fmaxf(
                resolvedFlexBasis,
                YGNodePaddingAndBorderForAxis(child, mainAxis, parentWidth),
            );
        };
    } else {
        if 0 != isMainAxisRow as libc::c_int && 0 != isRowStyleDimDefined as libc::c_int {
            (*child).layout.computedFlexBasis = fmaxf(
                YGResolveValue(
                    (*child).resolvedDimensions[YGDimensionWidth as libc::c_int as usize],
                    parentWidth,
                ),
                YGNodePaddingAndBorderForAxis(child, YGFlexDirectionRow, parentWidth),
            );
        } else {
            if !isMainAxisRow && 0 != isColumnStyleDimDefined as libc::c_int {
                (*child).layout.computedFlexBasis = fmaxf(
                    YGResolveValue(
                        (*child).resolvedDimensions[YGDimensionHeight as libc::c_int as usize],
                        parentHeight,
                    ),
                    YGNodePaddingAndBorderForAxis(child, YGFlexDirectionColumn, parentWidth),
                );
            } else {
                childWidth = ::std::f32::NAN;
                childHeight = ::std::f32::NAN;
                childWidthMeasureMode = YGMeasureModeUndefined;
                childHeightMeasureMode = YGMeasureModeUndefined;
                let marginRow: libc::c_float =
                    YGNodeMarginForAxis(child, YGFlexDirectionRow, parentWidth);
                let marginColumn: libc::c_float =
                    YGNodeMarginForAxis(child, YGFlexDirectionColumn, parentWidth);
                if isRowStyleDimDefined {
                    childWidth = YGResolveValue(
                        (*child).resolvedDimensions[YGDimensionWidth as libc::c_int as usize],
                        parentWidth,
                    ) + marginRow;
                    childWidthMeasureMode = YGMeasureModeExactly;
                };
                if isColumnStyleDimDefined {
                    childHeight = YGResolveValue(
                        (*child).resolvedDimensions[YGDimensionHeight as libc::c_int as usize],
                        parentHeight,
                    ) + marginColumn;
                    childHeightMeasureMode = YGMeasureModeExactly;
                };
                if !isMainAxisRow
                    && (*node).style.overflow as libc::c_uint
                        == YGOverflowScroll as libc::c_int as libc::c_uint
                    || (*node).style.overflow as libc::c_uint
                        != YGOverflowScroll as libc::c_int as libc::c_uint
                {
                    if 0 != YGFloatIsUndefined(childWidth) as libc::c_int
                        && !YGFloatIsUndefined(width)
                    {
                        childWidth = width;
                        childWidthMeasureMode = YGMeasureModeAtMost;
                    };
                };
                if 0 != isMainAxisRow as libc::c_int
                    && (*node).style.overflow as libc::c_uint
                        == YGOverflowScroll as libc::c_int as libc::c_uint
                    || (*node).style.overflow as libc::c_uint
                        != YGOverflowScroll as libc::c_int as libc::c_uint
                {
                    if 0 != YGFloatIsUndefined(childHeight) as libc::c_int
                        && !YGFloatIsUndefined(height)
                    {
                        childHeight = height;
                        childHeightMeasureMode = YGMeasureModeAtMost;
                    };
                };
                if !YGFloatIsUndefined((*child).style.aspectRatio) {
                    if !isMainAxisRow
                        && childWidthMeasureMode as libc::c_uint
                            == YGMeasureModeExactly as libc::c_int as libc::c_uint
                    {
                        childHeight = (childWidth - marginRow) / (*child).style.aspectRatio;
                        childHeightMeasureMode = YGMeasureModeExactly;
                    } else {
                        if 0 != isMainAxisRow as libc::c_int
                            && childHeightMeasureMode as libc::c_uint
                                == YGMeasureModeExactly as libc::c_int as libc::c_uint
                        {
                            childWidth = (childHeight - marginColumn) * (*child).style.aspectRatio;
                            childWidthMeasureMode = YGMeasureModeExactly;
                        };
                    };
                };
                let hasExactWidth: bool = !YGFloatIsUndefined(width)
                    && widthMode as libc::c_uint
                        == YGMeasureModeExactly as libc::c_int as libc::c_uint;
                let childWidthStretch: bool = YGNodeAlignItem(node, child) as libc::c_uint
                    == YGAlignStretch as libc::c_int as libc::c_uint
                    && childWidthMeasureMode as libc::c_uint
                        != YGMeasureModeExactly as libc::c_int as libc::c_uint;
                if !isMainAxisRow
                    && !isRowStyleDimDefined
                    && 0 != hasExactWidth as libc::c_int
                    && 0 != childWidthStretch as libc::c_int
                {
                    childWidth = width;
                    childWidthMeasureMode = YGMeasureModeExactly;
                    if !YGFloatIsUndefined((*child).style.aspectRatio) {
                        childHeight = (childWidth - marginRow) / (*child).style.aspectRatio;
                        childHeightMeasureMode = YGMeasureModeExactly;
                    };
                };
                let hasExactHeight: bool = !YGFloatIsUndefined(height)
                    && heightMode as libc::c_uint
                        == YGMeasureModeExactly as libc::c_int as libc::c_uint;
                let childHeightStretch: bool = YGNodeAlignItem(node, child) as libc::c_uint
                    == YGAlignStretch as libc::c_int as libc::c_uint
                    && childHeightMeasureMode as libc::c_uint
                        != YGMeasureModeExactly as libc::c_int as libc::c_uint;
                if 0 != isMainAxisRow as libc::c_int
                    && !isColumnStyleDimDefined
                    && 0 != hasExactHeight as libc::c_int
                    && 0 != childHeightStretch as libc::c_int
                {
                    childHeight = height;
                    childHeightMeasureMode = YGMeasureModeExactly;
                    if !YGFloatIsUndefined((*child).style.aspectRatio) {
                        childWidth = (childHeight - marginColumn) * (*child).style.aspectRatio;
                        childWidthMeasureMode = YGMeasureModeExactly;
                    };
                };
                YGConstrainMaxSizeForMode(
                    child,
                    YGFlexDirectionRow,
                    parentWidth,
                    parentWidth,
                    &mut childWidthMeasureMode as *mut YGMeasureMode,
                    &mut childWidth as *mut libc::c_float,
                );
                YGConstrainMaxSizeForMode(
                    child,
                    YGFlexDirectionColumn,
                    parentHeight,
                    parentWidth,
                    &mut childHeightMeasureMode as *mut YGMeasureMode,
                    &mut childHeight as *mut libc::c_float,
                );
                YGLayoutNodeInternal(
                    child,
                    childWidth,
                    childHeight,
                    direction,
                    childWidthMeasureMode,
                    childHeightMeasureMode,
                    parentWidth,
                    parentHeight,
                    0 != 0i32,
                    b"measure\x00" as *const u8 as *const libc::c_char,
                    config,
                );
                (*child).layout.computedFlexBasis = fmaxf(
                    (*child).layout.measuredDimensions[dim[mainAxis as usize] as usize],
                    YGNodePaddingAndBorderForAxis(child, mainAxis, parentWidth),
                );
            };
        };
    };
    (*child).layout.computedFlexBasisGeneration = gCurrentGenerationCount;
}
unsafe extern "C" fn YGNodeResolveFlexBasisPtr(node: YGNodeRef) -> *const YGValue {
    if (*node).style.flexBasis.unit as libc::c_uint != YGUnitAuto as libc::c_int as libc::c_uint
        && (*node).style.flexBasis.unit as libc::c_uint
            != YGUnitUndefined as libc::c_int as libc::c_uint
    {
        return &mut (*node).style.flexBasis as *mut YGValue;
    };
    if !YGFloatIsUndefined((*node).style.flex) && (*node).style.flex > 0.0f32 {
        return if 0 != (*(*node).config).useWebDefaults as libc::c_int {
            &YGValueAuto as *const YGValue
        } else {
            &mut YGValueZero as *mut YGValue
        };
    };
    return &YGValueAuto as *const YGValue;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigIsExperimentalFeatureEnabled(
    config: YGConfigRef,
    feature: YGExperimentalFeature_0,
) -> bool {
    return (*config).experimentalFeatures[feature as usize];
}
unsafe extern "C" fn YGResolveDimensions(mut node: YGNodeRef) -> () {
    let mut dim_0: YGDimension = YGDimensionWidth;
    while dim_0 as libc::c_uint <= YGDimensionHeight as libc::c_int as libc::c_uint {
        {
            if (*node).style.maxDimensions[dim_0 as usize].unit as libc::c_uint
                != YGUnitUndefined as libc::c_int as libc::c_uint
                && 0 != YGValueEqual(
                    (*node).style.maxDimensions[dim_0 as usize],
                    (*node).style.minDimensions[dim_0 as usize],
                ) as libc::c_int
            {
                (*node).resolvedDimensions[dim_0 as usize] =
                    &mut (*node).style.maxDimensions[dim_0 as usize] as *mut YGValue;
            } else {
                (*node).resolvedDimensions[dim_0 as usize] =
                    &mut (*node).style.dimensions[dim_0 as usize] as *mut YGValue;
            };
        }
        dim_0 += 1;
    }
}
unsafe extern "C" fn YGZeroOutLayoutRecursivly(node: YGNodeRef) -> () {
    memset(
        &mut (*node).layout as *mut YGLayout_0 as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<YGLayout_0>() as libc::c_ulong,
    );
    (*node).hasNewLayout = 0 != 1i32;
    YGCloneChildrenIfNeeded(node);
    let childCount: uint32_t = YGNodeGetChildCount(node);
    {
        let mut i: uint32_t = 0i32 as uint32_t;
        while i < childCount {
            {
                let child: YGNodeRef = YGNodeListGet((*node).children, i);
                YGZeroOutLayoutRecursivly(child);
            }
            i = i.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn YGNodeFixedSizeSetMeasuredDimensions(
    node: YGNodeRef,
    availableWidth: libc::c_float,
    availableHeight: libc::c_float,
    widthMeasureMode: YGMeasureMode,
    heightMeasureMode: YGMeasureMode,
    parentWidth: libc::c_float,
    parentHeight: libc::c_float,
) -> bool {
    if widthMeasureMode as libc::c_uint == YGMeasureModeAtMost as libc::c_int as libc::c_uint
        && availableWidth <= 0.0f32
        || heightMeasureMode as libc::c_uint == YGMeasureModeAtMost as libc::c_int as libc::c_uint
            && availableHeight <= 0.0f32
        || widthMeasureMode as libc::c_uint == YGMeasureModeExactly as libc::c_int as libc::c_uint
            && heightMeasureMode as libc::c_uint
                == YGMeasureModeExactly as libc::c_int as libc::c_uint
    {
        let marginAxisColumn: libc::c_float =
            YGNodeMarginForAxis(node, YGFlexDirectionColumn, parentWidth);
        let marginAxisRow: libc::c_float =
            YGNodeMarginForAxis(node, YGFlexDirectionRow, parentWidth);
        (*node).layout.measuredDimensions[YGDimensionWidth as libc::c_int as usize] =
            YGNodeBoundAxis(
                node,
                YGFlexDirectionRow,
                if 0 != YGFloatIsUndefined(availableWidth) as libc::c_int
                    || widthMeasureMode as libc::c_uint
                        == YGMeasureModeAtMost as libc::c_int as libc::c_uint
                        && availableWidth < 0.0f32
                {
                    0.0f32
                } else {
                    availableWidth - marginAxisRow
                },
                parentWidth,
                parentWidth,
            );
        (*node).layout.measuredDimensions[YGDimensionHeight as libc::c_int as usize] =
            YGNodeBoundAxis(
                node,
                YGFlexDirectionColumn,
                if 0 != YGFloatIsUndefined(availableHeight) as libc::c_int
                    || heightMeasureMode as libc::c_uint
                        == YGMeasureModeAtMost as libc::c_int as libc::c_uint
                        && availableHeight < 0.0f32
                {
                    0.0f32
                } else {
                    availableHeight - marginAxisColumn
                },
                parentHeight,
                parentWidth,
            );
        return 0 != 1i32;
    };
    return 0 != 0i32;
}
unsafe extern "C" fn YGNodeEmptyContainerSetMeasuredDimensions(
    node: YGNodeRef,
    availableWidth: libc::c_float,
    availableHeight: libc::c_float,
    widthMeasureMode: YGMeasureMode,
    heightMeasureMode: YGMeasureMode,
    parentWidth: libc::c_float,
    parentHeight: libc::c_float,
) -> () {
    let paddingAndBorderAxisRow: libc::c_float =
        YGNodePaddingAndBorderForAxis(node, YGFlexDirectionRow, parentWidth);
    let paddingAndBorderAxisColumn: libc::c_float =
        YGNodePaddingAndBorderForAxis(node, YGFlexDirectionColumn, parentWidth);
    let marginAxisRow: libc::c_float = YGNodeMarginForAxis(node, YGFlexDirectionRow, parentWidth);
    let marginAxisColumn: libc::c_float =
        YGNodeMarginForAxis(node, YGFlexDirectionColumn, parentWidth);
    (*node).layout.measuredDimensions[YGDimensionWidth as libc::c_int as usize] = YGNodeBoundAxis(
        node,
        YGFlexDirectionRow,
        if widthMeasureMode as libc::c_uint == YGMeasureModeUndefined as libc::c_int as libc::c_uint
            || widthMeasureMode as libc::c_uint
                == YGMeasureModeAtMost as libc::c_int as libc::c_uint
        {
            paddingAndBorderAxisRow
        } else {
            availableWidth - marginAxisRow
        },
        parentWidth,
        parentWidth,
    );
    (*node).layout.measuredDimensions[YGDimensionHeight as libc::c_int as usize] = YGNodeBoundAxis(
        node,
        YGFlexDirectionColumn,
        if heightMeasureMode as libc::c_uint
            == YGMeasureModeUndefined as libc::c_int as libc::c_uint
            || heightMeasureMode as libc::c_uint
                == YGMeasureModeAtMost as libc::c_int as libc::c_uint
        {
            paddingAndBorderAxisColumn
        } else {
            availableHeight - marginAxisColumn
        },
        parentHeight,
        parentWidth,
    );
}
unsafe extern "C" fn YGNodeWithMeasureFuncSetMeasuredDimensions(
    node: YGNodeRef,
    availableWidth: libc::c_float,
    availableHeight: libc::c_float,
    widthMeasureMode: YGMeasureMode,
    heightMeasureMode: YGMeasureMode,
    parentWidth: libc::c_float,
    parentHeight: libc::c_float,
) -> () {
    YGAssertWithNode(
        node,
        (*node).measure.is_some(),
        b"Expected node to have custom measure function\x00" as *const u8 as *const libc::c_char,
    );
    let paddingAndBorderAxisRow: libc::c_float =
        YGNodePaddingAndBorderForAxis(node, YGFlexDirectionRow, availableWidth);
    let paddingAndBorderAxisColumn: libc::c_float =
        YGNodePaddingAndBorderForAxis(node, YGFlexDirectionColumn, availableWidth);
    let marginAxisRow: libc::c_float =
        YGNodeMarginForAxis(node, YGFlexDirectionRow, availableWidth);
    let marginAxisColumn: libc::c_float =
        YGNodeMarginForAxis(node, YGFlexDirectionColumn, availableWidth);
    let innerWidth: libc::c_float = if 0 != YGFloatIsUndefined(availableWidth) as libc::c_int {
        availableWidth
    } else {
        fmaxf(
            0i32 as libc::c_float,
            availableWidth - marginAxisRow - paddingAndBorderAxisRow,
        )
    };
    let innerHeight: libc::c_float = if 0 != YGFloatIsUndefined(availableHeight) as libc::c_int {
        availableHeight
    } else {
        fmaxf(
            0i32 as libc::c_float,
            availableHeight - marginAxisColumn - paddingAndBorderAxisColumn,
        )
    };
    if widthMeasureMode as libc::c_uint == YGMeasureModeExactly as libc::c_int as libc::c_uint
        && heightMeasureMode as libc::c_uint == YGMeasureModeExactly as libc::c_int as libc::c_uint
    {
        (*node).layout.measuredDimensions[YGDimensionWidth as libc::c_int as usize] =
            YGNodeBoundAxis(
                node,
                YGFlexDirectionRow,
                availableWidth - marginAxisRow,
                parentWidth,
                parentWidth,
            );
        (*node).layout.measuredDimensions[YGDimensionHeight as libc::c_int as usize] =
            YGNodeBoundAxis(
                node,
                YGFlexDirectionColumn,
                availableHeight - marginAxisColumn,
                parentHeight,
                parentWidth,
            );
    } else {
        let measuredSize: YGSize_0 = (*node).measure.expect("non-null function pointer")(
            node,
            innerWidth,
            widthMeasureMode,
            innerHeight,
            heightMeasureMode,
        );
        (*node).layout.measuredDimensions[YGDimensionWidth as libc::c_int as usize] =
            YGNodeBoundAxis(
                node,
                YGFlexDirectionRow,
                if widthMeasureMode as libc::c_uint
                    == YGMeasureModeUndefined as libc::c_int as libc::c_uint
                    || widthMeasureMode as libc::c_uint
                        == YGMeasureModeAtMost as libc::c_int as libc::c_uint
                {
                    measuredSize.width + paddingAndBorderAxisRow
                } else {
                    availableWidth - marginAxisRow
                },
                availableWidth,
                availableWidth,
            );
        (*node).layout.measuredDimensions[YGDimensionHeight as libc::c_int as usize] =
            YGNodeBoundAxis(
                node,
                YGFlexDirectionColumn,
                if heightMeasureMode as libc::c_uint
                    == YGMeasureModeUndefined as libc::c_int as libc::c_uint
                    || heightMeasureMode as libc::c_uint
                        == YGMeasureModeAtMost as libc::c_int as libc::c_uint
                {
                    measuredSize.height + paddingAndBorderAxisColumn
                } else {
                    availableHeight - marginAxisColumn
                },
                availableHeight,
                availableWidth,
            );
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeCanUseCachedMeasurement(
    widthMode: YGMeasureMode,
    width: libc::c_float,
    heightMode: YGMeasureMode,
    height: libc::c_float,
    lastWidthMode: YGMeasureMode,
    lastWidth: libc::c_float,
    lastHeightMode: YGMeasureMode,
    lastHeight: libc::c_float,
    lastComputedWidth: libc::c_float,
    lastComputedHeight: libc::c_float,
    marginRow: libc::c_float,
    marginColumn: libc::c_float,
    config: YGConfigRef,
) -> bool {
    if lastComputedHeight < 0i32 as libc::c_float || lastComputedWidth < 0i32 as libc::c_float {
        return 0 != 0i32;
    };
    let mut useRoundedComparison: bool =
        !config.is_null() && (*config).pointScaleFactor != 0i32 as libc::c_float;
    let effectiveWidth: libc::c_float = if 0 != useRoundedComparison as libc::c_int {
        YGRoundValueToPixelGrid(width, (*config).pointScaleFactor, 0 != 0i32, 0 != 0i32)
    } else {
        width
    };
    let effectiveHeight: libc::c_float = if 0 != useRoundedComparison as libc::c_int {
        YGRoundValueToPixelGrid(height, (*config).pointScaleFactor, 0 != 0i32, 0 != 0i32)
    } else {
        height
    };
    let effectiveLastWidth: libc::c_float = if 0 != useRoundedComparison as libc::c_int {
        YGRoundValueToPixelGrid(lastWidth, (*config).pointScaleFactor, 0 != 0i32, 0 != 0i32)
    } else {
        lastWidth
    };
    let effectiveLastHeight: libc::c_float = if 0 != useRoundedComparison as libc::c_int {
        YGRoundValueToPixelGrid(lastHeight, (*config).pointScaleFactor, 0 != 0i32, 0 != 0i32)
    } else {
        lastHeight
    };
    let hasSameWidthSpec: bool = lastWidthMode as libc::c_uint == widthMode as libc::c_uint
        && 0 != YGFloatsEqual(effectiveLastWidth, effectiveWidth) as libc::c_int;
    let hasSameHeightSpec: bool = lastHeightMode as libc::c_uint == heightMode as libc::c_uint
        && 0 != YGFloatsEqual(effectiveLastHeight, effectiveHeight) as libc::c_int;
    let widthIsCompatible: bool = 0 != hasSameWidthSpec as libc::c_int
        || 0 != YGMeasureModeSizeIsExactAndMatchesOldMeasuredSize(
            widthMode,
            width - marginRow,
            lastComputedWidth,
        ) as libc::c_int
        || 0 != YGMeasureModeOldSizeIsUnspecifiedAndStillFits(
            widthMode,
            width - marginRow,
            lastWidthMode,
            lastComputedWidth,
        ) as libc::c_int
        || 0 != YGMeasureModeNewMeasureSizeIsStricterAndStillValid(
            widthMode,
            width - marginRow,
            lastWidthMode,
            lastWidth,
            lastComputedWidth,
        ) as libc::c_int;
    let heightIsCompatible: bool = 0 != hasSameHeightSpec as libc::c_int
        || 0 != YGMeasureModeSizeIsExactAndMatchesOldMeasuredSize(
            heightMode,
            height - marginColumn,
            lastComputedHeight,
        ) as libc::c_int
        || 0 != YGMeasureModeOldSizeIsUnspecifiedAndStillFits(
            heightMode,
            height - marginColumn,
            lastHeightMode,
            lastComputedHeight,
        ) as libc::c_int
        || 0 != YGMeasureModeNewMeasureSizeIsStricterAndStillValid(
            heightMode,
            height - marginColumn,
            lastHeightMode,
            lastHeight,
            lastComputedHeight,
        ) as libc::c_int;
    return 0 != widthIsCompatible as libc::c_int && 0 != heightIsCompatible as libc::c_int;
}
unsafe extern "C" fn YGMeasureModeNewMeasureSizeIsStricterAndStillValid(
    mut sizeMode: YGMeasureMode,
    mut size: libc::c_float,
    mut lastSizeMode: YGMeasureMode,
    mut lastSize: libc::c_float,
    mut lastComputedSize: libc::c_float,
) -> bool {
    return lastSizeMode as libc::c_uint == YGMeasureModeAtMost as libc::c_int as libc::c_uint
        && sizeMode as libc::c_uint == YGMeasureModeAtMost as libc::c_int as libc::c_uint
        && lastSize > size
        && (lastComputedSize <= size || 0 != YGFloatsEqual(size, lastComputedSize) as libc::c_int);
}
unsafe extern "C" fn YGMeasureModeOldSizeIsUnspecifiedAndStillFits(
    mut sizeMode: YGMeasureMode,
    mut size: libc::c_float,
    mut lastSizeMode: YGMeasureMode,
    mut lastComputedSize: libc::c_float,
) -> bool {
    return sizeMode as libc::c_uint == YGMeasureModeAtMost as libc::c_int as libc::c_uint
        && lastSizeMode as libc::c_uint == YGMeasureModeUndefined as libc::c_int as libc::c_uint
        && (size >= lastComputedSize || 0 != YGFloatsEqual(size, lastComputedSize) as libc::c_int);
}
unsafe extern "C" fn YGMeasureModeSizeIsExactAndMatchesOldMeasuredSize(
    mut sizeMode: YGMeasureMode,
    mut size: libc::c_float,
    mut lastComputedSize: libc::c_float,
) -> bool {
    return sizeMode as libc::c_uint == YGMeasureModeExactly as libc::c_int as libc::c_uint
        && 0 != YGFloatsEqual(size, lastComputedSize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeMarkDirty(node: YGNodeRef) -> () {
    YGAssertWithNode(
        node,
        (*node).measure.is_some(),
        b"Only leaf nodes with custom measure functionsshould manually mark themselves as dirty\x00"
            as *const u8 as *const libc::c_char,
    );
    YGNodeMarkDirtyInternal(node);
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeIsDirty(node: YGNodeRef) -> bool {
    return (*node).isDirty;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeCopyStyle(dstNode: YGNodeRef, srcNode: YGNodeRef) -> () {
    if memcmp(
        &mut (*dstNode).style as *mut YGStyle_0 as *const libc::c_void,
        &mut (*srcNode).style as *mut YGStyle_0 as *const libc::c_void,
        ::std::mem::size_of::<YGStyle_0>() as libc::c_ulong,
    ) != 0i32
    {
        memcpy(
            &mut (*dstNode).style as *mut YGStyle_0 as *mut libc::c_void,
            &mut (*srcNode).style as *mut YGStyle_0 as *const libc::c_void,
            ::std::mem::size_of::<YGStyle_0>() as libc::c_ulong,
        );
        YGNodeMarkDirtyInternal(dstNode);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeSetContext(node: YGNodeRef, mut context: *mut libc::c_void) -> () {
    (*node).context = context;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetContext(node: YGNodeRef) -> *mut libc::c_void {
    return (*node).context;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeSetMeasureFunc(
    node: YGNodeRef,
    mut measureFunc: YGMeasureFunc,
) -> () {
    if measureFunc.is_none() {
        (*node).measure = None;
        (*node).nodeType = YGNodeTypeDefault;
    } else {
        YGAssertWithNode(
            node,
            YGNodeGetChildCount(node) == 0i32 as libc::c_uint,
            b"Cannot set measure function: Nodes with measure functions cannot have children.\x00"
                as *const u8 as *const libc::c_char,
        );
        (*node).measure = measureFunc;
        (*node).nodeType = YGNodeTypeText;
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetMeasureFunc(node: YGNodeRef) -> YGMeasureFunc {
    return (*node).measure;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeSetBaselineFunc(
    node: YGNodeRef,
    mut baselineFunc: YGBaselineFunc,
) -> () {
    (*node).baseline = baselineFunc;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetBaselineFunc(node: YGNodeRef) -> YGBaselineFunc {
    return (*node).baseline;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeSetHasNewLayout(node: YGNodeRef, mut hasNewLayout: bool) -> () {
    (*node).hasNewLayout = hasNewLayout;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetHasNewLayout(node: YGNodeRef) -> bool {
    return (*node).hasNewLayout;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeSetNodeType(node: YGNodeRef, mut nodeType: YGNodeType_0) -> () {
    (*node).nodeType = nodeType;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeGetNodeType(node: YGNodeRef) -> YGNodeType_0 {
    return (*node).nodeType;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetDirection(node: YGNodeRef, direction: YGDirection_0) -> () {
    if (*node).style.direction as libc::c_uint != direction as libc::c_uint {
        (*node).style.direction = direction;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetDirection(node: YGNodeRef) -> YGDirection_0 {
    return (*node).style.direction;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetFlexDirection(
    node: YGNodeRef,
    flexDirection: YGFlexDirection_0,
) -> () {
    if (*node).style.flexDirection as libc::c_uint != flexDirection as libc::c_uint {
        (*node).style.flexDirection = flexDirection;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetFlexDirection(node: YGNodeRef) -> YGFlexDirection_0 {
    return (*node).style.flexDirection;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetJustifyContent(
    node: YGNodeRef,
    justifyContent: YGJustify,
) -> () {
    if (*node).style.justifyContent as libc::c_uint != justifyContent as libc::c_uint {
        (*node).style.justifyContent = justifyContent;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetJustifyContent(node: YGNodeRef) -> YGJustify {
    return (*node).style.justifyContent;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetAlignContent(
    node: YGNodeRef,
    alignContent: YGAlign_0,
) -> () {
    if (*node).style.alignContent as libc::c_uint != alignContent as libc::c_uint {
        (*node).style.alignContent = alignContent;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetAlignContent(node: YGNodeRef) -> YGAlign_0 {
    return (*node).style.alignContent;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetAlignItems(node: YGNodeRef, alignItems: YGAlign_0) -> () {
    if (*node).style.alignItems as libc::c_uint != alignItems as libc::c_uint {
        (*node).style.alignItems = alignItems;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetAlignItems(node: YGNodeRef) -> YGAlign_0 {
    return (*node).style.alignItems;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetAlignSelf(node: YGNodeRef, alignSelf: YGAlign_0) -> () {
    if (*node).style.alignSelf as libc::c_uint != alignSelf as libc::c_uint {
        (*node).style.alignSelf = alignSelf;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetAlignSelf(node: YGNodeRef) -> YGAlign_0 {
    return (*node).style.alignSelf;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetPositionType(
    node: YGNodeRef,
    positionType: YGPositionType,
) -> () {
    if (*node).style.positionType as libc::c_uint != positionType as libc::c_uint {
        (*node).style.positionType = positionType;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetPositionType(node: YGNodeRef) -> YGPositionType {
    return (*node).style.positionType;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetFlexWrap(node: YGNodeRef, flexWrap: YGWrap_0) -> () {
    if (*node).style.flexWrap as libc::c_uint != flexWrap as libc::c_uint {
        (*node).style.flexWrap = flexWrap;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetFlexWrap(node: YGNodeRef) -> YGWrap_0 {
    return (*node).style.flexWrap;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetOverflow(node: YGNodeRef, overflow: YGOverflow_0) -> () {
    if (*node).style.overflow as libc::c_uint != overflow as libc::c_uint {
        (*node).style.overflow = overflow;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetOverflow(node: YGNodeRef) -> YGOverflow_0 {
    return (*node).style.overflow;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetDisplay(node: YGNodeRef, display: YGDisplay) -> () {
    if (*node).style.display as libc::c_uint != display as libc::c_uint {
        (*node).style.display = display;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetDisplay(node: YGNodeRef) -> YGDisplay {
    return (*node).style.display;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetFlex(node: YGNodeRef, flex: libc::c_float) -> () {
    if (*node).style.flex != flex {
        (*node).style.flex = flex;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetFlex(node: YGNodeRef) -> libc::c_float {
    return (*node).style.flex;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetFlexGrow(node: YGNodeRef, flexGrow: libc::c_float) -> () {
    if (*node).style.flexGrow != flexGrow {
        (*node).style.flexGrow = flexGrow;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetFlexGrow(node: YGNodeRef) -> libc::c_float {
    return if 0 != YGFloatIsUndefined((*node).style.flexGrow) as libc::c_int {
        kDefaultFlexGrow
    } else {
        (*node).style.flexGrow
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetFlexShrink(
    node: YGNodeRef,
    flexShrink: libc::c_float,
) -> () {
    if (*node).style.flexShrink != flexShrink {
        (*node).style.flexShrink = flexShrink;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetFlexShrink(node: YGNodeRef) -> libc::c_float {
    return if 0 != YGFloatIsUndefined((*node).style.flexShrink) as libc::c_int {
        if 0 != (*(*node).config).useWebDefaults as libc::c_int {
            kWebDefaultFlexShrink
        } else {
            kDefaultFlexShrink
        }
    } else {
        (*node).style.flexShrink
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetFlexBasis(node: YGNodeRef, flexBasis: libc::c_float) -> () {
    if (*node).style.flexBasis.value != flexBasis
        || (*node).style.flexBasis.unit as libc::c_uint
            != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.flexBasis.value = flexBasis;
        (*node).style.flexBasis.unit = (if 0 != YGFloatIsUndefined(flexBasis) as libc::c_int {
            YGUnitAuto as libc::c_int
        } else {
            YGUnitPoint as libc::c_int
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetFlexBasisPercent(
    node: YGNodeRef,
    flexBasis: libc::c_float,
) -> () {
    if (*node).style.flexBasis.value != flexBasis
        || (*node).style.flexBasis.unit as libc::c_uint
            != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.flexBasis.value = flexBasis;
        (*node).style.flexBasis.unit = (if 0 != YGFloatIsUndefined(flexBasis) as libc::c_int {
            YGUnitAuto as libc::c_int
        } else {
            YGUnitPercent as libc::c_int
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetFlexBasis(node: YGNodeRef) -> YGValue {
    return (*node).style.flexBasis;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetFlexBasisAuto(node: YGNodeRef) -> () {
    if (*node).style.flexBasis.unit as libc::c_uint != YGUnitAuto as libc::c_int as libc::c_uint {
        (*node).style.flexBasis.value = ::std::f32::NAN;
        (*node).style.flexBasis.unit = YGUnitAuto;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetPosition(
    node: YGNodeRef,
    edge: YGEdge,
    position: libc::c_float,
) -> () {
    if (*node).style.position[edge as usize].value != position
        || (*node).style.position[edge as usize].unit as libc::c_uint
            != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.position[edge as usize].value = position;
        (*node).style.position[edge as usize].unit =
            (if 0 != YGFloatIsUndefined(position) as libc::c_int {
                YGUnitUndefined as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetPositionPercent(
    node: YGNodeRef,
    edge: YGEdge,
    position: libc::c_float,
) -> () {
    if (*node).style.position[edge as usize].value != position
        || (*node).style.position[edge as usize].unit as libc::c_uint
            != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.position[edge as usize].value = position;
        (*node).style.position[edge as usize].unit =
            (if 0 != YGFloatIsUndefined(position) as libc::c_int {
                YGUnitUndefined as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetPosition(node: YGNodeRef, edge: YGEdge) -> YGValue {
    return (*node).style.position[edge as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMargin(
    node: YGNodeRef,
    edge: YGEdge,
    margin: libc::c_float,
) -> () {
    if (*node).style.margin[edge as usize].value != margin
        || (*node).style.margin[edge as usize].unit as libc::c_uint
            != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.margin[edge as usize].value = margin;
        (*node).style.margin[edge as usize].unit =
            (if 0 != YGFloatIsUndefined(margin) as libc::c_int {
                YGUnitUndefined as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMarginPercent(
    node: YGNodeRef,
    edge: YGEdge,
    margin: libc::c_float,
) -> () {
    if (*node).style.margin[edge as usize].value != margin
        || (*node).style.margin[edge as usize].unit as libc::c_uint
            != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.margin[edge as usize].value = margin;
        (*node).style.margin[edge as usize].unit =
            (if 0 != YGFloatIsUndefined(margin) as libc::c_int {
                YGUnitUndefined as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetMargin(node: YGNodeRef, edge: YGEdge) -> YGValue {
    return (*node).style.margin[edge as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMarginAuto(node: YGNodeRef, edge: YGEdge) -> () {
    if (*node).style.margin[edge as usize].unit as libc::c_uint
        != YGUnitAuto as libc::c_int as libc::c_uint
    {
        (*node).style.margin[edge as usize].value = ::std::f32::NAN;
        (*node).style.margin[edge as usize].unit = YGUnitAuto;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetPadding(
    node: YGNodeRef,
    edge: YGEdge,
    padding: libc::c_float,
) -> () {
    if (*node).style.padding[edge as usize].value != padding
        || (*node).style.padding[edge as usize].unit as libc::c_uint
            != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.padding[edge as usize].value = padding;
        (*node).style.padding[edge as usize].unit =
            (if 0 != YGFloatIsUndefined(padding) as libc::c_int {
                YGUnitUndefined as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetPaddingPercent(
    node: YGNodeRef,
    edge: YGEdge,
    padding: libc::c_float,
) -> () {
    if (*node).style.padding[edge as usize].value != padding
        || (*node).style.padding[edge as usize].unit as libc::c_uint
            != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.padding[edge as usize].value = padding;
        (*node).style.padding[edge as usize].unit =
            (if 0 != YGFloatIsUndefined(padding) as libc::c_int {
                YGUnitUndefined as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetPadding(node: YGNodeRef, edge: YGEdge) -> YGValue {
    return (*node).style.padding[edge as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetBorder(
    node: YGNodeRef,
    edge: YGEdge,
    border: libc::c_float,
) -> () {
    if (*node).style.border[edge as usize].value != border
        || (*node).style.border[edge as usize].unit as libc::c_uint
            != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.border[edge as usize].value = border;
        (*node).style.border[edge as usize].unit =
            (if 0 != YGFloatIsUndefined(border) as libc::c_int {
                YGUnitUndefined as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetBorder(node: YGNodeRef, edge: YGEdge) -> libc::c_float {
    return (*node).style.border[edge as usize].value;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetWidth(node: YGNodeRef, width: libc::c_float) -> () {
    if (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].value != width
        || (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].unit as libc::c_uint
            != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].value = width;
        (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(width) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetWidthPercent(node: YGNodeRef, width: libc::c_float) -> () {
    if (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].value != width
        || (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].unit as libc::c_uint
            != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].value = width;
        (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(width) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetWidth(node: YGNodeRef) -> YGValue {
    return (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetWidthAuto(node: YGNodeRef) -> () {
    if (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].unit as libc::c_uint
        != YGUnitAuto as libc::c_int as libc::c_uint
    {
        (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].value = ::std::f32::NAN;
        (*node).style.dimensions[YGDimensionWidth as libc::c_int as usize].unit = YGUnitAuto;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetHeight(node: YGNodeRef, height: libc::c_float) -> () {
    if (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].value != height
        || (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].unit as libc::c_uint
            != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].value = height;
        (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(height) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetHeightPercent(node: YGNodeRef, height: libc::c_float) -> () {
    if (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].value != height
        || (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].unit as libc::c_uint
            != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].value = height;
        (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(height) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetHeight(node: YGNodeRef) -> YGValue {
    return (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetHeightAuto(node: YGNodeRef) -> () {
    if (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].unit as libc::c_uint
        != YGUnitAuto as libc::c_int as libc::c_uint
    {
        (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].value = ::std::f32::NAN;
        (*node).style.dimensions[YGDimensionHeight as libc::c_int as usize].unit = YGUnitAuto;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMinWidth(node: YGNodeRef, minWidth: libc::c_float) -> () {
    if (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize].value != minWidth
        || (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize].unit
            as libc::c_uint != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize].value = minWidth;
        (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(minWidth) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMinWidthPercent(
    node: YGNodeRef,
    minWidth: libc::c_float,
) -> () {
    if (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize].value != minWidth
        || (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize].unit
            as libc::c_uint != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize].value = minWidth;
        (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(minWidth) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetMinWidth(node: YGNodeRef) -> YGValue {
    return (*node).style.minDimensions[YGDimensionWidth as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMinHeight(node: YGNodeRef, minHeight: libc::c_float) -> () {
    if (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize].value != minHeight
        || (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize].unit
            as libc::c_uint != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize].value = minHeight;
        (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(minHeight) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMinHeightPercent(
    node: YGNodeRef,
    minHeight: libc::c_float,
) -> () {
    if (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize].value != minHeight
        || (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize].unit
            as libc::c_uint != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize].value = minHeight;
        (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(minHeight) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetMinHeight(node: YGNodeRef) -> YGValue {
    return (*node).style.minDimensions[YGDimensionHeight as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMaxWidth(node: YGNodeRef, maxWidth: libc::c_float) -> () {
    if (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize].value != maxWidth
        || (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize].unit
            as libc::c_uint != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize].value = maxWidth;
        (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(maxWidth) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMaxWidthPercent(
    node: YGNodeRef,
    maxWidth: libc::c_float,
) -> () {
    if (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize].value != maxWidth
        || (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize].unit
            as libc::c_uint != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize].value = maxWidth;
        (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(maxWidth) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetMaxWidth(node: YGNodeRef) -> YGValue {
    return (*node).style.maxDimensions[YGDimensionWidth as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMaxHeight(node: YGNodeRef, maxHeight: libc::c_float) -> () {
    if (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize].value != maxHeight
        || (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize].unit
            as libc::c_uint != YGUnitPoint as libc::c_int as libc::c_uint
    {
        (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize].value = maxHeight;
        (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(maxHeight) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPoint as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetMaxHeightPercent(
    node: YGNodeRef,
    maxHeight: libc::c_float,
) -> () {
    if (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize].value != maxHeight
        || (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize].unit
            as libc::c_uint != YGUnitPercent as libc::c_int as libc::c_uint
    {
        (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize].value = maxHeight;
        (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize].unit =
            (if 0 != YGFloatIsUndefined(maxHeight) as libc::c_int {
                YGUnitAuto as libc::c_int
            } else {
                YGUnitPercent as libc::c_int
            }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetMaxHeight(node: YGNodeRef) -> YGValue {
    return (*node).style.maxDimensions[YGDimensionHeight as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleSetAspectRatio(
    node: YGNodeRef,
    aspectRatio: libc::c_float,
) -> () {
    if (*node).style.aspectRatio != aspectRatio {
        (*node).style.aspectRatio = aspectRatio;
        YGNodeMarkDirtyInternal(node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeStyleGetAspectRatio(node: YGNodeRef) -> libc::c_float {
    return (*node).style.aspectRatio;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetLeft(node: YGNodeRef) -> libc::c_float {
    return (*node).layout.position[YGEdgeLeft as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetTop(node: YGNodeRef) -> libc::c_float {
    return (*node).layout.position[YGEdgeTop as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetRight(node: YGNodeRef) -> libc::c_float {
    return (*node).layout.position[YGEdgeRight as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetBottom(node: YGNodeRef) -> libc::c_float {
    return (*node).layout.position[YGEdgeBottom as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetWidth(node: YGNodeRef) -> libc::c_float {
    return (*node).layout.dimensions[YGDimensionWidth as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetHeight(node: YGNodeRef) -> libc::c_float {
    return (*node).layout.dimensions[YGDimensionHeight as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetDirection(node: YGNodeRef) -> YGDirection_0 {
    return (*node).layout.direction;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetHadOverflow(node: YGNodeRef) -> bool {
    return (*node).layout.hadOverflow;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetMargin(node: YGNodeRef, edge: YGEdge) -> libc::c_float {
    YGAssertWithNode(
        node,
        (edge as libc::c_uint) < YGEdgeEnd as libc::c_int as libc::c_uint,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8
            as *const libc::c_char,
    );
    if edge as libc::c_uint == YGEdgeLeft as libc::c_int as libc::c_uint {
        if (*node).layout.direction as libc::c_uint == YGDirectionRTL as libc::c_int as libc::c_uint
        {
            return (*node).layout.margin[YGEdgeEnd as libc::c_int as usize];
        } else {
            return (*node).layout.margin[YGEdgeStart as libc::c_int as usize];
        };
    };
    if edge as libc::c_uint == YGEdgeRight as libc::c_int as libc::c_uint {
        if (*node).layout.direction as libc::c_uint == YGDirectionRTL as libc::c_int as libc::c_uint
        {
            return (*node).layout.margin[YGEdgeStart as libc::c_int as usize];
        } else {
            return (*node).layout.margin[YGEdgeEnd as libc::c_int as usize];
        };
    };
    return (*node).layout.margin[edge as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetBorder(node: YGNodeRef, edge: YGEdge) -> libc::c_float {
    YGAssertWithNode(
        node,
        (edge as libc::c_uint) < YGEdgeEnd as libc::c_int as libc::c_uint,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8
            as *const libc::c_char,
    );
    if edge as libc::c_uint == YGEdgeLeft as libc::c_int as libc::c_uint {
        if (*node).layout.direction as libc::c_uint == YGDirectionRTL as libc::c_int as libc::c_uint
        {
            return (*node).layout.border[YGEdgeEnd as libc::c_int as usize];
        } else {
            return (*node).layout.border[YGEdgeStart as libc::c_int as usize];
        };
    };
    if edge as libc::c_uint == YGEdgeRight as libc::c_int as libc::c_uint {
        if (*node).layout.direction as libc::c_uint == YGDirectionRTL as libc::c_int as libc::c_uint
        {
            return (*node).layout.border[YGEdgeStart as libc::c_int as usize];
        } else {
            return (*node).layout.border[YGEdgeEnd as libc::c_int as usize];
        };
    };
    return (*node).layout.border[edge as usize];
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeLayoutGetPadding(node: YGNodeRef, edge: YGEdge) -> libc::c_float {
    YGAssertWithNode(
        node,
        (edge as libc::c_uint) < YGEdgeEnd as libc::c_int as libc::c_uint,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8
            as *const libc::c_char,
    );
    if edge as libc::c_uint == YGEdgeLeft as libc::c_int as libc::c_uint {
        if (*node).layout.direction as libc::c_uint == YGDirectionRTL as libc::c_int as libc::c_uint
        {
            return (*node).layout.padding[YGEdgeEnd as libc::c_int as usize];
        } else {
            return (*node).layout.padding[YGEdgeStart as libc::c_int as usize];
        };
    };
    if edge as libc::c_uint == YGEdgeRight as libc::c_int as libc::c_uint {
        if (*node).layout.direction as libc::c_uint == YGDirectionRTL as libc::c_int as libc::c_uint
        {
            return (*node).layout.padding[YGEdgeStart as libc::c_int as usize];
        } else {
            return (*node).layout.padding[YGEdgeEnd as libc::c_int as usize];
        };
    };
    return (*node).layout.padding[edge as usize];
}

#[no_mangle]
pub unsafe extern "C" fn YGConfigSetPointScaleFactor(
    config: YGConfigRef,
    pixelsInPoint: libc::c_float,
) -> () {
    YGAssertWithConfig(
        config,
        pixelsInPoint >= 0.0f32,
        b"Scale factor should not be less than zero\x00" as *const u8 as *const libc::c_char,
    );
    if pixelsInPoint == 0.0f32 {
        (*config).pointScaleFactor = 0.0f32;
    } else {
        (*config).pointScaleFactor = pixelsInPoint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigSetUseLegacyStretchBehaviour(
    config: YGConfigRef,
    useLegacyStretchBehaviour: bool,
) -> () {
    (*config).useLegacyStretchBehaviour = useLegacyStretchBehaviour;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigNew() -> YGConfigRef {
    let config: YGConfigRef = gYGMalloc.expect("non-null function pointer")(::std::mem::size_of::<
        YGConfig,
    >()
        as libc::c_ulong) as YGConfigRef;
    YGAssert(
        !config.is_null(),
        b"Could not allocate memory for config\x00" as *const u8 as *const libc::c_char,
    );
    gConfigInstanceCount += 1;
    memcpy(
        config as *mut libc::c_void,
        &mut gYGConfigDefaults as *mut YGConfig as *const libc::c_void,
        ::std::mem::size_of::<YGConfig>() as libc::c_ulong,
    );
    return config;
}
#[no_mangle]
pub static mut gConfigInstanceCount: int32_t = unsafe { 0i32 };
#[no_mangle]
pub unsafe extern "C" fn YGConfigFree(config: YGConfigRef) -> () {
    gYGFree.expect("non-null function pointer")(config as *mut libc::c_void);
    gConfigInstanceCount -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigCopy(dest: YGConfigRef, src: YGConfigRef) -> () {
    memcpy(
        dest as *mut libc::c_void,
        src as *const libc::c_void,
        ::std::mem::size_of::<YGConfig>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigGetInstanceCount() -> int32_t {
    return gConfigInstanceCount;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigSetExperimentalFeatureEnabled(
    config: YGConfigRef,
    feature: YGExperimentalFeature_0,
    enabled: bool,
) -> () {
    (*config).experimentalFeatures[feature as usize] = enabled;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigSetUseWebDefaults(config: YGConfigRef, enabled: bool) -> () {
    (*config).useWebDefaults = enabled;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigGetUseWebDefaults(config: YGConfigRef) -> bool {
    return (*config).useWebDefaults;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigSetNodeClonedFunc(
    config: YGConfigRef,
    callback: YGNodeClonedFunc,
) -> () {
    (*config).cloneNodeCallback = callback;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigGetDefault() -> YGConfigRef {
    return &mut gYGConfigDefaults as *mut YGConfig;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigSetContext(
    config: YGConfigRef,
    mut context: *mut libc::c_void,
) -> () {
    (*config).context = context;
}
#[no_mangle]
pub unsafe extern "C" fn YGConfigGetContext(config: YGConfigRef) -> *mut libc::c_void {
    return (*config).context;
}
#[no_mangle]
pub unsafe extern "C" fn YGNodeListAdd(mut listp: *mut YGNodeListRef, node: YGNodeRef) -> () {
    if (*listp).is_null() {
        *listp = YGNodeListNew(4i32 as uint32_t);
    };
    YGNodeListInsert(listp, node, (**listp).count);
}
#[no_mangle]
pub static mut gYGCalloc: YGCalloc = unsafe { Some(calloc) };

unsafe extern "C" fn YGConstrainMaxSizeForMode(
    node: YGNodeRef,
    axis: YGFlexDirection,
    parentAxisSize: libc::c_float,
    parentWidth: libc::c_float,
    mode: *mut YGMeasureMode,
    size: *mut libc::c_float,
) {
    // TODO(anp): impl this
    // const float maxSize = YGResolveValue(&node->style.maxDimensions[dim[axis]], parentAxisSize) +
    //                       YGNodeMarginForAxis(node, axis, parentWidth);
    // switch (*mode)
    // {
    // case YGMeasureModeExactly:
    // case YGMeasureModeAtMost:
    //     *size = (YGFloatIsUndefined(maxSize) || *size < maxSize) ? *size : maxSize;
    //     break;
    // case YGMeasureModeUndefined:
    //     if (!YGFloatIsUndefined(maxSize))
    //     {
    //         *mode = YGMeasureModeAtMost;
    //         *size = maxSize;
    //     }
    //     break;
    // }
}
