// TODO(anp): check out the inline annotations from the c code
// TODO(anp): double check c code for interesting comments
// TODO(anp): revist raph's continuation-based layout stuff, in case you forget, june 2018 meetup at mozilla
use std::f32::NAN as NAN32;
use std::ffi::CStr;
use std::mem::size_of;

use libc::*;

use ffi_types::{
    align::Align, dimension::{Dimension, Dimensions, MeasuredDimensions, ResolvedDimensions},
    direction::Direction, display::Display, edge::Edge, flex_direction::FlexDirection,
    justify::Justify, measure_mode::MeasureMode, node_type::NodeType,
};

unsafe fn YGResolveValue(value: *const YGValue, parentSize: c_float) -> c_float {
    match (*value).unit {
        YGUnitPoint => (*value).value,
        YGUnitPercent => (*value).value * parentSize / 100.0f32,
        _ => ::std::f32::NAN,
    }
}

type YGNodeClonedFunc =
    Option<unsafe extern "C" fn(_: YGNodeRef, _: YGNodeRef, _: YGNodeRef, _: i32) -> ()>;

const _IEEE_: _LIB_VERSION_TYPE = -1;
const _ISOC_: _LIB_VERSION_TYPE = 3;
const _POSIX_: _LIB_VERSION_TYPE = 2;
const _SVID_: _LIB_VERSION_TYPE = 0;
const _XOPEN_: _LIB_VERSION_TYPE = 1;
const YGExperimentalFeatureWebFlexBasis: YGExperimentalFeature = 0;
pub const YGOverflowHidden: YGOverflow = 1;
pub const YGOverflowScroll: YGOverflow = 2;
pub const YGOverflowVisible: YGOverflow = 0;
pub const YGPositionTypeAbsolute: YGPositionType_0 = 1;
pub const YGPositionTypeRelative: YGPositionType_0 = 0;
pub const YGUnitAuto: YGUnit = 3;
pub const YGUnitPercent: YGUnit = 2;
pub const YGUnitPoint: YGUnit = 1;
pub const YGUnitUndefined: YGUnit = 0;
pub const YGWrapNoWrap: YGWrap = 0;
pub const YGWrapWrap: YGWrap = 1;
pub const YGWrapWrapReverse: YGWrap = 2;
type __builtin_va_list = [__va_list_tag; 1];
type __off_t = i64;
type __off64_t = i64;
type _IO_lock_t = ();
type _LIB_VERSION_TYPE = i32;
type FILE = _IO_FILE;
type int32_t = i32;
type size_t = c_ulong;
type uint32_t = c_uint;
type va_list = __builtin_va_list;
type YGCachedMeasurement_0 = YGCachedMeasurement;
type YGConfig = YGConfig_0;
type YGConfigRef = *mut YGConfig_0;
type YGExperimentalFeature = c_uint;
type YGExperimentalFeature_0 = YGExperimentalFeature;
type YGFree = Option<unsafe extern "C" fn(_: *mut c_void) -> ()>;
type YGLayout_0 = YGLayout;
type YGMalloc = Option<unsafe extern "C" fn(_: size_t) -> *mut c_void>;
pub type YGNode = YGNode_0;
pub type YGNodeListRef = *mut YGNodeList;
pub type YGNodeRef = *mut YGNode_0;
pub type YGOverflow = c_uint;
type YGOverflow_0 = YGOverflow;
pub type YGPositionType = YGPositionType_0;
type YGPositionType_0 = c_uint;
type YGSize_0 = YGSize;
type YGStringStream_0 = YGStringStream;
type YGStyle_0 = YGStyle;
pub type YGUnit = c_uint;
type YGUnit_0 = YGUnit;
pub type YGValue = YGValue_0;
pub type YGWrap = c_uint;
type YGWrap_0 = YGWrap;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGNodeList {
    capacity: usize,
    count: usize,
    items: *mut YGNodeRef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGValue_0 {
    pub value: c_float,
    pub unit: YGUnit_0,
}

impl PartialEq for YGValue {
    fn eq(&self, other: &Self) -> bool {
        if self.unit != other.unit {
            return false;
        }

        if self.unit == YGUnitUndefined {
            return true;
        }

        return (self.value - other.value).abs() < 0.0001;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct YGLayout {
    position: [c_float; 4],
    dimensions: [c_float; 2],
    margin: [c_float; 6],
    border: [c_float; 6],
    padding: [c_float; 6],
    direction: Direction,
    computedFlexBasisGeneration: uint32_t,
    computedFlexBasis: c_float,
    hadOverflow: bool,
    generationCount: uint32_t,
    lastParentDirection: Direction,
    nextCachedMeasurementsIndex: usize,
    cachedMeasurements: [YGCachedMeasurement_0; 16],
    measuredDimensions: MeasuredDimensions,
    cachedLayout: YGCachedMeasurement_0,
}
type YGRealloc = Option<unsafe extern "C" fn(_: *mut c_void, _: size_t) -> *mut c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
struct YGStringStream {
    str_0: *mut c_char,
    length: uint32_t,
    capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct _IO_FILE {
    _flags: i32,
    _IO_read_ptr: *mut c_char,
    _IO_read_end: *mut c_char,
    _IO_read_base: *mut c_char,
    _IO_write_base: *mut c_char,
    _IO_write_ptr: *mut c_char,
    _IO_write_end: *mut c_char,
    _IO_buf_base: *mut c_char,
    _IO_buf_end: *mut c_char,
    _IO_save_base: *mut c_char,
    _IO_backup_base: *mut c_char,
    _IO_save_end: *mut c_char,
    _markers: *mut _IO_marker,
    _chain: *mut _IO_FILE,
    _fileno: i32,
    _flags2: i32,
    _old_offset: __off_t,
    _cur_column: c_ushort,
    _vtable_offset: c_schar,
    _shortbuf: [c_char; 1],
    _lock: *mut c_void,
    _offset: __off64_t,
    __pad1: *mut c_void,
    __pad2: *mut c_void,
    __pad3: *mut c_void,
    __pad4: *mut c_void,
    __pad5: size_t,
    _mode: i32,
    _unused2: [c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct YGStyle {
    direction: Direction,
    flexDirection: FlexDirection,
    justifyContent: Justify,
    alignContent: Align,
    alignItems: Align,
    alignSelf: Align,
    positionType: YGPositionType,
    flexWrap: YGWrap_0,
    overflow: YGOverflow_0,
    display: Display,
    flex: c_float,
    flexGrow: c_float,
    flexShrink: c_float,
    flexBasis: YGValue,
    margin: [YGValue; 9],
    position: [YGValue; 9],
    padding: [YGValue; 9],
    border: [YGValue; 9],
    dimensions: Dimensions,
    minDimensions: Dimensions,
    maxDimensions: Dimensions,
    aspectRatio: c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct __va_list_tag {
    gp_offset: c_uint,
    fp_offset: c_uint,
    overflow_arg_area: *mut c_void,
    reg_save_area: *mut c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct YGCachedMeasurement {
    availableWidth: c_float,
    availableHeight: c_float,
    widthMeasureMode: Option<MeasureMode>,
    heightMeasureMode: Option<MeasureMode>,
    computedWidth: c_float,
    computedHeight: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGSize {
    pub width: c_float,
    pub height: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct _IO_marker {
    _next: *mut _IO_marker,
    _sbuf: *mut _IO_FILE,
    _pos: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGNode_0 {
    style: YGStyle_0,
    layout: YGLayout_0,
    lineIndex: usize,
    parent: YGNodeRef,
    children: YGNodeListRef,
    nextChild: *mut YGNode_0,
    measure: YGMeasureFunc,
    baseline: YGBaselineFunc,
    config: YGConfigRef,
    context: *mut c_void,
    isDirty: bool,
    hasNewLayout: bool,
    nodeType: NodeType,
    resolvedDimensions: ResolvedDimensions,
}
type YGBaselineFunc = Option<unsafe extern "C" fn(_: YGNodeRef, _: c_float, _: c_float) -> c_float>;
type YGMeasureFunc = Option<
    unsafe extern "C" fn(_: YGNodeRef, _: c_float, _: MeasureMode, _: c_float, _: MeasureMode)
        -> YGSize_0,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YGConfig_0 {
    experimentalFeatures: [bool; 2],
    useWebDefaults: bool,
    useLegacyStretchBehaviour: bool,
    pointScaleFactor: c_float,
    cloneNodeCallback: YGNodeClonedFunc,
    context: *mut c_void,
}

pub unsafe extern "C" fn YGRoundValueToPixelGrid(
    value: c_float,
    pointScaleFactor: c_float,
    forceCeil: bool,
    forceFloor: bool,
) -> c_float {
    let mut scaledValue: c_float = value * pointScaleFactor;
    let mut fractial: c_float = scaledValue % 1.0;
    if YGFloatsEqual(fractial, 0i32 as c_float) {
        scaledValue = scaledValue - fractial;
    } else {
        if YGFloatsEqual(fractial, 1.0f64 as c_float) {
            scaledValue = ((scaledValue - fractial) as c_double + 1.0f64) as c_float;
        } else {
            if forceCeil {
                scaledValue = scaledValue - fractial + 1.0f32;
            } else {
                if forceFloor {
                    scaledValue = scaledValue - fractial;
                } else {
                    scaledValue = scaledValue - fractial
                        + if fractial > 0.5f32 || 0 != YGFloatsEqual(fractial, 0.5f32) as i32 {
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
pub unsafe extern "C" fn YGFloatsEqual(a: c_float, b: c_float) -> bool {
    if a.is_nan() {
        return b.is_nan();
    };
    return ((a - b).abs() as f64) < 0.00009999999747378752f32 as c_double;
}

static mut YGValueUndefined: YGValue = YGValue_0 {
    value: ::std::f32::NAN,
    unit: YGUnitUndefined,
};
static mut YGValueAuto: YGValue = YGValue_0 {
    value: ::std::f32::NAN,
    unit: YGUnitAuto,
};

pub unsafe extern "C" fn YGNodeNew() -> YGNodeRef {
    return YGNodeNewWithConfig(&mut gYGConfigDefaults as *mut YGConfig);
}
static mut gYGConfigDefaults: YGConfig = YGConfig_0 {
    experimentalFeatures: [0 != 0i32, false],
    useWebDefaults: 0 != 0i32,
    useLegacyStretchBehaviour: false,
    pointScaleFactor: 1.0f32,
    cloneNodeCallback: None,
    context: 0 as *const c_void as *mut c_void,
};

pub unsafe extern "C" fn YGNodeNewWithConfig(config: YGConfigRef) -> YGNodeRef {
    let node: YGNodeRef = malloc(size_of::<YGNode>()) as YGNodeRef;
    YGAssertWithConfig(
        config,
        !node.is_null(),
        b"Could not allocate memory for node\x00" as *const u8 as *const c_char,
    );
    gNodeInstanceCount += 1;
    memcpy(
        node as *mut c_void,
        &gYGNodeDefaults as *const YGNode as *const c_void,
        size_of::<YGNode>(),
    );
    if (*config).useWebDefaults {
        (*node).style.flexDirection = FlexDirection::Row;
        (*node).style.alignContent = Align::Stretch;
    };
    (*node).config = config;
    return node;
}

static mut gYGNodeDefaults: YGNode = unsafe {
    YGNode_0 {
        style: YGStyle {
            direction: Direction::Inherit,
            flexDirection: FlexDirection::Column,
            justifyContent: Justify::FlexStart,
            alignContent: Align::FlexStart,
            alignItems: Align::Stretch,
            alignSelf: Align::Auto,
            positionType: YGPositionTypeRelative,
            flexWrap: YGWrapNoWrap,
            overflow: YGOverflowVisible,
            display: Display::Flex,
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
            dimensions: Dimensions {
                width: YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitAuto,
                },
                height: YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitAuto,
                },
            },
            minDimensions: Dimensions {
                width: YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                height: YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
            },
            maxDimensions: Dimensions {
                width: YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
                height: YGValue_0 {
                    value: ::std::f32::NAN,
                    unit: YGUnitUndefined,
                },
            },
            aspectRatio: ::std::f32::NAN,
        },
        layout: YGLayout {
            position: [0.; 4],
            dimensions: [::std::f32::NAN, ::std::f32::NAN],
            margin: [0.; 6],
            border: [0.; 6],
            padding: [0.; 6],
            direction: Direction::Inherit,
            computedFlexBasisGeneration: 0,
            computedFlexBasis: ::std::f32::NAN,
            hadOverflow: 0 != 0i32,
            generationCount: 0,
            // RIIR(anp): this is not technically correct, it was uninit  before
            lastParentDirection: Direction::Inherit,
            nextCachedMeasurementsIndex: 0,
            cachedMeasurements: [YGCachedMeasurement {
                availableWidth: 0.,
                availableHeight: 0.,
                widthMeasureMode: None,
                heightMeasureMode: None,
                computedWidth: 0.,
                computedHeight: 0.,
            }; 16],
            measuredDimensions: MeasuredDimensions {
                width: NAN32,
                height: NAN32,
            },
            cachedLayout: YGCachedMeasurement {
                availableWidth: 0.,
                availableHeight: 0.,
                widthMeasureMode: None,
                heightMeasureMode: None,
                computedWidth: -1i32 as c_float,
                computedHeight: -1i32 as c_float,
            },
        },
        lineIndex: 0,
        parent: 0 as *const YGNode_0 as YGNodeRef,
        children: 0 as *const YGNodeList as YGNodeListRef,
        nextChild: 0 as *const YGNode_0 as *mut YGNode_0,
        measure: None,
        baseline: None,
        config: 0 as *const YGConfig_0 as *mut YGConfig_0,
        context: 0 as *const c_void as *mut c_void,
        isDirty: 0 != 0i32,
        hasNewLayout: 0 != 1i32,
        nodeType: NodeType::Default,
        resolvedDimensions: ResolvedDimensions {
            width: &YGValueUndefined as *const YGValue,
            height: &YGValueUndefined as *const YGValue,
        },
    }
};

static mut gNodeInstanceCount: int32_t = 0i32;

pub unsafe extern "C" fn YGAssertWithConfig(
    config: YGConfigRef,
    condition: bool,
    mut message: *const c_char,
) -> () {
    if !condition {
        error!("{:?} (config: {:?})", CStr::from_ptr(message), config);
    };
}

pub unsafe extern "C" fn YGNodeClone(oldNode: YGNodeRef) -> YGNodeRef {
    let node: YGNodeRef = malloc(size_of::<YGNode>()) as YGNodeRef;
    YGAssertWithConfig(
        (*oldNode).config,
        !node.is_null(),
        b"Could not allocate memory for node\x00" as *const u8 as *const c_char,
    );
    gNodeInstanceCount += 1;
    memcpy(
        node as *mut c_void,
        oldNode as *const c_void,
        size_of::<YGNode>(),
    );
    (*node).children = YGNodeListClone((*oldNode).children);
    (*node).parent = 0 as YGNodeRef;
    return node;
}

pub unsafe extern "C" fn YGNodeListClone(oldList: YGNodeListRef) -> YGNodeListRef {
    if oldList.is_null() {
        return 0 as YGNodeListRef;
    };
    let count = (*oldList).count;
    if count == 0 {
        return 0 as YGNodeListRef;
    };
    let newList: YGNodeListRef = YGNodeListNew(count);
    memcpy(
        (*newList).items as *mut c_void,
        (*oldList).items as *const c_void,
        size_of::<YGNodeRef>().wrapping_mul(count as usize),
    );
    (*newList).count = count;
    return newList;
}

pub unsafe extern "C" fn YGNodeListNew(initialCapacity: usize) -> YGNodeListRef {
    let list: YGNodeListRef = malloc(size_of::<YGNodeList>()) as YGNodeListRef;
    assert!(!list.is_null(), "Could not allocate memory for list");

    (*list).capacity = initialCapacity;
    (*list).count = 0;
    (*list).items = malloc(size_of::<YGNodeRef>().wrapping_mul((*list).capacity)) as *mut YGNodeRef;

    assert!(
        !(*list).items.is_null(),
        "Could not allocate memory for items"
    );

    return list;
}

pub unsafe extern "C" fn YGAssert(condition: bool, mut message: *const c_char) -> () {
    if !condition {
        error!("{:?}", CStr::from_ptr(message));
    };
}

pub unsafe extern "C" fn YGNodeFree(node: YGNodeRef) -> () {
    if !(*node).parent.is_null() {
        YGNodeListDelete((*(*node).parent).children, node);
        (*node).parent = 0 as YGNodeRef;
    };
    let childCount = YGNodeGetChildCount(node);
    {
        let mut i = 0;
        while i < childCount {
            {
                let child: YGNodeRef = YGNodeGetChild(node, i);
                (*child).parent = 0 as YGNodeRef;
            }
            i = i.wrapping_add(1);
        }
    }
    YGNodeListFree((*node).children);
    gYGFree.expect("non-null function pointer")(node as *mut c_void);
    gNodeInstanceCount -= 1;
}

static mut gYGFree: YGFree = Some(free);

pub unsafe extern "C" fn YGNodeListFree(list: YGNodeListRef) -> () {
    if !list.is_null() {
        gYGFree.expect("non-null function pointer")((*list).items as *mut c_void);
        gYGFree.expect("non-null function pointer")(list as *mut c_void);
    };
}

pub unsafe extern "C" fn YGNodeGetChild(node: YGNodeRef, index: usize) -> YGNodeRef {
    return YGNodeListGet((*node).children, index);
}

pub unsafe extern "C" fn YGNodeListGet(list: YGNodeListRef, index: usize) -> YGNodeRef {
    if YGNodeListCount(list) > 0 {
        return *(*list).items.offset(index as isize);
    };
    return 0 as YGNodeRef;
}

pub unsafe extern "C" fn YGNodeListCount(list: YGNodeListRef) -> usize {
    if !list.is_null() {
        return (*list).count;
    };
    return 0;
}

pub unsafe extern "C" fn YGNodeGetChildCount(node: YGNodeRef) -> usize {
    return YGNodeListCount((*node).children);
}

pub unsafe extern "C" fn YGNodeListDelete(list: YGNodeListRef, node: YGNodeRef) -> YGNodeRef {
    {
        let mut i = 0;
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

pub unsafe extern "C" fn YGNodeListRemove(list: YGNodeListRef, index: usize) -> YGNodeRef {
    let removed: YGNodeRef = *(*list).items.offset(index as isize);
    let ref mut fresh0 = *(*list).items.offset(index as isize);
    *fresh0 = 0 as YGNodeRef;
    {
        let mut i = index;
        while i < (*list).count.wrapping_sub(1) {
            {
                let ref mut fresh1 = *(*list).items.offset(i as isize);
                *fresh1 = *(*list).items.offset(i.wrapping_add(1) as isize);
                let ref mut fresh2 = *(*list).items.offset(i.wrapping_add(1) as isize);
                *fresh2 = 0 as YGNodeRef;
            }
            i = i.wrapping_add(1);
        }
    }
    (*list).count = (*list).count.wrapping_sub(1);
    return removed;
}

pub unsafe extern "C" fn YGNodeFreeRecursive(root: YGNodeRef) -> () {
    while YGNodeGetChildCount(root) > 0 {
        let child: YGNodeRef = YGNodeGetChild(root, 0);
        if (*child).parent != root {
            break;
        };
        YGNodeRemoveChild(root, child);
        YGNodeFreeRecursive(child);
    }
    YGNodeFree(root);
}

pub unsafe extern "C" fn YGNodeRemoveChild(parent: YGNodeRef, excludedChild: YGNodeRef) -> () {
    let childCount = YGNodeGetChildCount(parent);
    if childCount == 0 {
        return;
    };

    let firstChild: YGNodeRef = YGNodeGetChild(parent, 0);
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
    let mut nextInsertIndex = 0usize;
    {
        let mut i = 0;
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
                            nextInsertIndex as i32,
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

pub unsafe extern "C" fn YGNodeListReplace(
    mut list: YGNodeListRef,
    index: usize,
    newNode: YGNodeRef,
) -> () {
    let ref mut fresh3 = *(*list).items.offset(index as isize);
    *fresh3 = newNode;
}
pub unsafe extern "C" fn YGNodeMarkDirtyInternal(node: YGNodeRef) -> () {
    if !(*node).isDirty {
        (*node).isDirty = 0 != 1i32;
        (*node).layout.computedFlexBasis = ::std::f32::NAN;
        if !(*node).parent.is_null() {
            YGNodeMarkDirtyInternal((*node).parent);
        };
    };
}

pub unsafe extern "C" fn YGNodeReset(node: YGNodeRef) -> () {
    YGAssertWithNode(
        node,
        YGNodeGetChildCount(node) == 0,
        b"Cannot reset a node which still has children attached\x00" as *const u8 as *const c_char,
    );
    YGAssertWithNode(
        node,
        (*node).parent.is_null(),
        b"Cannot reset a node still attached to a parent\x00" as *const u8 as *const c_char,
    );
    YGNodeListFree((*node).children);
    let config: YGConfigRef = (*node).config;
    memcpy(
        node as *mut c_void,
        &gYGNodeDefaults as *const YGNode as *const c_void,
        size_of::<YGNode>(),
    );
    if (*config).useWebDefaults {
        (*node).style.flexDirection = FlexDirection::Row;
        (*node).style.alignContent = Align::Stretch;
    };
    (*node).config = config;
}

pub unsafe extern "C" fn YGAssertWithNode(
    node: YGNodeRef,
    condition: bool,
    mut message: *const c_char,
) -> () {
    if !condition {
        error!("{:?} (node: {:?})", &CStr::from_ptr(message), node);
    };
}

pub unsafe extern "C" fn YGNodeGetInstanceCount() -> int32_t {
    return gNodeInstanceCount;
}

pub unsafe extern "C" fn YGNodeInsertChild(node: YGNodeRef, child: YGNodeRef, index: usize) -> () {
    YGAssertWithNode(
        node,
        (*child).parent.is_null(),
        b"Child already has a parent, it must be removed first.\x00" as *const u8 as *const c_char,
    );
    YGAssertWithNode(
        node,
        (*node).measure.is_none(),
        b"Cannot add child: Nodes with measure functions cannot have children.\x00" as *const u8
            as *const c_char,
    );
    YGCloneChildrenIfNeeded(node);
    YGNodeListInsert(&mut (*node).children as *mut YGNodeListRef, child, index);
    (*child).parent = node;
    YGNodeMarkDirtyInternal(node);
}

pub unsafe extern "C" fn YGNodeListInsert(
    mut listp: *mut YGNodeListRef,
    node: YGNodeRef,
    index: usize,
) -> () {
    if (*listp).is_null() {
        *listp = YGNodeListNew(4);
    };
    let mut list: YGNodeListRef = *listp;
    if (*list).count == (*list).capacity {
        (*list).capacity = (*list).capacity.wrapping_mul(2);
        (*list).items = realloc(
            (*list).items as *mut c_void,
            size_of::<YGNodeRef>().wrapping_mul((*list).capacity),
        ) as *mut YGNodeRef;
        YGAssert(
            !(*list).items.is_null(),
            b"Could not extend allocation for items\x00" as *const u8 as *const c_char,
        );
    };
    {
        let mut i = (*list).count;
        while i > index {
            {
                let ref mut fresh4 = *(*list).items.offset(i as isize);
                *fresh4 = *(*list).items.offset(i.wrapping_sub(1) as isize);
            }
            i = i.wrapping_sub(1);
        }
    }
    (*list).count = (*list).count.wrapping_add(1);
    let ref mut fresh5 = *(*list).items.offset(index as isize);
    *fresh5 = node;
}

pub unsafe extern "C" fn YGCloneChildrenIfNeeded(parent: YGNodeRef) -> () {
    let childCount = YGNodeGetChildCount(parent);
    if childCount == 0 {
        return;
    };
    let firstChild: YGNodeRef = YGNodeGetChild(parent, 0);
    if (*firstChild).parent == parent {
        return;
    };
    let cloneNodeCallback: YGNodeClonedFunc = (*(*parent).config).cloneNodeCallback;
    let children: YGNodeListRef = (*parent).children;
    {
        let mut i = 0;
        while i < childCount {
            {
                let oldChild: YGNodeRef = YGNodeListGet(children, i);
                let newChild: YGNodeRef = YGNodeClone(oldChild);
                YGNodeListReplace(children, i, newChild);
                (*newChild).parent = parent;
                if cloneNodeCallback.is_some() {
                    cloneNodeCallback.expect("non-null function pointer")(
                        oldChild, newChild, parent, i as i32,
                    );
                };
            }
            i = i.wrapping_add(1);
        }
    };
}

pub unsafe extern "C" fn YGNodeRemoveAllChildren(parent: YGNodeRef) -> () {
    let childCount = YGNodeGetChildCount(parent);
    if childCount == 0 {
        return;
    };
    let firstChild: YGNodeRef = YGNodeGetChild(parent, 0);
    if (*firstChild).parent == parent {
        {
            let mut i = 0usize;
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

pub unsafe extern "C" fn YGNodeListRemoveAll(list: YGNodeListRef) -> () {
    {
        let mut i = 0usize;
        while i < (*list).count {
            {
                let ref mut fresh6 = *(*list).items.offset(i as isize);
                *fresh6 = 0 as YGNodeRef;
            }
            i = i.wrapping_add(1);
        }
    }
    (*list).count = 0;
}

pub unsafe extern "C" fn YGNodeGetParent(node: YGNodeRef) -> YGNodeRef {
    return (*node).parent;
}

pub unsafe extern "C" fn YGNodeCalculateLayout(
    node: YGNodeRef,
    parentWidth: c_float,
    parentHeight: c_float,
    parentDirection: Direction,
) -> () {
    gCurrentGenerationCount = gCurrentGenerationCount.wrapping_add(1);
    YGResolveDimensions(node);
    let mut width: c_float;
    let mut widthMeasureMode: MeasureMode;
    if YGNodeIsStyleDimDefined(node, FlexDirection::Row, parentWidth) {
        width = YGResolveValue(
            (*node).resolvedDimensions[DIM[FlexDirection::Row as i32 as usize]],
            parentWidth,
        ) + YGNodeMarginForAxis(node, FlexDirection::Row, parentWidth);
        widthMeasureMode = MeasureMode::Exactly;
    } else {
        if YGResolveValue(
            &mut (*node).style.maxDimensions.width as *mut YGValue,
            parentWidth,
        ) >= 0.0f32
        {
            width = YGResolveValue(
                &mut (*node).style.maxDimensions.width as *mut YGValue,
                parentWidth,
            );
            widthMeasureMode = MeasureMode::AtMost;
        } else {
            width = parentWidth;
            widthMeasureMode = if width.is_nan() {
                MeasureMode::Undefined
            } else {
                MeasureMode::Exactly
            };
        };
    };
    let mut height: c_float;
    let mut heightMeasureMode: MeasureMode;
    if YGNodeIsStyleDimDefined(node, FlexDirection::Column, parentHeight) {
        height = YGResolveValue(
            (*node).resolvedDimensions[DIM[FlexDirection::Column as i32 as usize]],
            parentHeight,
        ) + YGNodeMarginForAxis(node, FlexDirection::Column, parentWidth);
        heightMeasureMode = MeasureMode::Exactly;
    } else {
        if YGResolveValue(
            &mut (*node).style.maxDimensions.height as *mut YGValue,
            parentHeight,
        ) >= 0.0f32
        {
            height = YGResolveValue(
                &mut (*node).style.maxDimensions.height as *mut YGValue,
                parentHeight,
            );
            heightMeasureMode = MeasureMode::AtMost;
        } else {
            height = parentHeight;
            heightMeasureMode = if height.is_nan() {
                MeasureMode::Undefined
            } else {
                MeasureMode::Exactly
            };
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
        "initial",
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

pub unsafe extern "C" fn YGComputedEdgeValue(
    mut edges: *const YGValue,
    edge: Edge,
    defaultValue: *const YGValue,
) -> *const YGValue {
    if (*edges.offset(edge as isize)).unit as c_uint != YGUnitUndefined as i32 as c_uint {
        return &*edges.offset(edge as isize) as *const YGValue;
    };
    if (edge as c_uint == Edge::Top as i32 as c_uint
        || edge as c_uint == Edge::Bottom as i32 as c_uint)
        && (*edges.offset(Edge::Vertical as i32 as isize)).unit as c_uint
            != YGUnitUndefined as i32 as c_uint
    {
        return &*edges.offset(Edge::Vertical as i32 as isize) as *const YGValue;
    };
    if (edge as c_uint == Edge::Left as i32 as c_uint
        || edge as c_uint == Edge::Right as i32 as c_uint
        || edge as c_uint == Edge::Start as i32 as c_uint
        || edge as c_uint == Edge::End as i32 as c_uint)
        && (*edges.offset(Edge::Horizontal as i32 as isize)).unit as c_uint
            != YGUnitUndefined as i32 as c_uint
    {
        return &*edges.offset(Edge::Horizontal as i32 as isize) as *const YGValue;
    };
    if (*edges.offset(Edge::All as i32 as isize)).unit as c_uint != YGUnitUndefined as i32 as c_uint
    {
        return &*edges.offset(Edge::All as i32 as isize) as *const YGValue;
    };
    if edge as c_uint == Edge::Start as i32 as c_uint
        || edge as c_uint == Edge::End as i32 as c_uint
    {
        return &YGValueUndefined as *const YGValue;
    };
    return defaultValue;
}
pub unsafe extern "C" fn YGNodeSetPosition(
    node: YGNodeRef,
    direction: Direction,
    mainSize: c_float,
    crossSize: c_float,
    parentWidth: c_float,
) -> () {
    let directionRespectingRoot: Direction = if !(*node).parent.is_null() {
        direction
    } else {
        Direction::LTR
    };
    let mainAxis: FlexDirection =
        YGResolveFlexDirection((*node).style.flexDirection, directionRespectingRoot);
    let crossAxis: FlexDirection = FlexDirectionCross(mainAxis, directionRespectingRoot);
    let relativePositionMain: c_float = YGNodeRelativePosition(node, mainAxis, mainSize);
    let relativePositionCross: c_float = YGNodeRelativePosition(node, crossAxis, crossSize);
    (*node).layout.position[leading[mainAxis as usize] as usize] =
        YGNodeLeadingMargin(node, mainAxis, parentWidth) + relativePositionMain;
    (*node).layout.position[trailing[mainAxis as usize] as usize] =
        YGNodeTrailingMargin(node, mainAxis, parentWidth) + relativePositionMain;
    (*node).layout.position[leading[crossAxis as usize] as usize] =
        YGNodeLeadingMargin(node, crossAxis, parentWidth) + relativePositionCross;
    (*node).layout.position[trailing[crossAxis as usize] as usize] =
        YGNodeTrailingMargin(node, crossAxis, parentWidth) + relativePositionCross;
}
pub unsafe extern "C" fn YGResolveFlexDirection(
    flexDirection: FlexDirection,
    direction: Direction,
) -> FlexDirection {
    if direction == Direction::RTL {
        if flexDirection as c_uint == FlexDirection::Row as i32 as c_uint {
            return FlexDirection::RowReverse;
        } else {
            if flexDirection as c_uint == FlexDirection::RowReverse as i32 as c_uint {
                return FlexDirection::Row;
            };
        };
    };
    return flexDirection;
}
unsafe extern "C" fn FlexDirectionCross(
    flexDirection: FlexDirection,
    direction: Direction,
) -> FlexDirection {
    return (if 0 != FlexDirectionIsColumn(flexDirection) as i32 {
        YGResolveFlexDirection(FlexDirection::Row, direction)
    } else {
        FlexDirection::Column
    }) as FlexDirection;
}
unsafe extern "C" fn FlexDirectionIsColumn(flexDirection: FlexDirection) -> bool {
    return flexDirection as c_uint == FlexDirection::Column as i32 as c_uint
        || flexDirection as c_uint == FlexDirection::ColumnReverse as i32 as c_uint;
}
pub unsafe extern "C" fn YGNodeRelativePosition(
    node: YGNodeRef,
    axis: FlexDirection,
    axisSize: c_float,
) -> c_float {
    return if 0 != YGNodeIsLeadingPosDefined(node, axis) as i32 {
        YGNodeLeadingPosition(node, axis, axisSize)
    } else {
        -YGNodeTrailingPosition(node, axis, axisSize)
    };
}
pub unsafe extern "C" fn YGNodeTrailingPosition(
    node: YGNodeRef,
    axis: FlexDirection,
    axisSize: c_float,
) -> c_float {
    if FlexDirectionIsRow(axis) {
        let mut trailingPosition: *const YGValue = YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            Edge::End,
            &YGValueUndefined as *const YGValue,
        );
        if (*trailingPosition).unit as c_uint != YGUnitUndefined as i32 as c_uint {
            return YGResolveValue(trailingPosition, axisSize);
        };
    };
    let mut trailingPosition: *const YGValue = YGComputedEdgeValue(
        (*node).style.position.as_mut_ptr() as *const YGValue,
        trailing[axis as usize],
        &YGValueUndefined as *const YGValue,
    );
    return if (*trailingPosition).unit as c_uint == YGUnitUndefined as i32 as c_uint {
        0.0f32
    } else {
        YGResolveValue(trailingPosition, axisSize)
    };
}
static mut trailing: [Edge; 4] = [Edge::Bottom, Edge::Top, Edge::Right, Edge::Left];
unsafe extern "C" fn FlexDirectionIsRow(flexDirection: FlexDirection) -> bool {
    return flexDirection as c_uint == FlexDirection::Row as i32 as c_uint
        || flexDirection as c_uint == FlexDirection::RowReverse as i32 as c_uint;
}
pub unsafe extern "C" fn YGNodeLeadingPosition(
    node: YGNodeRef,
    axis: FlexDirection,
    axisSize: c_float,
) -> c_float {
    if FlexDirectionIsRow(axis) {
        let mut leadingPosition: *const YGValue = YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            Edge::Start,
            &YGValueUndefined as *const YGValue,
        );
        if (*leadingPosition).unit as c_uint != YGUnitUndefined as i32 as c_uint {
            return YGResolveValue(leadingPosition, axisSize);
        };
    };
    let mut leadingPosition: *const YGValue = YGComputedEdgeValue(
        (*node).style.position.as_mut_ptr() as *const YGValue,
        leading[axis as usize],
        &YGValueUndefined as *const YGValue,
    );
    return if (*leadingPosition).unit as c_uint == YGUnitUndefined as i32 as c_uint {
        0.0f32
    } else {
        YGResolveValue(leadingPosition, axisSize)
    };
}
static mut leading: [Edge; 4] = [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right];
pub unsafe extern "C" fn YGNodeIsLeadingPosDefined(node: YGNodeRef, axis: FlexDirection) -> bool {
    return 0 != FlexDirectionIsRow(axis) as i32
        && (*YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            Edge::Start,
            &YGValueUndefined as *const YGValue,
        )).unit as c_uint != YGUnitUndefined as i32 as c_uint
        || (*YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            leading[axis as usize],
            &YGValueUndefined as *const YGValue,
        )).unit as c_uint != YGUnitUndefined as i32 as c_uint;
}
pub unsafe extern "C" fn YGNodeTrailingMargin(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    if 0 != FlexDirectionIsRow(axis) as i32
        && (*node).style.margin[Edge::End as i32 as usize].unit as c_uint
            != YGUnitUndefined as i32 as c_uint
    {
        return YGResolveValueMargin(
            &mut (*node).style.margin[Edge::End as i32 as usize] as *mut YGValue,
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
static mut YGValueZero: YGValue = YGValue_0 {
    value: 0i32 as c_float,
    unit: YGUnitPoint,
};
pub unsafe extern "C" fn YGResolveValueMargin(
    value: *const YGValue,
    parentSize: c_float,
) -> c_float {
    return if (*value).unit as c_uint == YGUnitAuto as i32 as c_uint {
        0i32 as c_float
    } else {
        YGResolveValue(value, parentSize)
    };
}
pub unsafe extern "C" fn YGNodeLeadingMargin(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    if 0 != FlexDirectionIsRow(axis) as i32
        && (*node).style.margin[Edge::Start as i32 as usize].unit as c_uint
            != YGUnitUndefined as i32 as c_uint
    {
        return YGResolveValueMargin(
            &mut (*node).style.margin[Edge::Start as i32 as usize] as *mut YGValue,
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

pub unsafe extern "C" fn YGLayoutNodeInternal(
    node: YGNodeRef,
    availableWidth: c_float,
    availableHeight: c_float,
    parentDirection: Direction,
    widthMeasureMode: MeasureMode,
    heightMeasureMode: MeasureMode,
    parentWidth: c_float,
    parentHeight: c_float,
    performLayout: bool,
    reason: &str,
    config: YGConfigRef,
) -> bool {
    trace!("layout for reason {} on node {:?}", reason, node);
    let mut layout: *mut YGLayout_0 = &mut (*node).layout as *mut YGLayout_0;
    gDepth = gDepth.wrapping_add(1);
    let needToVisitNode: bool = 0 != (*node).isDirty as i32
        && (*layout).generationCount != gCurrentGenerationCount
        || (*layout).lastParentDirection as c_uint != parentDirection as c_uint;
    if needToVisitNode {
        (*layout).nextCachedMeasurementsIndex = 0;
        (*layout).cachedLayout.widthMeasureMode = None;
        (*layout).cachedLayout.heightMeasureMode = None;
        (*layout).cachedLayout.computedWidth = -1i32 as c_float;
        (*layout).cachedLayout.computedHeight = -1i32 as c_float;
    };
    let mut cachedResults: *mut YGCachedMeasurement_0 = 0 as *mut YGCachedMeasurement_0;
    if (*node).measure.is_some() {
        let marginAxisRow: c_float = YGNodeMarginForAxis(node, FlexDirection::Row, parentWidth);
        let marginAxisColumn: c_float =
            YGNodeMarginForAxis(node, FlexDirection::Column, parentWidth);
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
            let mut i = 0usize;
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
            if 0 != YGFloatsEqual((*layout).cachedLayout.availableWidth, availableWidth) as i32
                && 0
                    != YGFloatsEqual((*layout).cachedLayout.availableHeight, availableHeight) as i32
                && (*layout).cachedLayout.widthMeasureMode == Some(widthMeasureMode)
                && (*layout).cachedLayout.heightMeasureMode == Some(heightMeasureMode)
            {
                cachedResults = &mut (*layout).cachedLayout as *mut YGCachedMeasurement_0;
            };
        } else {
            let mut i = 0usize;
            'loop2: while i < (*layout).nextCachedMeasurementsIndex {
                {
                    if 0 != YGFloatsEqual(
                        (*layout).cachedMeasurements[i as usize].availableWidth,
                        availableWidth,
                    ) as i32
                        && 0 != YGFloatsEqual(
                            (*layout).cachedMeasurements[i as usize].availableHeight,
                            availableHeight,
                        ) as i32
                        && (*layout).cachedMeasurements[i as usize].widthMeasureMode
                            == Some(widthMeasureMode)
                        && (*layout).cachedMeasurements[i as usize].heightMeasureMode
                            == Some(heightMeasureMode)
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
        (*layout).measuredDimensions.width = (*cachedResults).computedWidth;
        (*layout).measuredDimensions.height = (*cachedResults).computedHeight;
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
            if (*layout).nextCachedMeasurementsIndex == 16 {
                (*layout).nextCachedMeasurementsIndex = 0;
            };
            let mut newCacheEntry: *mut YGCachedMeasurement_0;
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
            (*newCacheEntry).widthMeasureMode = Some(widthMeasureMode);
            (*newCacheEntry).heightMeasureMode = Some(heightMeasureMode);
            (*newCacheEntry).computedWidth = (*layout).measuredDimensions.width;
            (*newCacheEntry).computedHeight = (*layout).measuredDimensions.height;
        };
    };
    if performLayout {
        (*node).layout.dimensions[Dimension::Width as usize] =
            (*node).layout.measuredDimensions.width;
        (*node).layout.dimensions[Dimension::Height as usize] =
            (*node).layout.measuredDimensions.height;
        (*node).hasNewLayout = 0 != 1i32;
        (*node).isDirty = 0 != 0i32;
    };
    gDepth = gDepth.wrapping_sub(1);
    (*layout).generationCount = gCurrentGenerationCount;
    return 0 != needToVisitNode as i32 || cachedResults.is_null();
}

static mut gCurrentGenerationCount: uint32_t = 0i32 as uint32_t;

static mut gDepth: uint32_t = 0i32 as uint32_t;
pub unsafe extern "C" fn MeasureModeName(mode: MeasureMode, performLayout: bool) -> *const c_char {
    let mut kMeasureModeNames: [*const c_char; 3] = [
        b"UNDEFINED\x00" as *const u8 as *const c_char,
        b"EXACTLY\x00" as *const u8 as *const c_char,
        b"AT_MOST\x00" as *const u8 as *const c_char,
    ];
    let mut kLayoutModeNames: [*const c_char; 3] = [
        b"LAY_UNDEFINED\x00" as *const u8 as *const c_char,
        b"LAY_EXACTLY\x00" as *const u8 as *const c_char,
        b"LAY_AT_MOST\x00" as *const u8 as *const c_char,
    ];
    if mode as c_uint >= 3i32 as c_uint {
        return b"\x00" as *const u8 as *const c_char;
    };
    return if 0 != performLayout as i32 {
        kLayoutModeNames[mode as usize]
    } else {
        kMeasureModeNames[mode as usize]
    };
}
pub unsafe extern "C" fn YGNodeResolveDirection(
    node: YGNodeRef,
    parentDirection: Direction,
) -> Direction {
    if (*node).style.direction == Direction::Inherit {
        return if parentDirection > Direction::Inherit {
            parentDirection
        } else {
            Direction::LTR
        };
    } else {
        return (*node).style.direction;
    };
}
pub unsafe extern "C" fn YGNodeSetChildTrailingPosition(
    node: YGNodeRef,
    child: YGNodeRef,
    axis: FlexDirection,
) -> () {
    let size: c_float = (*child).layout.measuredDimensions[DIM[axis as usize]];
    (*child).layout.position[trailing[axis as usize] as usize] = (*node).layout.measuredDimensions
        [DIM[axis as usize]] - size
        - (*child).layout.position[pos[axis as usize] as usize];
}
static mut pos: [Edge; 4] = [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right];
const DIM: [Dimension; 4] = [
    Dimension::Height,
    Dimension::Height,
    Dimension::Width,
    Dimension::Width,
];
pub unsafe extern "C" fn YGNodePaddingAndBorderForAxis(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    return YGNodeLeadingPaddingAndBorder(node, axis, widthSize)
        + YGNodeTrailingPaddingAndBorder(node, axis, widthSize);
}
pub unsafe extern "C" fn YGNodeTrailingPaddingAndBorder(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    return YGNodeTrailingPadding(node, axis, widthSize) + YGNodeTrailingBorder(node, axis);
}
pub unsafe extern "C" fn YGNodeTrailingBorder(node: YGNodeRef, axis: FlexDirection) -> c_float {
    if 0 != FlexDirectionIsRow(axis) as i32
        && (*node).style.border[Edge::End as i32 as usize].unit as c_uint
            != YGUnitUndefined as i32 as c_uint
        && (*node).style.border[Edge::End as i32 as usize].value >= 0.0f32
    {
        return (*node).style.border[Edge::End as i32 as usize].value;
    };
    return (*YGComputedEdgeValue(
        (*node).style.border.as_mut_ptr() as *const YGValue,
        trailing[axis as usize],
        &mut YGValueZero as *mut YGValue,
    )).value
        .max(0.0f32);
}
pub unsafe extern "C" fn YGNodeTrailingPadding(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    if 0 != FlexDirectionIsRow(axis) as i32
        && (*node).style.padding[Edge::End as i32 as usize].unit as c_uint
            != YGUnitUndefined as i32 as c_uint
        && YGResolveValue(
            &mut (*node).style.padding[Edge::End as i32 as usize] as *mut YGValue,
            widthSize,
        ) >= 0.0f32
    {
        return YGResolveValue(
            &mut (*node).style.padding[Edge::End as i32 as usize] as *mut YGValue,
            widthSize,
        );
    };
    return YGResolveValue(
        YGComputedEdgeValue(
            (*node).style.padding.as_mut_ptr() as *const YGValue,
            trailing[axis as usize],
            &mut YGValueZero as *mut YGValue,
        ),
        widthSize,
    ).max(0.0f32);
}
pub unsafe extern "C" fn YGNodeLeadingPaddingAndBorder(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    return YGNodeLeadingPadding(node, axis, widthSize) + YGNodeLeadingBorder(node, axis);
}
pub unsafe extern "C" fn YGNodeLeadingBorder(node: YGNodeRef, axis: FlexDirection) -> c_float {
    if 0 != FlexDirectionIsRow(axis) as i32
        && (*node).style.border[Edge::Start as i32 as usize].unit as c_uint
            != YGUnitUndefined as i32 as c_uint
        && (*node).style.border[Edge::Start as i32 as usize].value >= 0.0f32
    {
        return (*node).style.border[Edge::Start as i32 as usize].value;
    };
    return (*YGComputedEdgeValue(
        (*node).style.border.as_mut_ptr() as *const YGValue,
        leading[axis as usize],
        &mut YGValueZero as *mut YGValue,
    )).value
        .max(0.0f32);
}
pub unsafe extern "C" fn YGNodeLeadingPadding(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    if 0 != FlexDirectionIsRow(axis) as i32
        && (*node).style.padding[Edge::Start as i32 as usize].unit as c_uint
            != YGUnitUndefined as i32 as c_uint
        && YGResolveValue(
            &mut (*node).style.padding[Edge::Start as i32 as usize] as *mut YGValue,
            widthSize,
        ) >= 0.0f32
    {
        return YGResolveValue(
            &mut (*node).style.padding[Edge::Start as i32 as usize] as *mut YGValue,
            widthSize,
        );
    };
    return YGResolveValue(
        YGComputedEdgeValue(
            (*node).style.padding.as_mut_ptr() as *const YGValue,
            leading[axis as usize],
            &mut YGValueZero as *mut YGValue,
        ),
        widthSize,
    ).max(0.0f32);
}
pub unsafe extern "C" fn YGNodeMarginForAxis(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    return YGNodeLeadingMargin(node, axis, widthSize) + YGNodeTrailingMargin(node, axis, widthSize);
}
pub unsafe extern "C" fn YGNodeAbsoluteLayoutChild(
    node: YGNodeRef,
    child: YGNodeRef,
    width: c_float,
    widthMode: MeasureMode,
    height: c_float,
    direction: Direction,
    config: YGConfigRef,
) -> () {
    let mainAxis: FlexDirection = YGResolveFlexDirection((*node).style.flexDirection, direction);
    let crossAxis: FlexDirection = FlexDirectionCross(mainAxis, direction);
    let isMainAxisRow: bool = FlexDirectionIsRow(mainAxis);
    let mut childWidth: c_float = ::std::f32::NAN;
    let mut childHeight: c_float = ::std::f32::NAN;
    let mut childWidthMeasureMode: MeasureMode;
    let mut childHeightMeasureMode: MeasureMode;
    let marginRow: c_float = YGNodeMarginForAxis(child, FlexDirection::Row, width);
    let marginColumn: c_float = YGNodeMarginForAxis(child, FlexDirection::Column, width);
    if YGNodeIsStyleDimDefined(child, FlexDirection::Row, width) {
        childWidth = YGResolveValue((*child).resolvedDimensions.width, width) + marginRow;
    } else {
        if 0 != YGNodeIsLeadingPosDefined(child, FlexDirection::Row) as i32
            && 0 != YGNodeIsTrailingPosDefined(child, FlexDirection::Row) as i32
        {
            childWidth = (*node).layout.measuredDimensions.width
                - (YGNodeLeadingBorder(node, FlexDirection::Row)
                    + YGNodeTrailingBorder(node, FlexDirection::Row))
                - (YGNodeLeadingPosition(child, FlexDirection::Row, width)
                    + YGNodeTrailingPosition(child, FlexDirection::Row, width));
            childWidth = YGNodeBoundAxis(child, FlexDirection::Row, childWidth, width, width);
        };
    };
    if YGNodeIsStyleDimDefined(child, FlexDirection::Column, height) {
        childHeight = YGResolveValue((*child).resolvedDimensions.height, height) + marginColumn;
    } else {
        if 0 != YGNodeIsLeadingPosDefined(child, FlexDirection::Column) as i32
            && 0 != YGNodeIsTrailingPosDefined(child, FlexDirection::Column) as i32
        {
            childHeight = (*node).layout.measuredDimensions.height
                - (YGNodeLeadingBorder(node, FlexDirection::Column)
                    + YGNodeTrailingBorder(node, FlexDirection::Column))
                - (YGNodeLeadingPosition(child, FlexDirection::Column, height)
                    + YGNodeTrailingPosition(child, FlexDirection::Column, height));
            childHeight = YGNodeBoundAxis(child, FlexDirection::Column, childHeight, height, width);
        };
    };
    if childWidth.is_nan() || childHeight.is_nan() {
        if !(*child).style.aspectRatio.is_nan() {
            if childWidth.is_nan() {
                childWidth = marginRow + (childHeight - marginColumn) * (*child).style.aspectRatio;
            } else {
                if childHeight.is_nan() {
                    childHeight =
                        marginColumn + (childWidth - marginRow) / (*child).style.aspectRatio;
                };
            };
        };
    };
    if childWidth.is_nan() || childHeight.is_nan() {
        childWidthMeasureMode = if childWidth.is_nan() {
            MeasureMode::Undefined
        } else {
            MeasureMode::Exactly
        };
        childHeightMeasureMode = if childHeight.is_nan() {
            MeasureMode::Undefined
        } else {
            MeasureMode::Exactly
        };
        if !isMainAxisRow
            && childWidth.is_nan()
            && widthMode as c_uint != MeasureMode::Undefined as i32 as c_uint
            && width > 0i32 as c_float
        {
            childWidth = width;
            childWidthMeasureMode = MeasureMode::AtMost;
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
            "abs-measure",
            config,
        );
        childWidth = (*child).layout.measuredDimensions.width
            + YGNodeMarginForAxis(child, FlexDirection::Row, width);
        childHeight = (*child).layout.measuredDimensions.height
            + YGNodeMarginForAxis(child, FlexDirection::Column, width);
    };
    YGLayoutNodeInternal(
        child,
        childWidth,
        childHeight,
        direction,
        MeasureMode::Exactly,
        MeasureMode::Exactly,
        childWidth,
        childHeight,
        0 != 1i32,
        "abs-layout",
        config,
    );
    if 0 != YGNodeIsTrailingPosDefined(child, mainAxis) as i32
        && !YGNodeIsLeadingPosDefined(child, mainAxis)
    {
        (*child).layout.position[leading[mainAxis as usize] as usize] =
            (*node).layout.measuredDimensions[DIM[mainAxis as usize]]
                - (*child).layout.measuredDimensions[DIM[mainAxis as usize]]
                - YGNodeTrailingBorder(node, mainAxis)
                - YGNodeTrailingMargin(child, mainAxis, width)
                - YGNodeTrailingPosition(
                    child,
                    mainAxis,
                    if 0 != isMainAxisRow as i32 {
                        width
                    } else {
                        height
                    },
                );
    } else {
        if !YGNodeIsLeadingPosDefined(child, mainAxis)
            && (*node).style.justifyContent as c_uint == Justify::Center as i32 as c_uint
        {
            (*child).layout.position[leading[mainAxis as usize] as usize] =
                ((*node).layout.measuredDimensions[DIM[mainAxis as usize]]
                    - (*child).layout.measuredDimensions[DIM[mainAxis as usize]])
                    / 2.0f32;
        } else {
            if !YGNodeIsLeadingPosDefined(child, mainAxis)
                && (*node).style.justifyContent as c_uint == Justify::FlexEnd as i32 as c_uint
            {
                (*child).layout.position[leading[mainAxis as usize] as usize] =
                    (*node).layout.measuredDimensions[DIM[mainAxis as usize]]
                        - (*child).layout.measuredDimensions[DIM[mainAxis as usize]];
            };
        };
    };
    if 0 != YGNodeIsTrailingPosDefined(child, crossAxis) as i32
        && !YGNodeIsLeadingPosDefined(child, crossAxis)
    {
        (*child).layout.position[leading[crossAxis as usize] as usize] =
            (*node).layout.measuredDimensions[DIM[crossAxis as usize]]
                - (*child).layout.measuredDimensions[DIM[crossAxis as usize]]
                - YGNodeTrailingBorder(node, crossAxis)
                - YGNodeTrailingMargin(child, crossAxis, width)
                - YGNodeTrailingPosition(
                    child,
                    crossAxis,
                    if 0 != isMainAxisRow as i32 {
                        height
                    } else {
                        width
                    },
                );
    } else {
        if !YGNodeIsLeadingPosDefined(child, crossAxis)
            && YGNodeAlignItem(node, child) as c_uint == Align::Center as i32 as c_uint
        {
            (*child).layout.position[leading[crossAxis as usize] as usize] =
                ((*node).layout.measuredDimensions[DIM[crossAxis as usize]]
                    - (*child).layout.measuredDimensions[DIM[crossAxis as usize]])
                    / 2.0f32;
        } else {
            if !YGNodeIsLeadingPosDefined(child, crossAxis)
                && 0 != (YGNodeAlignItem(node, child) as c_uint == Align::FlexEnd as i32 as c_uint)
                    as i32
                    ^ ((*node).style.flexWrap as c_uint == YGWrapWrapReverse as i32 as c_uint)
                        as i32
            {
                (*child).layout.position[leading[crossAxis as usize] as usize] =
                    (*node).layout.measuredDimensions[DIM[crossAxis as usize]]
                        - (*child).layout.measuredDimensions[DIM[crossAxis as usize]];
            };
        };
    };
}
pub unsafe extern "C" fn YGNodeAlignItem(node: YGNodeRef, child: YGNodeRef) -> Align {
    let align: Align = if (*child).style.alignSelf == Align::Auto {
        (*node).style.alignItems
    } else {
        (*child).style.alignSelf
    };

    if align == Align::Baseline && 0 != FlexDirectionIsColumn((*node).style.flexDirection) as i32 {
        return Align::FlexStart;
    };
    return align;
}
pub unsafe extern "C" fn YGNodeIsTrailingPosDefined(node: YGNodeRef, axis: FlexDirection) -> bool {
    return 0 != FlexDirectionIsRow(axis) as i32
        && (*YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            Edge::End,
            &YGValueUndefined as *const YGValue,
        )).unit as c_uint != YGUnitUndefined as i32 as c_uint
        || (*YGComputedEdgeValue(
            (*node).style.position.as_mut_ptr() as *const YGValue,
            trailing[axis as usize],
            &YGValueUndefined as *const YGValue,
        )).unit as c_uint != YGUnitUndefined as i32 as c_uint;
}
pub unsafe extern "C" fn YGNodeBoundAxis(
    node: YGNodeRef,
    axis: FlexDirection,
    value: c_float,
    axisSize: c_float,
    widthSize: c_float,
) -> c_float {
    return YGNodeBoundAxisWithinMinAndMax(node, axis, value, axisSize)
        .max(YGNodePaddingAndBorderForAxis(node, axis, widthSize));
}
pub unsafe extern "C" fn YGNodeBoundAxisWithinMinAndMax(
    node: YGNodeRef,
    axis: FlexDirection,
    value: c_float,
    axisSize: c_float,
) -> c_float {
    let mut min: c_float = ::std::f32::NAN;
    let mut max: c_float = ::std::f32::NAN;
    if FlexDirectionIsColumn(axis) {
        min = YGResolveValue(
            &mut (*node).style.minDimensions[Dimension::Height] as *mut YGValue,
            axisSize,
        );
        max = YGResolveValue(
            &mut (*node).style.maxDimensions[Dimension::Height] as *mut YGValue,
            axisSize,
        );
    } else {
        if FlexDirectionIsRow(axis) {
            min = YGResolveValue(
                &mut (*node).style.minDimensions[Dimension::Width] as *mut YGValue,
                axisSize,
            );
            max = YGResolveValue(
                &mut (*node).style.maxDimensions[Dimension::Width] as *mut YGValue,
                axisSize,
            );
        };
    };
    let mut boundValue: c_float = value;
    if !max.is_nan() && max >= 0.0f32 && boundValue > max {
        boundValue = max;
    };
    if !min.is_nan() && min >= 0.0f32 && boundValue < min {
        boundValue = min;
    };
    return boundValue;
}
pub unsafe extern "C" fn YGNodeIsStyleDimDefined(
    node: YGNodeRef,
    axis: FlexDirection,
    parentSize: c_float,
) -> bool {
    return !((*(*node).resolvedDimensions[DIM[axis as usize]]).unit as c_uint
        == YGUnitAuto as i32 as c_uint
        || (*(*node).resolvedDimensions[DIM[axis as usize]]).unit as c_uint
            == YGUnitUndefined as i32 as c_uint
        || (*(*node).resolvedDimensions[DIM[axis as usize]]).unit as c_uint
            == YGUnitPoint as i32 as c_uint
            && (*(*node).resolvedDimensions[DIM[axis as usize]]).value < 0.0f32
        || (*(*node).resolvedDimensions[DIM[axis as usize]]).unit as c_uint
            == YGUnitPercent as i32 as c_uint
            && ((*(*node).resolvedDimensions[DIM[axis as usize]]).value < 0.0f32
                || parentSize.is_nan()));
}
pub unsafe extern "C" fn YGBaseline(node: YGNodeRef) -> c_float {
    if (*node).baseline.is_some() {
        let baseline: c_float = (*node).baseline.expect("non-null function pointer")(
            node,
            (*node).layout.measuredDimensions.width,
            (*node).layout.measuredDimensions.height,
        );
        YGAssertWithNode(
            node,
            !baseline.is_nan(),
            b"Expect custom baseline function to not return NaN\x00" as *const u8 as *const c_char,
        );
        return baseline;
    };
    let mut baselineChild: YGNodeRef = 0 as YGNodeRef;
    let childCount = YGNodeGetChildCount(node);
    {
        let mut i = 0usize;
        'loop5: while i < childCount {
            'body3: loop {
                {
                    let child: YGNodeRef = YGNodeGetChild(node, i);
                    if (*child).lineIndex > 0 {
                        break 'loop5;
                    };
                    if (*child).style.positionType as c_uint
                        == YGPositionTypeAbsolute as i32 as c_uint
                    {
                        break 'body3;
                    };
                    if YGNodeAlignItem(node, child) as c_uint == Align::Baseline as i32 as c_uint {
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
        return (*node).layout.measuredDimensions.height;
    };
    let baseline: c_float = YGBaseline(baselineChild);
    return baseline + (*baselineChild).layout.position[Edge::Top as i32 as usize];
}
pub unsafe extern "C" fn YGNodeIsLayoutDimDefined(node: YGNodeRef, axis: FlexDirection) -> bool {
    let value: c_float = (*node).layout.measuredDimensions[DIM[axis as usize]];
    return !value.is_nan() && value >= 0.0f32;
}
pub unsafe extern "C" fn YGIsBaselineLayout(node: YGNodeRef) -> bool {
    if FlexDirectionIsColumn((*node).style.flexDirection) {
        return 0 != 0i32;
    };
    if (*node).style.alignItems as c_uint == Align::Baseline as i32 as c_uint {
        return 0 != 1i32;
    };
    let childCount = YGNodeGetChildCount(node);
    {
        let mut i = 0;
        while i < childCount {
            {
                let child: YGNodeRef = YGNodeGetChild(node, i);
                if (*child).style.positionType as c_uint == YGPositionTypeRelative as i32 as c_uint
                    && (*child).style.alignSelf as c_uint == Align::Baseline as i32 as c_uint
                {
                    return 0 != 1i32;
                };
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 != 0i32;
}
pub unsafe extern "C" fn YGNodeDimWithMargin(
    node: YGNodeRef,
    axis: FlexDirection,
    widthSize: c_float,
) -> c_float {
    return (*node).layout.measuredDimensions[DIM[axis as usize]]
        + YGNodeLeadingMargin(node, axis, widthSize)
        + YGNodeTrailingMargin(node, axis, widthSize);
}
pub unsafe extern "C" fn YGMarginLeadingValue(
    node: YGNodeRef,
    axis: FlexDirection,
) -> *mut YGValue {
    if 0 != FlexDirectionIsRow(axis) as i32
        && (*node).style.margin[Edge::Start as i32 as usize].unit as c_uint
            != YGUnitUndefined as i32 as c_uint
    {
        return &mut (*node).style.margin[Edge::Start as i32 as usize] as *mut YGValue;
    } else {
        return &mut (*node).style.margin[leading[axis as usize] as usize] as *mut YGValue;
    };
}
pub unsafe extern "C" fn YGMarginTrailingValue(
    node: YGNodeRef,
    axis: FlexDirection,
) -> *mut YGValue {
    if 0 != FlexDirectionIsRow(axis) as i32
        && (*node).style.margin[Edge::End as i32 as usize].unit as c_uint
            != YGUnitUndefined as i32 as c_uint
    {
        return &mut (*node).style.margin[Edge::End as i32 as usize] as *mut YGValue;
    } else {
        return &mut (*node).style.margin[trailing[axis as usize] as usize] as *mut YGValue;
    };
}
pub unsafe extern "C" fn YGResolveFlexGrow(node: YGNodeRef) -> c_float {
    if (*node).parent.is_null() {
        return 0.0f64 as c_float;
    };
    if !(*node).style.flexGrow.is_nan() {
        return (*node).style.flexGrow;
    };
    if !(*node).style.flex.is_nan() && (*node).style.flex > 0.0f32 {
        return (*node).style.flex;
    };
    return kDefaultFlexGrow;
}
static mut kDefaultFlexGrow: c_float = 0.0f32;
pub unsafe extern "C" fn YGNodeResolveFlexShrink(node: YGNodeRef) -> c_float {
    if (*node).parent.is_null() {
        return 0.0f64 as c_float;
    };
    if !(*node).style.flexShrink.is_nan() {
        return (*node).style.flexShrink;
    };
    if !(*(*node).config).useWebDefaults
        && !(*node).style.flex.is_nan()
        && (*node).style.flex < 0.0f32
    {
        return -(*node).style.flex;
    };
    return if 0 != (*(*node).config).useWebDefaults as i32 {
        kWebDefaultFlexShrink
    } else {
        kDefaultFlexShrink
    };
}
static mut kDefaultFlexShrink: c_float = 0.0f32;
static mut kWebDefaultFlexShrink: c_float = 1.0f32;
pub unsafe extern "C" fn YGNodeIsFlex(node: YGNodeRef) -> bool {
    return (*node).style.positionType as c_uint == YGPositionTypeRelative as i32 as c_uint
        && (YGResolveFlexGrow(node) != 0i32 as c_float
            || YGNodeResolveFlexShrink(node) != 0i32 as c_float);
}
pub unsafe extern "C" fn YGNodeComputeFlexBasisForChild(
    node: YGNodeRef,
    child: YGNodeRef,
    width: c_float,
    widthMode: MeasureMode,
    height: c_float,
    parentWidth: c_float,
    parentHeight: c_float,
    heightMode: MeasureMode,
    direction: Direction,
    config: YGConfigRef,
) -> () {
    let mainAxis: FlexDirection = YGResolveFlexDirection((*node).style.flexDirection, direction);
    let isMainAxisRow: bool = FlexDirectionIsRow(mainAxis);
    let mainAxisSize: c_float = if 0 != isMainAxisRow as i32 {
        width
    } else {
        height
    };
    let mainAxisParentSize: c_float = if 0 != isMainAxisRow as i32 {
        parentWidth
    } else {
        parentHeight
    };
    let mut childWidth: c_float;
    let mut childHeight: c_float;
    let mut childWidthMeasureMode: MeasureMode;
    let mut childHeightMeasureMode: MeasureMode;
    let resolvedFlexBasis: c_float =
        YGResolveValue(YGNodeResolveFlexBasisPtr(child), mainAxisParentSize);
    let isRowStyleDimDefined: bool =
        YGNodeIsStyleDimDefined(child, FlexDirection::Row, parentWidth);
    let isColumnStyleDimDefined: bool =
        YGNodeIsStyleDimDefined(child, FlexDirection::Column, parentHeight);
    if !resolvedFlexBasis.is_nan() && !mainAxisSize.is_nan() {
        if (*child).layout.computedFlexBasis.is_nan()
            || 0 != YGConfigIsExperimentalFeatureEnabled(
                (*child).config,
                YGExperimentalFeatureWebFlexBasis,
            ) as i32
                && (*child).layout.computedFlexBasisGeneration != gCurrentGenerationCount
        {
            (*child).layout.computedFlexBasis =
                resolvedFlexBasis.max(YGNodePaddingAndBorderForAxis(child, mainAxis, parentWidth));
        };
    } else {
        if 0 != isMainAxisRow as i32 && 0 != isRowStyleDimDefined as i32 {
            (*child).layout.computedFlexBasis =
                YGResolveValue((*child).resolvedDimensions.width, parentWidth).max(
                    YGNodePaddingAndBorderForAxis(child, FlexDirection::Row, parentWidth),
                );
        } else {
            if !isMainAxisRow && 0 != isColumnStyleDimDefined as i32 {
                (*child).layout.computedFlexBasis =
                    YGResolveValue((*child).resolvedDimensions.height, parentHeight).max(
                        YGNodePaddingAndBorderForAxis(child, FlexDirection::Column, parentWidth),
                    );
            } else {
                childWidth = ::std::f32::NAN;
                childHeight = ::std::f32::NAN;
                childWidthMeasureMode = MeasureMode::Undefined;
                childHeightMeasureMode = MeasureMode::Undefined;
                let marginRow: c_float =
                    YGNodeMarginForAxis(child, FlexDirection::Row, parentWidth);
                let marginColumn: c_float =
                    YGNodeMarginForAxis(child, FlexDirection::Column, parentWidth);
                if isRowStyleDimDefined {
                    childWidth =
                        YGResolveValue((*child).resolvedDimensions.width, parentWidth) + marginRow;
                    childWidthMeasureMode = MeasureMode::Exactly;
                };
                if isColumnStyleDimDefined {
                    childHeight = YGResolveValue((*child).resolvedDimensions.height, parentHeight)
                        + marginColumn;
                    childHeightMeasureMode = MeasureMode::Exactly;
                };
                if !isMainAxisRow
                    && (*node).style.overflow as c_uint == YGOverflowScroll as i32 as c_uint
                    || (*node).style.overflow as c_uint != YGOverflowScroll as i32 as c_uint
                {
                    if childWidth.is_nan() && !width.is_nan() {
                        childWidth = width;
                        childWidthMeasureMode = MeasureMode::AtMost;
                    };
                };
                if 0 != isMainAxisRow as i32
                    && (*node).style.overflow as c_uint == YGOverflowScroll as i32 as c_uint
                    || (*node).style.overflow as c_uint != YGOverflowScroll as i32 as c_uint
                {
                    if childHeight.is_nan() && !height.is_nan() {
                        childHeight = height;
                        childHeightMeasureMode = MeasureMode::AtMost;
                    };
                };
                if !(*child).style.aspectRatio.is_nan() {
                    if !isMainAxisRow
                        && childWidthMeasureMode as c_uint == MeasureMode::Exactly as i32 as c_uint
                    {
                        childHeight = (childWidth - marginRow) / (*child).style.aspectRatio;
                        childHeightMeasureMode = MeasureMode::Exactly;
                    } else {
                        if 0 != isMainAxisRow as i32
                            && childHeightMeasureMode as c_uint
                                == MeasureMode::Exactly as i32 as c_uint
                        {
                            childWidth = (childHeight - marginColumn) * (*child).style.aspectRatio;
                            childWidthMeasureMode = MeasureMode::Exactly;
                        };
                    };
                };
                let hasExactWidth: bool =
                    !width.is_nan() && widthMode as c_uint == MeasureMode::Exactly as i32 as c_uint;
                let childWidthStretch: bool = YGNodeAlignItem(node, child) as c_uint
                    == Align::Stretch as i32 as c_uint
                    && childWidthMeasureMode as c_uint != MeasureMode::Exactly as i32 as c_uint;
                if !isMainAxisRow
                    && !isRowStyleDimDefined
                    && 0 != hasExactWidth as i32
                    && 0 != childWidthStretch as i32
                {
                    childWidth = width;
                    childWidthMeasureMode = MeasureMode::Exactly;
                    if !(*child).style.aspectRatio.is_nan() {
                        childHeight = (childWidth - marginRow) / (*child).style.aspectRatio;
                        childHeightMeasureMode = MeasureMode::Exactly;
                    };
                };
                let hasExactHeight: bool = !height.is_nan()
                    && heightMode as c_uint == MeasureMode::Exactly as i32 as c_uint;
                let childHeightStretch: bool = YGNodeAlignItem(node, child) as c_uint
                    == Align::Stretch as i32 as c_uint
                    && childHeightMeasureMode as c_uint != MeasureMode::Exactly as i32 as c_uint;
                if 0 != isMainAxisRow as i32
                    && !isColumnStyleDimDefined
                    && 0 != hasExactHeight as i32
                    && 0 != childHeightStretch as i32
                {
                    childHeight = height;
                    childHeightMeasureMode = MeasureMode::Exactly;
                    if !(*child).style.aspectRatio.is_nan() {
                        childWidth = (childHeight - marginColumn) * (*child).style.aspectRatio;
                        childWidthMeasureMode = MeasureMode::Exactly;
                    };
                };
                YGConstrainMaxSizeForMode(
                    child,
                    FlexDirection::Row,
                    parentWidth,
                    parentWidth,
                    &mut childWidthMeasureMode as *mut MeasureMode,
                    &mut childWidth as *mut c_float,
                );
                YGConstrainMaxSizeForMode(
                    child,
                    FlexDirection::Column,
                    parentHeight,
                    parentWidth,
                    &mut childHeightMeasureMode as *mut MeasureMode,
                    &mut childHeight as *mut c_float,
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
                    "measure",
                    config,
                );
                (*child).layout.computedFlexBasis = (*child).layout.measuredDimensions
                    [DIM[mainAxis as usize]]
                    .max(YGNodePaddingAndBorderForAxis(child, mainAxis, parentWidth));
            };
        };
    };
    (*child).layout.computedFlexBasisGeneration = gCurrentGenerationCount;
}
pub unsafe extern "C" fn YGNodeResolveFlexBasisPtr(node: YGNodeRef) -> *const YGValue {
    if (*node).style.flexBasis.unit as c_uint != YGUnitAuto as i32 as c_uint
        && (*node).style.flexBasis.unit as c_uint != YGUnitUndefined as i32 as c_uint
    {
        return &mut (*node).style.flexBasis as *mut YGValue;
    };
    if !(*node).style.flex.is_nan() && (*node).style.flex > 0.0f32 {
        return if 0 != (*(*node).config).useWebDefaults as i32 {
            &YGValueAuto as *const YGValue
        } else {
            &mut YGValueZero as *mut YGValue
        };
    };
    return &YGValueAuto as *const YGValue;
}

pub unsafe extern "C" fn YGConfigIsExperimentalFeatureEnabled(
    config: YGConfigRef,
    feature: YGExperimentalFeature_0,
) -> bool {
    return (*config).experimentalFeatures[feature as usize];
}

pub unsafe extern "C" fn YGResolveDimensions(mut node: YGNodeRef) -> () {
    for &dim in [Dimension::Width, Dimension::Height].into_iter() {
        if (*node).style.maxDimensions[dim].unit != YGUnitUndefined
            && (*node).style.maxDimensions[dim] != (*node).style.minDimensions[dim]
        {
            (*node).resolvedDimensions[dim] = &mut (*node).style.maxDimensions[dim] as *mut YGValue;
        } else {
            (*node).resolvedDimensions[dim] = &mut (*node).style.dimensions[dim] as *mut YGValue;
        };
    }
}

pub unsafe extern "C" fn YGZeroOutLayoutRecursivly(node: YGNodeRef) -> () {
    memset(
        &mut (*node).layout as *mut YGLayout_0 as *mut c_void,
        0i32,
        size_of::<YGLayout_0>(),
    );
    (*node).hasNewLayout = 0 != 1i32;
    YGCloneChildrenIfNeeded(node);
    let childCount = YGNodeGetChildCount(node);
    {
        let mut i = 0;
        while i < childCount {
            {
                let child: YGNodeRef = YGNodeListGet((*node).children, i);
                YGZeroOutLayoutRecursivly(child);
            }
            i = i.wrapping_add(1);
        }
    };
}
pub unsafe extern "C" fn YGNodeFixedSizeSetMeasuredDimensions(
    node: YGNodeRef,
    availableWidth: c_float,
    availableHeight: c_float,
    widthMeasureMode: MeasureMode,
    heightMeasureMode: MeasureMode,
    parentWidth: c_float,
    parentHeight: c_float,
) -> bool {
    if widthMeasureMode as c_uint == MeasureMode::AtMost as i32 as c_uint
        && availableWidth <= 0.0f32
        || heightMeasureMode as c_uint == MeasureMode::AtMost as i32 as c_uint
            && availableHeight <= 0.0f32
        || widthMeasureMode as c_uint == MeasureMode::Exactly as i32 as c_uint
            && heightMeasureMode as c_uint == MeasureMode::Exactly as i32 as c_uint
    {
        let marginAxisColumn: c_float =
            YGNodeMarginForAxis(node, FlexDirection::Column, parentWidth);
        let marginAxisRow: c_float = YGNodeMarginForAxis(node, FlexDirection::Row, parentWidth);
        (*node).layout.measuredDimensions.width = YGNodeBoundAxis(
            node,
            FlexDirection::Row,
            if availableWidth.is_nan()
                || widthMeasureMode as c_uint == MeasureMode::AtMost as i32 as c_uint
                    && availableWidth < 0.0f32
            {
                0.0f32
            } else {
                availableWidth - marginAxisRow
            },
            parentWidth,
            parentWidth,
        );
        (*node).layout.measuredDimensions.height = YGNodeBoundAxis(
            node,
            FlexDirection::Column,
            if availableHeight.is_nan()
                || heightMeasureMode as c_uint == MeasureMode::AtMost as i32 as c_uint
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
pub unsafe extern "C" fn YGNodeEmptyContainerSetMeasuredDimensions(
    node: YGNodeRef,
    availableWidth: c_float,
    availableHeight: c_float,
    widthMeasureMode: MeasureMode,
    heightMeasureMode: MeasureMode,
    parentWidth: c_float,
    parentHeight: c_float,
) -> () {
    let paddingAndBorderAxisRow: c_float =
        YGNodePaddingAndBorderForAxis(node, FlexDirection::Row, parentWidth);
    let paddingAndBorderAxisColumn: c_float =
        YGNodePaddingAndBorderForAxis(node, FlexDirection::Column, parentWidth);
    let marginAxisRow: c_float = YGNodeMarginForAxis(node, FlexDirection::Row, parentWidth);
    let marginAxisColumn: c_float = YGNodeMarginForAxis(node, FlexDirection::Column, parentWidth);
    (*node).layout.measuredDimensions.width = YGNodeBoundAxis(
        node,
        FlexDirection::Row,
        if widthMeasureMode as c_uint == MeasureMode::Undefined as i32 as c_uint
            || widthMeasureMode as c_uint == MeasureMode::AtMost as i32 as c_uint
        {
            paddingAndBorderAxisRow
        } else {
            availableWidth - marginAxisRow
        },
        parentWidth,
        parentWidth,
    );
    (*node).layout.measuredDimensions.height = YGNodeBoundAxis(
        node,
        FlexDirection::Column,
        if heightMeasureMode as c_uint == MeasureMode::Undefined as i32 as c_uint
            || heightMeasureMode as c_uint == MeasureMode::AtMost as i32 as c_uint
        {
            paddingAndBorderAxisColumn
        } else {
            availableHeight - marginAxisColumn
        },
        parentHeight,
        parentWidth,
    );
}
pub unsafe extern "C" fn YGNodeWithMeasureFuncSetMeasuredDimensions(
    node: YGNodeRef,
    availableWidth: c_float,
    availableHeight: c_float,
    widthMeasureMode: MeasureMode,
    heightMeasureMode: MeasureMode,
    parentWidth: c_float,
    parentHeight: c_float,
) -> () {
    YGAssertWithNode(
        node,
        (*node).measure.is_some(),
        b"Expected node to have custom measure function\x00" as *const u8 as *const c_char,
    );
    let paddingAndBorderAxisRow: c_float =
        YGNodePaddingAndBorderForAxis(node, FlexDirection::Row, availableWidth);
    let paddingAndBorderAxisColumn: c_float =
        YGNodePaddingAndBorderForAxis(node, FlexDirection::Column, availableWidth);
    let marginAxisRow: c_float = YGNodeMarginForAxis(node, FlexDirection::Row, availableWidth);
    let marginAxisColumn: c_float =
        YGNodeMarginForAxis(node, FlexDirection::Column, availableWidth);
    let innerWidth: c_float = if availableWidth.is_nan() {
        availableWidth
    } else {
        (0i32 as c_float).max(availableWidth - marginAxisRow - paddingAndBorderAxisRow)
    };
    let innerHeight: c_float = if availableHeight.is_nan() {
        availableHeight
    } else {
        (0i32 as c_float).max(availableHeight - marginAxisColumn - paddingAndBorderAxisColumn)
    };
    if widthMeasureMode as c_uint == MeasureMode::Exactly as i32 as c_uint
        && heightMeasureMode as c_uint == MeasureMode::Exactly as i32 as c_uint
    {
        (*node).layout.measuredDimensions.width = YGNodeBoundAxis(
            node,
            FlexDirection::Row,
            availableWidth - marginAxisRow,
            parentWidth,
            parentWidth,
        );
        (*node).layout.measuredDimensions.height = YGNodeBoundAxis(
            node,
            FlexDirection::Column,
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
        (*node).layout.measuredDimensions.width = YGNodeBoundAxis(
            node,
            FlexDirection::Row,
            if widthMeasureMode as c_uint == MeasureMode::Undefined as i32 as c_uint
                || widthMeasureMode as c_uint == MeasureMode::AtMost as i32 as c_uint
            {
                measuredSize.width + paddingAndBorderAxisRow
            } else {
                availableWidth - marginAxisRow
            },
            availableWidth,
            availableWidth,
        );
        (*node).layout.measuredDimensions.height = YGNodeBoundAxis(
            node,
            FlexDirection::Column,
            if heightMeasureMode as c_uint == MeasureMode::Undefined as i32 as c_uint
                || heightMeasureMode as c_uint == MeasureMode::AtMost as i32 as c_uint
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

pub unsafe extern "C" fn YGNodeCanUseCachedMeasurement(
    widthMode: MeasureMode,
    width: c_float,
    heightMode: MeasureMode,
    height: c_float,
    lastWidthMode: Option<MeasureMode>,
    lastWidth: c_float,
    lastHeightMode: Option<MeasureMode>,
    lastHeight: c_float,
    lastComputedWidth: c_float,
    lastComputedHeight: c_float,
    marginRow: c_float,
    marginColumn: c_float,
    config: YGConfigRef,
) -> bool {
    if lastComputedHeight < 0i32 as c_float || lastComputedWidth < 0i32 as c_float {
        return 0 != 0i32;
    };
    let mut useRoundedComparison: bool =
        !config.is_null() && (*config).pointScaleFactor != 0i32 as c_float;
    let effectiveWidth: c_float = if 0 != useRoundedComparison as i32 {
        YGRoundValueToPixelGrid(width, (*config).pointScaleFactor, 0 != 0i32, 0 != 0i32)
    } else {
        width
    };
    let effectiveHeight: c_float = if 0 != useRoundedComparison as i32 {
        YGRoundValueToPixelGrid(height, (*config).pointScaleFactor, 0 != 0i32, 0 != 0i32)
    } else {
        height
    };
    let effectiveLastWidth: c_float = if 0 != useRoundedComparison as i32 {
        YGRoundValueToPixelGrid(lastWidth, (*config).pointScaleFactor, 0 != 0i32, 0 != 0i32)
    } else {
        lastWidth
    };
    let effectiveLastHeight: c_float = if 0 != useRoundedComparison as i32 {
        YGRoundValueToPixelGrid(lastHeight, (*config).pointScaleFactor, 0 != 0i32, 0 != 0i32)
    } else {
        lastHeight
    };
    let hasSameWidthSpec: bool = lastWidthMode == Some(widthMode)
        && 0 != YGFloatsEqual(effectiveLastWidth, effectiveWidth) as i32;
    let hasSameHeightSpec: bool = lastHeightMode == Some(heightMode)
        && 0 != YGFloatsEqual(effectiveLastHeight, effectiveHeight) as i32;
    let widthIsCompatible: bool = 0 != hasSameWidthSpec as i32
        || 0 != MeasureModeSizeIsExactAndMatchesOldMeasuredSize(
            widthMode,
            width - marginRow,
            lastComputedWidth,
        ) as i32
        || 0 != MeasureModeOldSizeIsUnspecifiedAndStillFits(
            widthMode,
            width - marginRow,
            lastWidthMode.unwrap(),
            lastComputedWidth,
        ) as i32
        || 0 != MeasureModeNewMeasureSizeIsStricterAndStillValid(
            widthMode,
            width - marginRow,
            lastWidthMode.unwrap(),
            lastWidth,
            lastComputedWidth,
        ) as i32;
    let heightIsCompatible: bool = 0 != hasSameHeightSpec as i32
        || 0 != MeasureModeSizeIsExactAndMatchesOldMeasuredSize(
            heightMode,
            height - marginColumn,
            lastComputedHeight,
        ) as i32
        || 0 != MeasureModeOldSizeIsUnspecifiedAndStillFits(
            heightMode,
            height - marginColumn,
            lastHeightMode.unwrap(),
            lastComputedHeight,
        ) as i32
        || 0 != MeasureModeNewMeasureSizeIsStricterAndStillValid(
            heightMode,
            height - marginColumn,
            lastHeightMode.unwrap(),
            lastHeight,
            lastComputedHeight,
        ) as i32;
    return 0 != widthIsCompatible as i32 && 0 != heightIsCompatible as i32;
}
pub unsafe extern "C" fn MeasureModeNewMeasureSizeIsStricterAndStillValid(
    mut sizeMode: MeasureMode,
    mut size: c_float,
    mut lastSizeMode: MeasureMode,
    mut lastSize: c_float,
    mut lastComputedSize: c_float,
) -> bool {
    return lastSizeMode as c_uint == MeasureMode::AtMost as i32 as c_uint
        && sizeMode as c_uint == MeasureMode::AtMost as i32 as c_uint
        && lastSize > size
        && (lastComputedSize <= size || 0 != YGFloatsEqual(size, lastComputedSize) as i32);
}
pub unsafe extern "C" fn MeasureModeOldSizeIsUnspecifiedAndStillFits(
    mut sizeMode: MeasureMode,
    mut size: c_float,
    mut lastSizeMode: MeasureMode,
    mut lastComputedSize: c_float,
) -> bool {
    return sizeMode as c_uint == MeasureMode::AtMost as i32 as c_uint
        && lastSizeMode as c_uint == MeasureMode::Undefined as i32 as c_uint
        && (size >= lastComputedSize || 0 != YGFloatsEqual(size, lastComputedSize) as i32);
}
pub unsafe extern "C" fn MeasureModeSizeIsExactAndMatchesOldMeasuredSize(
    mut sizeMode: MeasureMode,
    mut size: c_float,
    mut lastComputedSize: c_float,
) -> bool {
    return sizeMode as c_uint == MeasureMode::Exactly as i32 as c_uint
        && 0 != YGFloatsEqual(size, lastComputedSize) as i32;
}

pub unsafe extern "C" fn YGNodeMarkDirty(node: YGNodeRef) -> () {
    YGAssertWithNode(
        node,
        (*node).measure.is_some(),
        b"Only leaf nodes with custom measure functionsshould manually mark themselves as dirty\x00"
            as *const u8 as *const c_char,
    );
    YGNodeMarkDirtyInternal(node);
}

pub unsafe extern "C" fn YGNodeIsDirty(node: YGNodeRef) -> bool {
    return (*node).isDirty;
}

pub unsafe extern "C" fn YGNodeCopyStyle(dstNode: YGNodeRef, srcNode: YGNodeRef) -> () {
    if memcmp(
        &mut (*dstNode).style as *mut YGStyle_0 as *const c_void,
        &mut (*srcNode).style as *mut YGStyle_0 as *const c_void,
        size_of::<YGStyle_0>(),
    ) != 0i32
    {
        memcpy(
            &mut (*dstNode).style as *mut YGStyle_0 as *mut c_void,
            &mut (*srcNode).style as *mut YGStyle_0 as *const c_void,
            size_of::<YGStyle_0>(),
        );
        YGNodeMarkDirtyInternal(dstNode);
    };
}

pub unsafe extern "C" fn YGNodeSetContext(node: YGNodeRef, mut context: *mut c_void) -> () {
    (*node).context = context;
}

pub unsafe extern "C" fn YGNodeGetContext(node: YGNodeRef) -> *mut c_void {
    return (*node).context;
}

pub unsafe extern "C" fn YGNodeSetMeasureFunc(
    node: YGNodeRef,
    mut measureFunc: YGMeasureFunc,
) -> () {
    if measureFunc.is_none() {
        (*node).measure = None;
        (*node).nodeType = NodeType::Default;
    } else {
        YGAssertWithNode(
            node,
            YGNodeGetChildCount(node) == 0,
            b"Cannot set measure function: Nodes with measure functions cannot have children.\x00"
                as *const u8 as *const c_char,
        );
        (*node).measure = measureFunc;
        (*node).nodeType = NodeType::Text;
    };
}

pub unsafe extern "C" fn YGNodeGetMeasureFunc(node: YGNodeRef) -> YGMeasureFunc {
    return (*node).measure;
}

pub unsafe extern "C" fn YGNodeSetBaselineFunc(
    node: YGNodeRef,
    mut baselineFunc: YGBaselineFunc,
) -> () {
    (*node).baseline = baselineFunc;
}

pub unsafe extern "C" fn YGNodeGetBaselineFunc(node: YGNodeRef) -> YGBaselineFunc {
    return (*node).baseline;
}

pub unsafe extern "C" fn YGNodeSetHasNewLayout(node: YGNodeRef, mut hasNewLayout: bool) -> () {
    (*node).hasNewLayout = hasNewLayout;
}

pub unsafe extern "C" fn YGNodeGetHasNewLayout(node: YGNodeRef) -> bool {
    return (*node).hasNewLayout;
}

pub unsafe extern "C" fn YGNodeSetNodeType(node: YGNodeRef, mut nodeType: NodeType) -> () {
    (*node).nodeType = nodeType;
}

pub unsafe extern "C" fn YGNodeGetNodeType(node: YGNodeRef) -> NodeType {
    return (*node).nodeType;
}

pub unsafe extern "C" fn YGNodeStyleSetDirection(node: YGNodeRef, direction: Direction) -> () {
    if (*node).style.direction as c_uint != direction as c_uint {
        (*node).style.direction = direction;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetDirection(node: YGNodeRef) -> Direction {
    return (*node).style.direction;
}

pub unsafe extern "C" fn YGNodeStyleSetFlexDirection(
    node: YGNodeRef,
    flexDirection: FlexDirection,
) -> () {
    if (*node).style.flexDirection as c_uint != flexDirection as c_uint {
        (*node).style.flexDirection = flexDirection;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetFlexDirection(node: YGNodeRef) -> FlexDirection {
    return (*node).style.flexDirection;
}

pub unsafe extern "C" fn YGNodeStyleSetJustifyContent(
    node: YGNodeRef,
    justifyContent: Justify,
) -> () {
    if (*node).style.justifyContent as c_uint != justifyContent as c_uint {
        (*node).style.justifyContent = justifyContent;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetJustifyContent(node: YGNodeRef) -> Justify {
    return (*node).style.justifyContent;
}

pub unsafe extern "C" fn YGNodeStyleSetAlignContent(node: YGNodeRef, alignContent: Align) -> () {
    if (*node).style.alignContent as c_uint != alignContent as c_uint {
        (*node).style.alignContent = alignContent;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetAlignContent(node: YGNodeRef) -> Align {
    return (*node).style.alignContent;
}

pub unsafe extern "C" fn YGNodeStyleSetAlignItems(node: YGNodeRef, alignItems: Align) -> () {
    if (*node).style.alignItems as c_uint != alignItems as c_uint {
        (*node).style.alignItems = alignItems;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetAlignItems(node: YGNodeRef) -> Align {
    return (*node).style.alignItems;
}

pub unsafe extern "C" fn YGNodeStyleSetAlignSelf(node: YGNodeRef, alignSelf: Align) -> () {
    if (*node).style.alignSelf as c_uint != alignSelf as c_uint {
        (*node).style.alignSelf = alignSelf;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetAlignSelf(node: YGNodeRef) -> Align {
    return (*node).style.alignSelf;
}

pub unsafe extern "C" fn YGNodeStyleSetPositionType(
    node: YGNodeRef,
    positionType: YGPositionType,
) -> () {
    if (*node).style.positionType as c_uint != positionType as c_uint {
        (*node).style.positionType = positionType;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetPositionType(node: YGNodeRef) -> YGPositionType {
    return (*node).style.positionType;
}

pub unsafe extern "C" fn YGNodeStyleSetFlexWrap(node: YGNodeRef, flexWrap: YGWrap_0) -> () {
    if (*node).style.flexWrap as c_uint != flexWrap as c_uint {
        (*node).style.flexWrap = flexWrap;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetFlexWrap(node: YGNodeRef) -> YGWrap_0 {
    return (*node).style.flexWrap;
}

pub unsafe extern "C" fn YGNodeStyleSetOverflow(node: YGNodeRef, overflow: YGOverflow_0) -> () {
    if (*node).style.overflow as c_uint != overflow as c_uint {
        (*node).style.overflow = overflow;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetOverflow(node: YGNodeRef) -> YGOverflow_0 {
    return (*node).style.overflow;
}

pub unsafe extern "C" fn YGNodeStyleSetDisplay(node: YGNodeRef, display: Display) -> () {
    if (*node).style.display as c_uint != display as c_uint {
        (*node).style.display = display;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetDisplay(node: YGNodeRef) -> Display {
    return (*node).style.display;
}

pub unsafe extern "C" fn YGNodeStyleSetFlex(node: YGNodeRef, flex: c_float) -> () {
    if (*node).style.flex != flex {
        (*node).style.flex = flex;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetFlex(node: YGNodeRef) -> c_float {
    return (*node).style.flex;
}

pub unsafe extern "C" fn YGNodeStyleSetFlexGrow(node: YGNodeRef, flexGrow: c_float) -> () {
    if (*node).style.flexGrow != flexGrow {
        (*node).style.flexGrow = flexGrow;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetFlexGrow(node: YGNodeRef) -> c_float {
    return if 0 != (*node).style.flexGrow.is_nan() as i32 {
        kDefaultFlexGrow
    } else {
        (*node).style.flexGrow
    };
}

pub unsafe extern "C" fn YGNodeStyleSetFlexShrink(node: YGNodeRef, flexShrink: c_float) -> () {
    if (*node).style.flexShrink != flexShrink {
        (*node).style.flexShrink = flexShrink;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetFlexShrink(node: YGNodeRef) -> c_float {
    return if (*node).style.flexShrink.is_nan() {
        if 0 != (*(*node).config).useWebDefaults as i32 {
            kWebDefaultFlexShrink
        } else {
            kDefaultFlexShrink
        }
    } else {
        (*node).style.flexShrink
    };
}

pub unsafe extern "C" fn YGNodeStyleSetFlexBasis(node: YGNodeRef, flexBasis: c_float) -> () {
    if (*node).style.flexBasis.value != flexBasis
        || (*node).style.flexBasis.unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.flexBasis.value = flexBasis;
        (*node).style.flexBasis.unit = (if flexBasis.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetFlexBasisPercent(node: YGNodeRef, flexBasis: c_float) -> () {
    if (*node).style.flexBasis.value != flexBasis
        || (*node).style.flexBasis.unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.flexBasis.value = flexBasis;
        (*node).style.flexBasis.unit = (if flexBasis.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetFlexBasis(node: YGNodeRef) -> YGValue {
    return (*node).style.flexBasis;
}

pub unsafe extern "C" fn YGNodeStyleSetFlexBasisAuto(node: YGNodeRef) -> () {
    if (*node).style.flexBasis.unit as c_uint != YGUnitAuto as i32 as c_uint {
        (*node).style.flexBasis.value = ::std::f32::NAN;
        (*node).style.flexBasis.unit = YGUnitAuto;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetPosition(
    node: YGNodeRef,
    edge: Edge,
    position: c_float,
) -> () {
    if (*node).style.position[edge as usize].value != position
        || (*node).style.position[edge as usize].unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.position[edge as usize].value = position;
        (*node).style.position[edge as usize].unit = (if position.is_nan() {
            YGUnitUndefined as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetPositionPercent(
    node: YGNodeRef,
    edge: Edge,
    position: c_float,
) -> () {
    if (*node).style.position[edge as usize].value != position
        || (*node).style.position[edge as usize].unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.position[edge as usize].value = position;
        (*node).style.position[edge as usize].unit = (if position.is_nan() {
            YGUnitUndefined as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetPosition(node: YGNodeRef, edge: Edge) -> YGValue {
    return (*node).style.position[edge as usize];
}

pub unsafe extern "C" fn YGNodeStyleSetMargin(node: YGNodeRef, edge: Edge, margin: c_float) -> () {
    if (*node).style.margin[edge as usize].value != margin
        || (*node).style.margin[edge as usize].unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.margin[edge as usize].value = margin;
        (*node).style.margin[edge as usize].unit = (if margin.is_nan() {
            YGUnitUndefined as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetMarginPercent(
    node: YGNodeRef,
    edge: Edge,
    margin: c_float,
) -> () {
    if (*node).style.margin[edge as usize].value != margin
        || (*node).style.margin[edge as usize].unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.margin[edge as usize].value = margin;
        (*node).style.margin[edge as usize].unit = (if margin.is_nan() {
            YGUnitUndefined as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetMargin(node: YGNodeRef, edge: Edge) -> YGValue {
    return (*node).style.margin[edge as usize];
}

pub unsafe extern "C" fn YGNodeStyleSetMarginAuto(node: YGNodeRef, edge: Edge) -> () {
    if (*node).style.margin[edge as usize].unit as c_uint != YGUnitAuto as i32 as c_uint {
        (*node).style.margin[edge as usize].value = ::std::f32::NAN;
        (*node).style.margin[edge as usize].unit = YGUnitAuto;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetPadding(
    node: YGNodeRef,
    edge: Edge,
    padding: c_float,
) -> () {
    if (*node).style.padding[edge as usize].value != padding
        || (*node).style.padding[edge as usize].unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.padding[edge as usize].value = padding;
        (*node).style.padding[edge as usize].unit = (if padding.is_nan() {
            YGUnitUndefined as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetPaddingPercent(
    node: YGNodeRef,
    edge: Edge,
    padding: c_float,
) -> () {
    if (*node).style.padding[edge as usize].value != padding
        || (*node).style.padding[edge as usize].unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.padding[edge as usize].value = padding;
        (*node).style.padding[edge as usize].unit = (if padding.is_nan() {
            YGUnitUndefined as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetPadding(node: YGNodeRef, edge: Edge) -> YGValue {
    return (*node).style.padding[edge as usize];
}

pub unsafe extern "C" fn YGNodeStyleSetBorder(node: YGNodeRef, edge: Edge, border: c_float) -> () {
    if (*node).style.border[edge as usize].value != border
        || (*node).style.border[edge as usize].unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.border[edge as usize].value = border;
        (*node).style.border[edge as usize].unit = (if border.is_nan() {
            YGUnitUndefined as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetBorder(node: YGNodeRef, edge: Edge) -> c_float {
    return (*node).style.border[edge as usize].value;
}

pub unsafe extern "C" fn YGNodeStyleSetWidth(node: YGNodeRef, width: c_float) -> () {
    if (*node).style.dimensions.width.value != width
        || (*node).style.dimensions.width.unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.dimensions.width.value = width;
        (*node).style.dimensions.width.unit = (if width.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetWidthPercent(node: YGNodeRef, width: c_float) -> () {
    if (*node).style.dimensions.width.value != width
        || (*node).style.dimensions.width.unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.dimensions.width.value = width;
        (*node).style.dimensions.width.unit = (if width.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetWidth(node: YGNodeRef) -> YGValue {
    return (*node).style.dimensions.width;
}

pub unsafe extern "C" fn YGNodeStyleSetWidthAuto(node: YGNodeRef) -> () {
    if (*node).style.dimensions.width.unit as c_uint != YGUnitAuto as i32 as c_uint {
        (*node).style.dimensions.width.value = ::std::f32::NAN;
        (*node).style.dimensions.width.unit = YGUnitAuto;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetHeight(node: YGNodeRef, height: c_float) -> () {
    if (*node).style.dimensions.height.value != height
        || (*node).style.dimensions.height.unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.dimensions.height.value = height;
        (*node).style.dimensions.height.unit = (if height.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetHeightPercent(node: YGNodeRef, height: c_float) -> () {
    if (*node).style.dimensions.height.value != height
        || (*node).style.dimensions.height.unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.dimensions.height.value = height;
        (*node).style.dimensions.height.unit = (if height.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetHeight(node: YGNodeRef) -> YGValue {
    return (*node).style.dimensions.height;
}

pub unsafe extern "C" fn YGNodeStyleSetHeightAuto(node: YGNodeRef) -> () {
    if (*node).style.dimensions.height.unit as c_uint != YGUnitAuto as i32 as c_uint {
        (*node).style.dimensions.height.value = ::std::f32::NAN;
        (*node).style.dimensions.height.unit = YGUnitAuto;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetMinWidth(node: YGNodeRef, minWidth: c_float) -> () {
    if (*node).style.minDimensions.width.value != minWidth
        || (*node).style.minDimensions.width.unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.minDimensions.width.value = minWidth;
        (*node).style.minDimensions.width.unit = (if minWidth.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetMinWidthPercent(node: YGNodeRef, minWidth: c_float) -> () {
    if (*node).style.minDimensions.width.value != minWidth
        || (*node).style.minDimensions.width.unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.minDimensions.width.value = minWidth;
        (*node).style.minDimensions.width.unit = (if minWidth.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetMinWidth(node: YGNodeRef) -> YGValue {
    return (*node).style.minDimensions.width;
}

pub unsafe extern "C" fn YGNodeStyleSetMinHeight(node: YGNodeRef, minHeight: c_float) -> () {
    if (*node).style.minDimensions.height.value != minHeight
        || (*node).style.minDimensions.height.unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.minDimensions.height.value = minHeight;
        (*node).style.minDimensions.height.unit = (if minHeight.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetMinHeightPercent(node: YGNodeRef, minHeight: c_float) -> () {
    if (*node).style.minDimensions.height.value != minHeight
        || (*node).style.minDimensions.height.unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.minDimensions.height.value = minHeight;
        (*node).style.minDimensions.height.unit = (if minHeight.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetMinHeight(node: YGNodeRef) -> YGValue {
    return (*node).style.minDimensions.height;
}

pub unsafe extern "C" fn YGNodeStyleSetMaxWidth(node: YGNodeRef, maxWidth: c_float) -> () {
    if (*node).style.maxDimensions.width.value != maxWidth
        || (*node).style.maxDimensions.width.unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.maxDimensions[Dimension::Width].value = maxWidth;
        (*node).style.maxDimensions[Dimension::Width].unit = (if maxWidth.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetMaxWidthPercent(node: YGNodeRef, maxWidth: c_float) -> () {
    if (*node).style.maxDimensions.width.value != maxWidth
        || (*node).style.maxDimensions.width.unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.maxDimensions.width.value = maxWidth;
        (*node).style.maxDimensions.width.unit = (if maxWidth.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetMaxWidth(node: YGNodeRef) -> YGValue {
    return (*node).style.maxDimensions.width;
}

pub unsafe extern "C" fn YGNodeStyleSetMaxHeight(node: YGNodeRef, maxHeight: c_float) -> () {
    if (*node).style.maxDimensions.height.value != maxHeight
        || (*node).style.maxDimensions.height.unit as c_uint != YGUnitPoint as i32 as c_uint
    {
        (*node).style.maxDimensions.height.value = maxHeight;
        (*node).style.maxDimensions.height.unit = (if maxHeight.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPoint as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleSetMaxHeightPercent(node: YGNodeRef, maxHeight: c_float) -> () {
    if (*node).style.maxDimensions.height.value != maxHeight
        || (*node).style.maxDimensions.height.unit as c_uint != YGUnitPercent as i32 as c_uint
    {
        (*node).style.maxDimensions.height.value = maxHeight;
        (*node).style.maxDimensions.height.unit = (if maxHeight.is_nan() {
            YGUnitAuto as i32
        } else {
            YGUnitPercent as i32
        }) as YGUnit_0;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetMaxHeight(node: YGNodeRef) -> YGValue {
    return (*node).style.maxDimensions.height;
}

pub unsafe extern "C" fn YGNodeStyleSetAspectRatio(node: YGNodeRef, aspectRatio: c_float) -> () {
    if (*node).style.aspectRatio != aspectRatio {
        (*node).style.aspectRatio = aspectRatio;
        YGNodeMarkDirtyInternal(node);
    };
}

pub unsafe extern "C" fn YGNodeStyleGetAspectRatio(node: YGNodeRef) -> c_float {
    return (*node).style.aspectRatio;
}

pub unsafe extern "C" fn YGNodeLayoutGetLeft(node: YGNodeRef) -> c_float {
    return (*node).layout.position[Edge::Left as i32 as usize];
}

pub unsafe extern "C" fn YGNodeLayoutGetTop(node: YGNodeRef) -> c_float {
    return (*node).layout.position[Edge::Top as i32 as usize];
}

pub unsafe extern "C" fn YGNodeLayoutGetRight(node: YGNodeRef) -> c_float {
    return (*node).layout.position[Edge::Right as i32 as usize];
}

pub unsafe extern "C" fn YGNodeLayoutGetBottom(node: YGNodeRef) -> c_float {
    return (*node).layout.position[Edge::Bottom as i32 as usize];
}

pub unsafe extern "C" fn YGNodeLayoutGetWidth(node: YGNodeRef) -> c_float {
    return (*node).layout.dimensions[Dimension::Width as usize];
}

pub unsafe extern "C" fn YGNodeLayoutGetHeight(node: YGNodeRef) -> c_float {
    return (*node).layout.dimensions[Dimension::Height as usize];
}

pub unsafe extern "C" fn YGNodeLayoutGetDirection(node: YGNodeRef) -> Direction {
    return (*node).layout.direction;
}

pub unsafe extern "C" fn YGNodeLayoutGetHadOverflow(node: YGNodeRef) -> bool {
    return (*node).layout.hadOverflow;
}

pub unsafe extern "C" fn YGNodeLayoutGetMargin(node: YGNodeRef, edge: Edge) -> c_float {
    YGAssertWithNode(
        node,
        (edge as c_uint) < Edge::End as i32 as c_uint,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8 as *const c_char,
    );
    if edge as c_uint == Edge::Left as i32 as c_uint {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.margin[Edge::End as i32 as usize];
        } else {
            return (*node).layout.margin[Edge::Start as i32 as usize];
        };
    };
    if edge as c_uint == Edge::Right as i32 as c_uint {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.margin[Edge::Start as i32 as usize];
        } else {
            return (*node).layout.margin[Edge::End as i32 as usize];
        };
    };
    return (*node).layout.margin[edge as usize];
}

pub unsafe extern "C" fn YGNodeLayoutGetBorder(node: YGNodeRef, edge: Edge) -> c_float {
    YGAssertWithNode(
        node,
        (edge as c_uint) < Edge::End as i32 as c_uint,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8 as *const c_char,
    );
    if edge as c_uint == Edge::Left as i32 as c_uint {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.border[Edge::End as i32 as usize];
        } else {
            return (*node).layout.border[Edge::Start as i32 as usize];
        };
    };
    if edge as c_uint == Edge::Right as i32 as c_uint {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.border[Edge::Start as i32 as usize];
        } else {
            return (*node).layout.border[Edge::End as i32 as usize];
        };
    };
    return (*node).layout.border[edge as usize];
}

pub unsafe extern "C" fn YGNodeLayoutGetPadding(node: YGNodeRef, edge: Edge) -> c_float {
    YGAssertWithNode(
        node,
        (edge as c_uint) < Edge::End as i32 as c_uint,
        b"Cannot get layout properties of multi-edge shorthands\x00" as *const u8 as *const c_char,
    );
    if edge as c_uint == Edge::Left as i32 as c_uint {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.padding[Edge::End as i32 as usize];
        } else {
            return (*node).layout.padding[Edge::Start as i32 as usize];
        };
    };
    if edge as c_uint == Edge::Right as i32 as c_uint {
        if (*node).layout.direction == Direction::RTL {
            return (*node).layout.padding[Edge::Start as i32 as usize];
        } else {
            return (*node).layout.padding[Edge::End as i32 as usize];
        };
    };
    return (*node).layout.padding[edge as usize];
}

pub unsafe extern "C" fn YGConfigSetPointScaleFactor(
    config: YGConfigRef,
    pixelsInPoint: c_float,
) -> () {
    YGAssertWithConfig(
        config,
        pixelsInPoint >= 0.0f32,
        b"Scale factor should not be less than zero\x00" as *const u8 as *const c_char,
    );
    if pixelsInPoint == 0.0f32 {
        (*config).pointScaleFactor = 0.0f32;
    } else {
        (*config).pointScaleFactor = pixelsInPoint;
    };
}

pub unsafe extern "C" fn YGConfigSetUseLegacyStretchBehaviour(
    config: YGConfigRef,
    useLegacyStretchBehaviour: bool,
) -> () {
    (*config).useLegacyStretchBehaviour = useLegacyStretchBehaviour;
}

pub unsafe extern "C" fn YGConfigNew() -> YGConfigRef {
    let config: YGConfigRef = malloc(size_of::<YGConfig>()) as YGConfigRef;
    YGAssert(
        !config.is_null(),
        b"Could not allocate memory for config\x00" as *const u8 as *const c_char,
    );
    gConfigInstanceCount += 1;
    memcpy(
        config as *mut c_void,
        &mut gYGConfigDefaults as *mut YGConfig as *const c_void,
        size_of::<YGConfig>(),
    );
    return config;
}

static mut gConfigInstanceCount: int32_t = 0i32;

pub unsafe extern "C" fn YGConfigFree(config: YGConfigRef) -> () {
    gYGFree.expect("non-null function pointer")(config as *mut c_void);
    gConfigInstanceCount -= 1;
}

pub unsafe extern "C" fn YGConfigCopy(dest: YGConfigRef, src: YGConfigRef) -> () {
    memcpy(
        dest as *mut c_void,
        src as *const c_void,
        size_of::<YGConfig>(),
    );
}

pub unsafe extern "C" fn YGConfigGetInstanceCount() -> int32_t {
    return gConfigInstanceCount;
}

pub unsafe extern "C" fn YGConfigSetExperimentalFeatureEnabled(
    config: YGConfigRef,
    feature: YGExperimentalFeature_0,
    enabled: bool,
) -> () {
    (*config).experimentalFeatures[feature as usize] = enabled;
}

pub unsafe extern "C" fn YGConfigSetUseWebDefaults(config: YGConfigRef, enabled: bool) -> () {
    (*config).useWebDefaults = enabled;
}

pub unsafe extern "C" fn YGConfigGetUseWebDefaults(config: YGConfigRef) -> bool {
    return (*config).useWebDefaults;
}

pub unsafe extern "C" fn YGConfigSetNodeClonedFunc(
    config: YGConfigRef,
    callback: YGNodeClonedFunc,
) -> () {
    (*config).cloneNodeCallback = callback;
}

pub unsafe extern "C" fn YGConfigGetDefault() -> YGConfigRef {
    return &mut gYGConfigDefaults as *mut YGConfig;
}

pub unsafe extern "C" fn YGConfigSetContext(config: YGConfigRef, mut context: *mut c_void) -> () {
    (*config).context = context;
}

pub unsafe extern "C" fn YGConfigGetContext(config: YGConfigRef) -> *mut c_void {
    return (*config).context;
}

pub unsafe extern "C" fn YGNodeListAdd(mut listp: *mut YGNodeListRef, node: YGNodeRef) -> () {
    if (*listp).is_null() {
        *listp = YGNodeListNew(4);
    };
    YGNodeListInsert(listp, node, (**listp).count);
}

unsafe fn YGRoundToPixelGrid(
    node: YGNodeRef,
    pointScaleFactor: c_float,
    absoluteLeft: c_float,
    absoluteTop: c_float,
) {
    if pointScaleFactor == 0.0 {
        return;
    }

    let nodeLeft = (*node).layout.position[Edge::Left as usize];
    let nodeTop = (*node).layout.position[Edge::Top as usize];

    let nodeWidth = (*node).layout.dimensions[Dimension::Width as usize];
    let nodeHeight = (*node).layout.dimensions[Dimension::Height as usize];

    let absoluteNodeLeft = absoluteLeft + nodeLeft;
    let absoluteNodeTop = absoluteTop + nodeTop;

    let absoluteNodeRight = absoluteNodeLeft + nodeWidth;
    let absoluteNodeBottom = absoluteNodeTop + nodeHeight;

    // If a node has a custom measure function we never want to round down its size as this could
    // lead to unwanted text truncation.
    let textRounding = (*node).nodeType == NodeType::Text;

    (*node).layout.position[Edge::Left as usize] =
        YGRoundValueToPixelGrid(nodeLeft, pointScaleFactor, false, textRounding);
    (*node).layout.position[Edge::Top as usize] =
        YGRoundValueToPixelGrid(nodeTop, pointScaleFactor, false, textRounding);

    // We multiply dimension by scale factor and if the result is close to the whole number, we don't
    // have any fraction
    // To verify if the result is close to whole number we want to check both floor and ceil numbers
    let hasFractionalWidth = !YGFloatsEqual(nodeWidth * pointScaleFactor % 1.0, 0.0)
        && !YGFloatsEqual(nodeWidth * pointScaleFactor % 1.0, 1.0);
    let hasFractionalHeight = !YGFloatsEqual(nodeHeight * pointScaleFactor % 1.0, 0.0)
        && !YGFloatsEqual(nodeHeight * pointScaleFactor % 1.0, 1.0);

    (*node).layout.dimensions[Dimension::Width as usize] = YGRoundValueToPixelGrid(
        absoluteNodeRight,
        pointScaleFactor,
        textRounding && hasFractionalWidth,
        textRounding && !hasFractionalWidth,
    )
        - YGRoundValueToPixelGrid(absoluteNodeLeft, pointScaleFactor, false, textRounding);
    (*node).layout.dimensions[Dimension::Height as usize] = YGRoundValueToPixelGrid(
        absoluteNodeBottom,
        pointScaleFactor,
        textRounding && hasFractionalHeight,
        textRounding && !hasFractionalHeight,
    )
        - YGRoundValueToPixelGrid(absoluteNodeTop, pointScaleFactor, false, textRounding);

    for i in 0..YGNodeListCount((*node).children) {
        YGRoundToPixelGrid(
            YGNodeGetChild(node, i),
            pointScaleFactor,
            absoluteNodeLeft,
            absoluteNodeTop,
        );
    }
}

pub unsafe extern "C" fn YGConstrainMaxSizeForMode(
    node: YGNodeRef,
    axis: FlexDirection,
    parentAxisSize: c_float,
    parentWidth: c_float,
    mode: *mut MeasureMode,
    size: *mut c_float,
) {
    let maxSize = YGResolveValue(
        &(*node).style.maxDimensions[DIM[axis as usize]],
        parentAxisSize,
    ) + YGNodeMarginForAxis(node, axis, parentWidth);
    match *mode {
        MeasureMode::Exactly | MeasureMode::AtMost => {
            *size = if maxSize.is_nan() || *size < maxSize {
                *size
            } else {
                maxSize
            }
        }
        MeasureMode::Undefined => if !maxSize.is_nan() {
            *mode = MeasureMode::AtMost;
            *size = maxSize;
        },
    }
}

static mut firstAbsoluteChild: YGNodeRef = ::std::ptr::null_mut();
static mut currentAbsoluteChild: YGNodeRef = ::std::ptr::null_mut();

//
// This is the main routine that implements a subset of the flexbox layout
// algorithm
// described in the W3C YG documentation: https://www.w3.org/TR/YG3-flexbox/.
//
// Limitations of this algorithm, compared to the full standard:
//  * Display property is always assumed to be 'flex' except for Text nodes,
//  which
//    are assumed to be 'inline-flex'.
//  * The 'zIndex' property (or any form of z ordering) is not supported. Nodes
//  are
//    stacked in document order.
//  * The 'order' property is not supported. The order of flex items is always
//  defined
//    by document order.
//  * The 'visibility' property is always assumed to be 'visible'. Values of
//  'collapse'
//    and 'hidden' are not supported.
//  * There is no support for forced breaks.
//  * It does not support vertical inline directions (top-to-bottom or
//  bottom-to-top text).
//
// Deviations from standard:
//  * Section 4.5 of the spec indicates that all flex items have a default
//  minimum
//    main size. For text blocks, for example, this is the width of the widest
//    word.
//    Calculating the minimum width is expensive, so we forego it and assume a
//    default
//    minimum main size of 0.
//  * Min/Max sizes in the main axis are not honored when resolving flexible
//  lengths.
//  * The spec indicates that the default value for 'flexDirection' is 'row',
//  but
//    the algorithm below assumes a default of 'column'.
//
// Input parameters:
//    - node: current node to be sized and layed out
//    - availableWidth & availableHeight: available size to be used for sizing
//    the node
//      or YGUndefined if the size is not available; interpretation depends on
//      layout
//      flags
//    - parentDirection: the inline (text) direction within the parent
//    (left-to-right or
//      right-to-left)
//    - widthMeasureMode: indicates the sizing rules for the width (see below
//    for explanation)
//    - heightMeasureMode: indicates the sizing rules for the height (see below
//    for explanation)
//    - performLayout: specifies whether the caller is interested in just the
//    dimensions
//      of the node or it requires the entire node and its subtree to be layed
//      out
//      (with final positions)
//
// Details:
//    This routine is called recursively to lay out subtrees of flexbox
//    elements. It uses the
//    information in node.style, which is treated as a read-only input. It is
//    responsible for
//    setting the layout.direction and layout.measuredDimensions fields for the
//    input node as well
//    as the layout.position and layout.lineIndex fields for its child nodes.
//    The
//    layout.measuredDimensions field includes any border or padding for the
//    node but does
//    not include margins.
//
//    The spec describes four different layout modes: "fill available", "max
//    content", "min
//    content",
//    and "fit content". Of these, we don't use "min content" because we don't
//    support default
//    minimum main sizes (see above for details). Each of our measure modes maps
//    to a layout mode
//    from the spec (https://www.w3.org/TR/YG3-sizing/#terms):
//      - MeasureMode::Undefined: max content
//      - MeasureMode::Exactly: fill available
//      - MeasureMode::AtMost: fit content
//
//    When calling YGNodelayoutImpl and YGLayoutNodeInternal, if the caller passes
//    an available size of
//    undefined then it must also pass a measure mode of MeasureMode::Undefined
//    in that dimension.
//
unsafe fn YGNodelayoutImpl(
    node: YGNodeRef,
    availableWidth: c_float,
    availableHeight: c_float,
    parentDirection: Direction,
    widthMeasureMode: MeasureMode,
    heightMeasureMode: MeasureMode,
    parentWidth: c_float,
    parentHeight: c_float,
    performLayout: bool,
    config: YGConfigRef,
) {
    assert!(
        if availableWidth.is_nan() {
            widthMeasureMode == MeasureMode::Undefined
        } else {
            true
        },
        "availableWidth is indefinite so widthMeasureMode must be MeasureMode::Undefined"
    );

    assert!(
        if availableHeight.is_nan() {
            heightMeasureMode == MeasureMode::Undefined
        } else {
            true
        },
        "availableHeight is indefinite so heightMeasureMode must be MeasureMode::Undefined"
    );

    // // Set the resolved resolution in the node's layout.
    let direction = YGNodeResolveDirection(node, parentDirection);
    (*node).layout.direction = direction;

    let flexRowDirection = YGResolveFlexDirection(FlexDirection::Row, direction);
    let flexColumnDirection = YGResolveFlexDirection(FlexDirection::Column, direction);

    (*node).layout.margin[Edge::Start as usize] =
        YGNodeLeadingMargin(node, flexRowDirection, parentWidth);
    (*node).layout.margin[Edge::End as usize] =
        YGNodeTrailingMargin(node, flexRowDirection, parentWidth);
    (*node).layout.margin[Edge::Top as usize] =
        YGNodeLeadingMargin(node, flexColumnDirection, parentWidth);
    (*node).layout.margin[Edge::Bottom as usize] =
        YGNodeTrailingMargin(node, flexColumnDirection, parentWidth);

    (*node).layout.border[Edge::Start as usize] = YGNodeLeadingBorder(node, flexRowDirection);
    (*node).layout.border[Edge::End as usize] = YGNodeTrailingBorder(node, flexRowDirection);
    (*node).layout.border[Edge::Top as usize] = YGNodeLeadingBorder(node, flexColumnDirection);
    (*node).layout.border[Edge::Bottom as usize] = YGNodeTrailingBorder(node, flexColumnDirection);

    (*node).layout.padding[Edge::Start as usize] =
        YGNodeLeadingPadding(node, flexRowDirection, parentWidth);
    (*node).layout.padding[Edge::End as usize] =
        YGNodeTrailingPadding(node, flexRowDirection, parentWidth);
    (*node).layout.padding[Edge::Top as usize] =
        YGNodeLeadingPadding(node, flexColumnDirection, parentWidth);
    (*node).layout.padding[Edge::Bottom as usize] =
        YGNodeTrailingPadding(node, flexColumnDirection, parentWidth);

    if (*node).measure.is_some() {
        YGNodeWithMeasureFuncSetMeasuredDimensions(
            node,
            availableWidth,
            availableHeight,
            widthMeasureMode,
            heightMeasureMode,
            parentWidth,
            parentHeight,
        );
        return;
    }

    let childCount = YGNodeListCount((*node).children);
    if childCount == 0 {
        YGNodeEmptyContainerSetMeasuredDimensions(
            node,
            availableWidth,
            availableHeight,
            widthMeasureMode,
            heightMeasureMode,
            parentWidth,
            parentHeight,
        );
        return;
    }

    // If we're not being asked to perform a full layout we can skip the algorithm if we already know
    // the size
    if !performLayout
        && YGNodeFixedSizeSetMeasuredDimensions(
            node,
            availableWidth,
            availableHeight,
            widthMeasureMode,
            heightMeasureMode,
            parentWidth,
            parentHeight,
        ) {
        return;
    }

    // At this point we know we're going to perform work. Ensure that each child has a mutable copy.
    YGCloneChildrenIfNeeded(node);

    // Reset layout flags, as they could have changed.
    (*node).layout.hadOverflow = false;

    // STEP 1: CALCULATE VALUES FOR REMAINDER OF ALGORITHM
    let mainAxis = YGResolveFlexDirection((*node).style.flexDirection, direction);
    let crossAxis = FlexDirectionCross(mainAxis, direction);
    let isMainAxisRow = FlexDirectionIsRow(mainAxis);
    let justifyContent = (*node).style.justifyContent;
    let isNodeFlexWrap = (*node).style.flexWrap != YGWrapNoWrap;

    let mainAxisParentSize = if isMainAxisRow {
        parentWidth
    } else {
        parentHeight
    };
    let crossAxisParentSize = if isMainAxisRow {
        parentHeight
    } else {
        parentWidth
    };

    let leadingPaddingAndBorderMain = YGNodeLeadingPaddingAndBorder(node, mainAxis, parentWidth);
    let trailingPaddingAndBorderMain = YGNodeTrailingPaddingAndBorder(node, mainAxis, parentWidth);
    let leadingPaddingAndBorderCross = YGNodeLeadingPaddingAndBorder(node, crossAxis, parentWidth);
    let paddingAndBorderAxisMain = YGNodePaddingAndBorderForAxis(node, mainAxis, parentWidth);
    let paddingAndBorderAxisCross = YGNodePaddingAndBorderForAxis(node, crossAxis, parentWidth);

    let mut measureModeMainDim = if isMainAxisRow {
        widthMeasureMode
    } else {
        heightMeasureMode
    };
    let measureModeCrossDim = if isMainAxisRow {
        heightMeasureMode
    } else {
        widthMeasureMode
    };

    let paddingAndBorderAxisRow = if isMainAxisRow {
        paddingAndBorderAxisMain
    } else {
        paddingAndBorderAxisCross
    };
    let paddingAndBorderAxisColumn = if isMainAxisRow {
        paddingAndBorderAxisCross
    } else {
        paddingAndBorderAxisMain
    };

    let marginAxisRow = YGNodeMarginForAxis(node, FlexDirection::Row, parentWidth);
    let marginAxisColumn = YGNodeMarginForAxis(node, FlexDirection::Column, parentWidth);

    // STEP 2: DETERMINE AVAILABLE SIZE IN MAIN AND CROSS DIRECTIONS
    let minInnerWidth = YGResolveValue(&(*node).style.minDimensions.width, parentWidth)
        - marginAxisRow
        - paddingAndBorderAxisRow;
    let maxInnerWidth = YGResolveValue(&(*node).style.maxDimensions.width, parentWidth)
        - marginAxisRow
        - paddingAndBorderAxisRow;
    let minInnerHeight = YGResolveValue(&(*node).style.minDimensions.height, parentHeight)
        - marginAxisColumn
        - paddingAndBorderAxisColumn;
    let maxInnerHeight = YGResolveValue(&(*node).style.maxDimensions.height, parentHeight)
        - marginAxisColumn
        - paddingAndBorderAxisColumn;
    let minInnerMainDim = if isMainAxisRow {
        minInnerWidth
    } else {
        minInnerHeight
    };
    let maxInnerMainDim = if isMainAxisRow {
        maxInnerWidth
    } else {
        maxInnerHeight
    };

    // Max dimension overrides predefined dimension value; Min dimension in turn overrides both of the
    // above
    let mut availableInnerWidth = availableWidth - marginAxisRow - paddingAndBorderAxisRow;
    if !availableInnerWidth.is_nan() {
        // We want to make sure our available width does not violate min and max constraints
        availableInnerWidth = availableInnerWidth.min(maxInnerWidth).max(minInnerWidth);
    }

    let mut availableInnerHeight = availableHeight - marginAxisColumn - paddingAndBorderAxisColumn;
    if !availableInnerHeight.is_nan() {
        // We want to make sure our available height does not violate min and max constraints
        availableInnerHeight = availableInnerHeight.min(maxInnerHeight).max(minInnerHeight);
    }

    let mut availableInnerMainDim = if isMainAxisRow {
        availableInnerWidth
    } else {
        availableInnerHeight
    };
    let mut availableInnerCrossDim = if isMainAxisRow {
        availableInnerHeight
    } else {
        availableInnerWidth
    };

    // If there is only one child with flexGrow + flexShrink it means we can set the
    // computedFlexBasis to 0 instead of measuring and shrinking / flexing the child to exactly
    // match the remaining space
    let mut singleFlexChild: YGNodeRef = ::std::ptr::null_mut();
    if measureModeMainDim == MeasureMode::Exactly {
        for i in 0..childCount {
            let child = YGNodeGetChild(node, i);
            if !singleFlexChild.is_null() {
                if YGNodeIsFlex(child) {
                    // There is already a flexible child, abort.
                    singleFlexChild = ::std::ptr::null_mut();
                    break;
                }
            } else if YGResolveFlexGrow(child) > 0.0 && YGNodeResolveFlexShrink(child) > 0.0 {
                singleFlexChild = child;
            }
        }
    }

    let mut totalOuterFlexBasis = 0;

    // // STEP 3: DETERMINE FLEX BASIS FOR EACH ITEM
    for i in 0..childCount {
        {
            let child = YGNodeListGet((*node).children, i);
            if (*child).style.display == Display::None {
                YGZeroOutLayoutRecursivly(child);
                (*child).hasNewLayout = true;
                (*child).isDirty = false;
                continue;
            }
            YGResolveDimensions(child);
            if performLayout {
                // Set the initial position (relative to the parent).
                let childDirection = YGNodeResolveDirection(child, direction);
                YGNodeSetPosition(
                    child,
                    childDirection,
                    availableInnerMainDim,
                    availableInnerCrossDim,
                    availableInnerWidth,
                );
            }

            // Absolute-positioned children don't participate in flex layout. Add them
            // to a list that we can process later.
            if (*child).style.positionType == YGPositionTypeAbsolute {
                // Store a private linked list of absolutely positioned children
                // so that we can efficiently traverse them later.
                if firstAbsoluteChild.is_null() {
                    firstAbsoluteChild = child;
                }
                if currentAbsoluteChild.is_null() {
                    (*currentAbsoluteChild).nextChild = child;
                }
                currentAbsoluteChild = child;
                (*child).nextChild = ::std::ptr::null_mut();
            } else {
                if child == singleFlexChild {
                    (*child).layout.computedFlexBasisGeneration = gCurrentGenerationCount;
                    (*child).layout.computedFlexBasis = 0.0;
                } else {
                    YGNodeComputeFlexBasisForChild(
                        node,
                        child,
                        availableInnerWidth,
                        widthMeasureMode,
                        availableInnerHeight,
                        availableInnerWidth,
                        availableInnerHeight,
                        heightMeasureMode,
                        direction,
                        config,
                    );
                }
            }

            totalOuterFlexBasis += ((*child).layout.computedFlexBasis
                + YGNodeMarginForAxis(child, mainAxis, availableInnerWidth))
                as i32;
        }

        let flexBasisOverflows = if measureModeMainDim == MeasureMode::Undefined {
            false
        } else {
            totalOuterFlexBasis > availableInnerMainDim as i32
        };
        if isNodeFlexWrap && flexBasisOverflows && measureModeMainDim == MeasureMode::AtMost {
            measureModeMainDim = MeasureMode::Exactly;
        }

        // STEP 4: COLLECT FLEX ITEMS INTO FLEX LINES

        // Indexes of children that represent the first and last items in the line.
        let mut startOfLineIndex = 0;
        let mut endOfLineIndex = 0;

        // Number of lines.
        let mut lineCount = 0;

        // Accumulated cross dimensions of all lines so far.
        let mut totalLineCrossDim = 0.0f32;

        // Max main dimension of all the lines.
        let mut maxLineMainDim = 0.0f32;

        while endOfLineIndex < childCount {
            // Number of items on the currently line. May be different than the
            // difference
            // between start and end indicates because we skip over absolute-positioned
            // items.
            let mut itemsOnLine = 0;

            // sizeConsumedOnCurrentLine is accumulation of the dimensions and margin
            // of all the children on the current line. This will be used in order to
            // either set the dimensions of the node if none already exist or to compute
            // the remaining space left for the flexible children.
            let mut sizeConsumedOnCurrentLine = 0.0f32;
            let mut sizeConsumedOnCurrentLineIncludingMinConstraint = 0.0f32;

            let mut totalFlexGrowFactors = 0.0f32;
            let mut totalFlexShrinkScaledFactors = 0.0f32;

            // Maintain a linked list of the child nodes that can shrink and/or grow.
            let mut firstRelativeChild: YGNodeRef = ::std::ptr::null_mut();
            let mut currentRelativeChild: YGNodeRef = ::std::ptr::null_mut();

            // Add items to the current line until it's full or we run out of items.
            let mut i = startOfLineIndex;
            while i < childCount {
                let child = YGNodeListGet((*node).children, i);
                if (*child).style.display == Display::None {
                    continue;
                }
                (*child).lineIndex = lineCount;

                if (*child).style.positionType != YGPositionTypeAbsolute {
                    let childMarginMainAxis =
                        YGNodeMarginForAxis(child, mainAxis, availableInnerWidth);
                    let flexBasisWithMaxConstraints = YGResolveValue(
                        &(*child).style.maxDimensions[DIM[mainAxis as usize]],
                        mainAxisParentSize,
                    ).min((*child).layout.computedFlexBasis);
                    let flexBasisWithMinAndMaxConstraints = YGResolveValue(
                        &(*child).style.minDimensions[DIM[mainAxis as usize]],
                        mainAxisParentSize,
                    ).max(flexBasisWithMaxConstraints);

                    // If this is a multi-line flow and this item pushes us over the
                    // available size, we've
                    // hit the end of the current line. Break out of the loop and lay out
                    // the current line.
                    if sizeConsumedOnCurrentLineIncludingMinConstraint
                        + flexBasisWithMinAndMaxConstraints
                        + childMarginMainAxis > availableInnerMainDim
                        && isNodeFlexWrap && itemsOnLine > 0
                    {
                        break;
                    }

                    sizeConsumedOnCurrentLineIncludingMinConstraint +=
                        flexBasisWithMinAndMaxConstraints + childMarginMainAxis;
                    sizeConsumedOnCurrentLine +=
                        flexBasisWithMinAndMaxConstraints + childMarginMainAxis;
                    itemsOnLine += 1;

                    if YGNodeIsFlex(child) {
                        totalFlexGrowFactors += YGResolveFlexGrow(child);

                        // Unlike the grow factor, the shrink factor is scaled relative to the child dimension.
                        totalFlexShrinkScaledFactors +=
                            -YGNodeResolveFlexShrink(child) * (*child).layout.computedFlexBasis;
                    }

                    // Store a private linked list of children that need to be layed out.
                    if firstRelativeChild.is_null() {
                        firstRelativeChild = child;
                    }
                    if currentRelativeChild.is_null() {
                        (*currentRelativeChild).nextChild = child;
                    }
                    currentRelativeChild = child;
                    (*child).nextChild = ::std::ptr::null_mut();
                }
                i += 1;
                endOfLineIndex += 1;
            }

            // The total flex factor needs to be floored to 1.
            if totalFlexGrowFactors > 0.0 && totalFlexGrowFactors < 1.0 {
                totalFlexGrowFactors = 1.0;
            }

            // The total flex shrink factor needs to be floored to 1.
            if totalFlexShrinkScaledFactors > 0.0 && totalFlexShrinkScaledFactors < 1.0 {
                totalFlexShrinkScaledFactors = 1.0;
            }

            // If we don't need to measure the cross axis, we can skip the entire flex
            // step.
            let canSkipFlex = !performLayout && measureModeCrossDim == MeasureMode::Exactly;

            // In order to position the elements in the main axis, we have two
            // controls. The space between the beginning and the first element
            // and the space between each two elements.
            let mut leadingMainDim = 0.0;
            let mut betweenMainDim = 0.0;

            // STEP 5: RESOLVING FLEXIBLE LENGTHS ON MAIN AXIS
            // Calculate the remaining available space that needs to be allocated.
            // If the main dimension size isn't known, it is computed based on
            // the line length, so there's no more space left to distribute.

            // If we don't measure with exact main dimension we want to ensure we don't violate min and max
            if measureModeMainDim != MeasureMode::Exactly {
                if !minInnerMainDim.is_nan() && sizeConsumedOnCurrentLine < minInnerMainDim {
                    availableInnerMainDim = minInnerMainDim;
                } else if !maxInnerMainDim.is_nan() && sizeConsumedOnCurrentLine > maxInnerMainDim {
                    availableInnerMainDim = maxInnerMainDim;
                } else if !(*(*node).config).useLegacyStretchBehaviour
                    && (totalFlexGrowFactors == 0.0 || YGResolveFlexGrow(node) == 0.0)
                {
                    // If we don't have any children to flex or we can't flex the node itself,
                    // space we've used is all space we need. Root node also should be shrunk to minimum
                    availableInnerMainDim = sizeConsumedOnCurrentLine;
                }
            }

            let mut remainingFreeSpace = 0.0;
            if !availableInnerMainDim.is_nan() {
                remainingFreeSpace = availableInnerMainDim - sizeConsumedOnCurrentLine;
            } else if sizeConsumedOnCurrentLine < 0.0 {
                // availableInnerMainDim is indefinite which means the node is being sized based on its
                // content.
                // sizeConsumedOnCurrentLine is negative which means the node will allocate 0 points for
                // its content. Consequently, remainingFreeSpace is 0 - sizeConsumedOnCurrentLine.
                remainingFreeSpace = -sizeConsumedOnCurrentLine;
            }

            let mut originalRemainingFreeSpace = remainingFreeSpace;
            let mut deltaFreeSpace = 0.0;

            if !canSkipFlex {
                let mut childFlexBasis: f32;
                let mut flexShrinkScaledFactor: f32;
                let mut flexGrowFactor: f32;
                let mut baseMainSize: f32;
                let mut boundMainSize: f32;

                // Do two passes over the flex items to figure out how to distribute the
                // remaining space.
                // The first pass finds the items whose min/max constraints trigger,
                // freezes them at those
                // sizes, and excludes those sizes from the remaining space. The second
                // pass sets the size
                // of each flexible item. It distributes the remaining space amongst the
                // items whose min/max
                // constraints didn't trigger in pass 1. For the other items, it sets
                // their sizes by forcing
                // their min/max constraints to trigger again.
                //
                // This two pass approach for resolving min/max constraints deviates from
                // the spec. The
                // spec (https://www.w3.org/TR/YG-flexbox-1/#resolve-flexible-lengths)
                // describes a process
                // that needs to be repeated a variable number of times. The algorithm
                // implemented here
                // won't handle all cases but it was simpler to implement and it mitigates
                // performance
                // concerns because we know exactly how many passes it'll do.

                // First pass: detect the flex items whose min/max constraints trigger
                let mut deltaFlexShrinkScaledFactors = 0.0;
                let mut deltaFlexGrowFactors = 0.0;
                currentRelativeChild = firstRelativeChild;
                while !currentRelativeChild.is_null() {
                    childFlexBasis = YGResolveValue(
                        &(*currentRelativeChild).style.maxDimensions[DIM[mainAxis as usize]],
                        mainAxisParentSize,
                    ).min(
                        YGResolveValue(
                            &(*currentRelativeChild).style.minDimensions[DIM[mainAxis as usize]],
                            mainAxisParentSize,
                        ).max((*currentRelativeChild).layout.computedFlexBasis),
                    );

                    if remainingFreeSpace < 0.0 {
                        flexShrinkScaledFactor =
                            -YGNodeResolveFlexShrink(currentRelativeChild) * childFlexBasis;

                        // Is this child able to shrink?
                        if flexShrinkScaledFactor != 0.0 {
                            baseMainSize = childFlexBasis
                                + remainingFreeSpace / totalFlexShrinkScaledFactors
                                    * flexShrinkScaledFactor;
                            boundMainSize = YGNodeBoundAxis(
                                currentRelativeChild,
                                mainAxis,
                                baseMainSize,
                                availableInnerMainDim,
                                availableInnerWidth,
                            );
                            if baseMainSize != boundMainSize {
                                // By excluding this item's size and flex factor from remaining,
                                // this item's
                                // min/max constraints should also trigger in the second pass
                                // resulting in the
                                // item's size calculation being identical in the first and second
                                // passes.
                                deltaFreeSpace -= boundMainSize - childFlexBasis;
                                deltaFlexShrinkScaledFactors -= flexShrinkScaledFactor;
                            }
                        }
                    } else if remainingFreeSpace > 0.0 {
                        flexGrowFactor = YGResolveFlexGrow(currentRelativeChild);

                        // Is this child able to grow?
                        if flexGrowFactor != 0.0 {
                            baseMainSize = childFlexBasis
                                + remainingFreeSpace / totalFlexGrowFactors * flexGrowFactor;
                            boundMainSize = YGNodeBoundAxis(
                                currentRelativeChild,
                                mainAxis,
                                baseMainSize,
                                availableInnerMainDim,
                                availableInnerWidth,
                            );

                            if baseMainSize != boundMainSize {
                                // By excluding this item's size and flex factor from remaining,
                                // this item's
                                // min/max constraints should also trigger in the second pass
                                // resulting in the
                                // item's size calculation being identical in the first and second
                                // passes.
                                deltaFreeSpace -= boundMainSize - childFlexBasis;
                                deltaFlexGrowFactors -= flexGrowFactor;
                            }
                        }
                    }

                    currentRelativeChild = (*currentRelativeChild).nextChild;
                }

                totalFlexShrinkScaledFactors += deltaFlexShrinkScaledFactors;
                totalFlexGrowFactors += deltaFlexGrowFactors;
                remainingFreeSpace += deltaFreeSpace;

                // Second pass: resolve the sizes of the flexible items
                deltaFreeSpace = 0.0;
                currentRelativeChild = firstRelativeChild;
                while !currentRelativeChild.is_null() {
                    childFlexBasis = YGResolveValue(
                        &(*currentRelativeChild).style.maxDimensions[DIM[mainAxis as usize]],
                        mainAxisParentSize,
                    ).min(
                        YGResolveValue(
                            &(*currentRelativeChild).style.minDimensions[DIM[mainAxis as usize]],
                            mainAxisParentSize,
                        ).max((*currentRelativeChild).layout.computedFlexBasis),
                    );
                    let mut updatedMainSize = childFlexBasis;

                    if remainingFreeSpace < 0.0 {
                        flexShrinkScaledFactor =
                            -YGNodeResolveFlexShrink(currentRelativeChild) * childFlexBasis;
                        // Is this child able to shrink?
                        if flexShrinkScaledFactor != 0.0 {
                            let mut childSize: f32;

                            if totalFlexShrinkScaledFactors == 0.0 {
                                childSize = childFlexBasis + flexShrinkScaledFactor;
                            } else {
                                childSize = childFlexBasis
                                    + (remainingFreeSpace / totalFlexShrinkScaledFactors)
                                        * flexShrinkScaledFactor;
                            }

                            updatedMainSize = YGNodeBoundAxis(
                                currentRelativeChild,
                                mainAxis,
                                childSize,
                                availableInnerMainDim,
                                availableInnerWidth,
                            );
                        }
                    } else if remainingFreeSpace > 0.0 {
                        flexGrowFactor = YGResolveFlexGrow(currentRelativeChild);

                        // Is this child able to grow?
                        if flexGrowFactor != 0.0 {
                            updatedMainSize = YGNodeBoundAxis(
                                currentRelativeChild,
                                mainAxis,
                                childFlexBasis
                                    + remainingFreeSpace / totalFlexGrowFactors * flexGrowFactor,
                                availableInnerMainDim,
                                availableInnerWidth,
                            );
                        }
                    }

                    deltaFreeSpace -= updatedMainSize - childFlexBasis;

                    let marginMain =
                        YGNodeMarginForAxis(currentRelativeChild, mainAxis, availableInnerWidth);
                    let marginCross =
                        YGNodeMarginForAxis(currentRelativeChild, crossAxis, availableInnerWidth);

                    let mut childCrossSize: f32;
                    let mut childMainSize = updatedMainSize + marginMain;
                    let mut childCrossMeasureMode: MeasureMode;
                    let mut childMainMeasureMode: MeasureMode = MeasureMode::Exactly;

                    // TODO(anp) check for bug on the C side -- this was an != NULL check
                    if !(*currentRelativeChild).style.aspectRatio.is_nan() {
                        childCrossSize = if isMainAxisRow {
                            (childMainSize - marginMain) / (*currentRelativeChild).style.aspectRatio
                        } else {
                            (childMainSize - marginMain) * (*currentRelativeChild).style.aspectRatio
                        };
                        childCrossMeasureMode = MeasureMode::Exactly;

                        childCrossSize += marginCross;
                    } else if !availableInnerCrossDim.is_nan()
                        && !YGNodeIsStyleDimDefined(
                            currentRelativeChild,
                            crossAxis,
                            availableInnerCrossDim,
                        )
                        && measureModeCrossDim == MeasureMode::Exactly
                        && !(isNodeFlexWrap && flexBasisOverflows)
                        && YGNodeAlignItem(node, currentRelativeChild) == Align::Stretch
                    {
                        childCrossSize = availableInnerCrossDim;
                        childCrossMeasureMode = MeasureMode::Exactly;
                    } else if !YGNodeIsStyleDimDefined(
                        currentRelativeChild,
                        crossAxis,
                        availableInnerCrossDim,
                    ) {
                        childCrossSize = availableInnerCrossDim;
                        childCrossMeasureMode = if childCrossSize.is_nan() {
                            MeasureMode::Undefined
                        } else {
                            MeasureMode::AtMost
                        };
                    } else {
                        childCrossSize = YGResolveValue(
                            (*currentRelativeChild).resolvedDimensions[DIM[crossAxis as usize]],
                            availableInnerCrossDim,
                        ) + marginCross;
                        let isLoosePercentageMeasurement =
                            (*(*currentRelativeChild).resolvedDimensions[DIM[crossAxis as usize]])
                                .unit == YGUnitPercent
                                && measureModeCrossDim != MeasureMode::Exactly;
                        childCrossMeasureMode =
                            if childCrossSize.is_nan() || isLoosePercentageMeasurement {
                                MeasureMode::Undefined
                            } else {
                                MeasureMode::Exactly
                            };
                    }

                    YGConstrainMaxSizeForMode(
                        currentRelativeChild,
                        mainAxis,
                        availableInnerMainDim,
                        availableInnerWidth,
                        &mut childMainMeasureMode,
                        &mut childMainSize,
                    );
                    YGConstrainMaxSizeForMode(
                        currentRelativeChild,
                        crossAxis,
                        availableInnerCrossDim,
                        availableInnerWidth,
                        &mut childCrossMeasureMode,
                        &mut childCrossSize,
                    );

                    let requiresStretchLayout = !YGNodeIsStyleDimDefined(
                        currentRelativeChild,
                        crossAxis,
                        availableInnerCrossDim,
                    )
                        && YGNodeAlignItem(node, currentRelativeChild) == Align::Stretch;

                    let childWidth = if isMainAxisRow {
                        childMainSize
                    } else {
                        childCrossSize
                    };
                    let childHeight = if !isMainAxisRow {
                        childMainSize
                    } else {
                        childCrossSize
                    };

                    let childWidthMeasureMode = if isMainAxisRow {
                        childMainMeasureMode
                    } else {
                        childCrossMeasureMode
                    };
                    let childHeightMeasureMode = if !isMainAxisRow {
                        childMainMeasureMode
                    } else {
                        childCrossMeasureMode
                    };

                    // Recursively call the layout algorithm for this child with the updated
                    // main size.
                    YGLayoutNodeInternal(
                        currentRelativeChild,
                        childWidth,
                        childHeight,
                        direction,
                        childWidthMeasureMode,
                        childHeightMeasureMode,
                        availableInnerWidth,
                        availableInnerHeight,
                        performLayout && !requiresStretchLayout,
                        "flex",
                        config,
                    );
                    (*node).layout.hadOverflow |= (*currentRelativeChild).layout.hadOverflow;

                    currentRelativeChild = (*currentRelativeChild).nextChild;
                }
            }

            remainingFreeSpace = originalRemainingFreeSpace + deltaFreeSpace;
            (*node).layout.hadOverflow |= remainingFreeSpace < 0.0;

            // STEP 6: MAIN-AXIS JUSTIFICATION & CROSS-AXIS SIZE DETERMINATION

            // At this point, all the children have their dimensions set in the main
            // axis.
            // Their dimensions are also set in the cross axis with the exception of
            // items
            // that are aligned "stretch". We need to compute these stretch values and
            // set the final positions.

            // If we are using "at most" rules in the main axis. Calculate the remaining space when
            // constraint by the min size defined for the main axis.

            if measureModeMainDim == MeasureMode::AtMost && remainingFreeSpace > 0.0 {
                if (*node).style.minDimensions[DIM[mainAxis as usize]].unit != YGUnitUndefined
                    && YGResolveValue(
                        &(*node).style.minDimensions[DIM[mainAxis as usize]],
                        mainAxisParentSize,
                    ) >= 0.0
                {
                    remainingFreeSpace = (0.0f32).max(
                        YGResolveValue(
                            &(*node).style.minDimensions[DIM[mainAxis as usize]],
                            mainAxisParentSize,
                        ) - (availableInnerMainDim - remainingFreeSpace),
                    );
                } else {
                    remainingFreeSpace = 0.0;
                }
            }

            let mut numberOfAutoMarginsOnCurrentLine = 0;
            for i in startOfLineIndex..endOfLineIndex {
                let child = YGNodeListGet((*node).children, i);
                if (*child).style.positionType == YGPositionTypeRelative {
                    if (*YGMarginLeadingValue(child, mainAxis)).unit == YGUnitAuto {
                        numberOfAutoMarginsOnCurrentLine += 1;
                    }
                    if (*YGMarginTrailingValue(child, mainAxis)).unit == YGUnitAuto {
                        numberOfAutoMarginsOnCurrentLine += 1;
                    }
                }
            }

            if numberOfAutoMarginsOnCurrentLine == 0 {
                match justifyContent {
                    Justify::Center => leadingMainDim = remainingFreeSpace / 2.0,
                    Justify::FlexEnd => leadingMainDim = remainingFreeSpace,
                    Justify::SpaceBetween => {
                        if itemsOnLine > 1 {
                            betweenMainDim = remainingFreeSpace.max(0.0) / (itemsOnLine - 1) as f32;
                        } else {
                            betweenMainDim = 0.0;
                        }
                    }
                    Justify::SpaceAround => {
                        // Space on the edges is half of the space between elements
                        betweenMainDim = remainingFreeSpace / itemsOnLine as f32;
                        leadingMainDim = betweenMainDim / 2.0;
                    }
                    _ => (),
                }
            }

            let mut mainDim = leadingPaddingAndBorderMain + leadingMainDim;
            let mut crossDim = 0.0;

            for i in startOfLineIndex..endOfLineIndex {
                let child = YGNodeListGet((*node).children, i);
                if (*child).style.display == Display::None {
                    continue;
                }
                if (*child).style.positionType == YGPositionTypeAbsolute
                    && YGNodeIsLeadingPosDefined(child, mainAxis)
                {
                    if performLayout {
                        // In case the child is position absolute and has left/top being
                        // defined, we override the position to whatever the user said
                        // (and margin/border).
                        (*child).layout.position[pos[mainAxis as usize] as usize] =
                            YGNodeLeadingPosition(child, mainAxis, availableInnerMainDim)
                                + YGNodeLeadingBorder(node, mainAxis)
                                + YGNodeLeadingMargin(child, mainAxis, availableInnerWidth);
                    }
                } else {
                    // Now that we placed the element, we need to update the variables.
                    // We need to do that only for relative elements. Absolute elements
                    // do not take part in that phase.
                    if (*child).style.positionType == YGPositionTypeRelative {
                        if (*YGMarginLeadingValue(child, mainAxis)).unit == YGUnitAuto {
                            mainDim += remainingFreeSpace / numberOfAutoMarginsOnCurrentLine as f32;
                        }

                        if performLayout {
                            (*child).layout.position[pos[mainAxis as usize] as usize] += mainDim;
                        }

                        if (*YGMarginTrailingValue(child, mainAxis)).unit == YGUnitAuto {
                            mainDim += remainingFreeSpace / numberOfAutoMarginsOnCurrentLine as f32;
                        }

                        if canSkipFlex {
                            // If we skipped the flex step, then we can't rely on the
                            // measuredDims because
                            // they weren't computed. This means we can't call YGNodeDimWithMargin.
                            mainDim += betweenMainDim
                                + YGNodeMarginForAxis(child, mainAxis, availableInnerWidth)
                                + (*child).layout.computedFlexBasis;
                            crossDim = availableInnerCrossDim;
                        } else {
                            // The main dimension is the sum of all the elements dimension plus the spacing.
                            mainDim += betweenMainDim
                                + YGNodeDimWithMargin(child, mainAxis, availableInnerWidth);

                            // The cross dimension is the max of the elements dimension since
                            // there can only be one element in that cross dimension.
                            crossDim = crossDim.max(YGNodeDimWithMargin(
                                child,
                                crossAxis,
                                availableInnerWidth,
                            ));
                        }
                    } else if performLayout {
                        (*child).layout.position[pos[mainAxis as usize] as usize] +=
                            YGNodeLeadingBorder(node, mainAxis) + leadingMainDim;
                    }
                }
            }

            mainDim += trailingPaddingAndBorderMain;

            let mut containerCrossAxis = availableInnerCrossDim;
            if measureModeCrossDim == MeasureMode::Undefined
                || measureModeCrossDim == MeasureMode::AtMost
            {
                // Compute the cross axis from the max cross dimension of the children.
                containerCrossAxis = YGNodeBoundAxis(
                    node,
                    crossAxis,
                    crossDim + paddingAndBorderAxisCross,
                    crossAxisParentSize,
                    parentWidth,
                ) - paddingAndBorderAxisCross;
            }

            // If there's no flex wrap, the cross dimension is defined by the container.
            if !isNodeFlexWrap && measureModeCrossDim == MeasureMode::Exactly {
                crossDim = availableInnerCrossDim;
            }

            // Clamp to the min/max size specified on the container.
            crossDim = YGNodeBoundAxis(
                node,
                crossAxis,
                crossDim + paddingAndBorderAxisCross,
                crossAxisParentSize,
                parentWidth,
            ) - paddingAndBorderAxisCross;

            // STEP 7: CROSS-AXIS ALIGNMENT
            // We can skip child alignment if we're just measuring the container.
            if performLayout {
                for i in startOfLineIndex..endOfLineIndex {
                    let child = YGNodeListGet((*node).children, i);
                    if (*child).style.display == Display::None {
                        continue;
                    }
                    if (*child).style.positionType == YGPositionTypeAbsolute {
                        // If the child is absolutely positioned and has a
                        // top/left/bottom/right
                        // set, override all the previously computed positions to set it
                        // correctly.
                        let isChildLeadingPosDefined = YGNodeIsLeadingPosDefined(child, crossAxis);
                        if isChildLeadingPosDefined {
                            (*child).layout.position[pos[crossAxis as usize] as usize] =
                                YGNodeLeadingPosition(child, crossAxis, availableInnerCrossDim)
                                    + YGNodeLeadingBorder(node, crossAxis)
                                    + YGNodeLeadingMargin(child, crossAxis, availableInnerWidth);
                        }
                        // If leading position is not defined or calculations result in Nan, default to border + margin
                        if !isChildLeadingPosDefined
                            || (*child).layout.position[pos[crossAxis as usize] as usize].is_nan()
                        {
                            (*child).layout.position[pos[crossAxis as usize] as usize] =
                                YGNodeLeadingBorder(node, crossAxis)
                                    + YGNodeLeadingMargin(child, crossAxis, availableInnerWidth);
                        }
                    } else {
                        let mut leadingCrossDim = leadingPaddingAndBorderCross;

                        // For a relative children, we're either using alignItems (parent) or
                        // alignSelf (child) in order to determine the position in the cross
                        // axis
                        let alignItem = YGNodeAlignItem(node, child);

                        // If the child uses align stretch, we need to lay it out one more
                        // time, this time
                        // forcing the cross-axis size to be the computed cross size for the
                        // current line.
                        if alignItem == Align::Stretch
                            && (*YGMarginLeadingValue(child, crossAxis)).unit != YGUnitAuto
                            && (*YGMarginTrailingValue(child, crossAxis)).unit != YGUnitAuto
                        {
                            // If the child defines a definite size for its cross axis, there's
                            // no need to stretch.
                            if !YGNodeIsStyleDimDefined(child, crossAxis, availableInnerCrossDim) {
                                let mut childMainSize =
                                    (*child).layout.measuredDimensions[DIM[mainAxis as usize]];
                                let mut childCrossSize = if !(*child).style.aspectRatio.is_nan() {
                                    ((YGNodeMarginForAxis(child, crossAxis, availableInnerWidth)
                                        + (if isMainAxisRow {
                                            childMainSize / (*child).style.aspectRatio
                                        } else {
                                            childMainSize * (*child).style.aspectRatio
                                        })))
                                } else {
                                    crossDim
                                };

                                childMainSize +=
                                    YGNodeMarginForAxis(child, mainAxis, availableInnerWidth);

                                let mut childMainMeasureMode = MeasureMode::Exactly;
                                let mut childCrossMeasureMode = MeasureMode::Exactly;
                                YGConstrainMaxSizeForMode(
                                    child,
                                    mainAxis,
                                    availableInnerMainDim,
                                    availableInnerWidth,
                                    &mut childMainMeasureMode,
                                    &mut childMainSize,
                                );
                                YGConstrainMaxSizeForMode(
                                    child,
                                    crossAxis,
                                    availableInnerCrossDim,
                                    availableInnerWidth,
                                    &mut childCrossMeasureMode,
                                    &mut childCrossSize,
                                );

                                let childWidth = if isMainAxisRow {
                                    childMainSize
                                } else {
                                    childCrossSize
                                };
                                let childHeight = if !isMainAxisRow {
                                    childMainSize
                                } else {
                                    childCrossSize
                                };

                                let childWidthMeasureMode = if childWidth.is_nan() {
                                    MeasureMode::Undefined
                                } else {
                                    MeasureMode::Exactly
                                };
                                let childHeightMeasureMode = if childHeight.is_nan() {
                                    MeasureMode::Undefined
                                } else {
                                    MeasureMode::Exactly
                                };

                                YGLayoutNodeInternal(
                                    child,
                                    childWidth,
                                    childHeight,
                                    direction,
                                    childWidthMeasureMode,
                                    childHeightMeasureMode,
                                    availableInnerWidth,
                                    availableInnerHeight,
                                    true,
                                    "stretch",
                                    config,
                                );
                            }
                        } else {
                            let remainingCrossDim = containerCrossAxis
                                - YGNodeDimWithMargin(child, crossAxis, availableInnerWidth);

                            if (*YGMarginLeadingValue(child, crossAxis)).unit == YGUnitAuto
                                && (*YGMarginTrailingValue(child, crossAxis)).unit == YGUnitAuto
                            {
                                leadingCrossDim += (remainingCrossDim / 2.0).max(0.0);
                            } else if (*YGMarginTrailingValue(child, crossAxis)).unit == YGUnitAuto
                            {
                                // No-Op
                            } else if (*YGMarginLeadingValue(child, crossAxis)).unit == YGUnitAuto {
                                leadingCrossDim += 0.0f32.max(remainingCrossDim);
                            } else if alignItem == Align::FlexStart {
                                // No-Op
                            } else if alignItem == Align::Center {
                                leadingCrossDim += remainingCrossDim / 2.0;
                            } else {
                                leadingCrossDim += remainingCrossDim;
                            }
                        }
                        // And we apply the position
                        (*child).layout.position[pos[crossAxis as usize] as usize] +=
                            totalLineCrossDim + leadingCrossDim;
                    }
                }
            }

            totalLineCrossDim += crossDim;
            maxLineMainDim = maxLineMainDim.max(mainDim);
            lineCount += 1;
            startOfLineIndex = endOfLineIndex;
        }

        // STEP 8: MULTI-LINE CONTENT ALIGNMENT
        if performLayout
            && (lineCount > 1 || YGIsBaselineLayout(node))
            && !availableInnerCrossDim.is_nan()
        {
            let remainingAlignContentDim = availableInnerCrossDim - totalLineCrossDim;

            let mut crossDimLead = 0.0;
            let mut currentLead = leadingPaddingAndBorderCross;

            match (*node).style.alignContent {
                Align::FlexEnd => currentLead += remainingAlignContentDim,
                Align::Center => currentLead += remainingAlignContentDim / 2.0,
                Align::Stretch => if availableInnerCrossDim > totalLineCrossDim {
                    crossDimLead = remainingAlignContentDim / lineCount as f32;
                },
                Align::SpaceAround => if availableInnerCrossDim > totalLineCrossDim {
                    currentLead += remainingAlignContentDim / (2.0 * lineCount as f32);
                    if lineCount > 1 {
                        crossDimLead = remainingAlignContentDim / lineCount as f32;
                    }
                } else {
                    currentLead += remainingAlignContentDim / 2.0;
                },
                Align::SpaceBetween => {
                    if availableInnerCrossDim > totalLineCrossDim && lineCount > 1 {
                        crossDimLead = remainingAlignContentDim / (lineCount as f32 - 1.0);
                    }
                }
                _ => (),
            }

            let mut endIndex = 0;
            for i in 0..lineCount {
                let startIndex = endIndex;

                // compute the line's height and find the endIndex
                let mut lineHeight = 0.0f32;
                let mut maxAscentForCurrentLine = 0.0f32;
                let mut maxDescentForCurrentLine = 0.0f32;

                for ii in startIndex..childCount {
                    endIndex = ii;
                    let child = YGNodeListGet((*node).children, ii);
                    if (*child).style.display == Display::None {
                        continue;
                    }
                    if (*child).style.positionType == YGPositionTypeRelative {
                        if (*child).lineIndex != i {
                            break;
                        }
                        if YGNodeIsLayoutDimDefined(child, crossAxis) {
                            lineHeight = lineHeight.max(
                                (*child).layout.measuredDimensions[DIM[crossAxis as usize]]
                                    + YGNodeMarginForAxis(child, crossAxis, availableInnerWidth),
                            );
                        }
                        if YGNodeAlignItem(node, child) == Align::Baseline {
                            let ascent = YGBaseline(child)
                                + YGNodeLeadingMargin(
                                    child,
                                    FlexDirection::Column,
                                    availableInnerWidth,
                                );
                            let descent = (*child).layout.measuredDimensions.height
                                + YGNodeMarginForAxis(
                                    child,
                                    FlexDirection::Column,
                                    availableInnerWidth,
                                ) - ascent;
                            maxAscentForCurrentLine = maxAscentForCurrentLine.max(ascent);
                            maxDescentForCurrentLine = maxDescentForCurrentLine.max(descent);
                            lineHeight =
                                lineHeight.max(maxAscentForCurrentLine + maxDescentForCurrentLine);
                        }
                    }
                }
                lineHeight += crossDimLead;

                if performLayout {
                    for ii in startIndex..endIndex {
                        let child = YGNodeListGet((*node).children, ii);
                        if (*child).style.display == Display::None {
                            continue;
                        }
                        if (*child).style.positionType == YGPositionTypeRelative {
                            match YGNodeAlignItem(node, child) {
                                Align::FlexStart => {
                                    (*child).layout.position[pos[crossAxis as usize] as usize] =
                                        currentLead
                                            + YGNodeLeadingMargin(
                                                child,
                                                crossAxis,
                                                availableInnerWidth,
                                            );
                                }
                                Align::FlexEnd => {
                                    (*child).layout.position[pos[crossAxis as usize] as usize] =
                                        currentLead + lineHeight
                                            - YGNodeTrailingMargin(
                                                child,
                                                crossAxis,
                                                availableInnerWidth,
                                            )
                                            - (*child).layout.measuredDimensions
                                                [DIM[crossAxis as usize]];
                                }
                                Align::Center => {
                                    let mut childHeight =
                                        (*child).layout.measuredDimensions[DIM[crossAxis as usize]];
                                    (*child).layout.position[pos[crossAxis as usize] as usize] =
                                        currentLead + (lineHeight - childHeight) / 2.0;
                                }
                                Align::Stretch => {
                                    (*child).layout.position[pos[crossAxis as usize] as usize] =
                                        currentLead
                                            + YGNodeLeadingMargin(
                                                child,
                                                crossAxis,
                                                availableInnerWidth,
                                            );

                                    // Remeasure child with the line height as it as been only measured with the
                                    // parents height yet.
                                    if !YGNodeIsStyleDimDefined(
                                        child,
                                        crossAxis,
                                        availableInnerCrossDim,
                                    ) {
                                        let childWidth = if isMainAxisRow {
                                            ((*child).layout.measuredDimensions.width
                                                + YGNodeMarginForAxis(
                                                    child,
                                                    mainAxis,
                                                    availableInnerWidth,
                                                ))
                                        } else {
                                            lineHeight
                                        };

                                        let childHeight = if !isMainAxisRow {
                                            ((*child).layout.measuredDimensions.height
                                                + YGNodeMarginForAxis(
                                                    child,
                                                    crossAxis,
                                                    availableInnerWidth,
                                                ))
                                        } else {
                                            lineHeight
                                        };

                                        if !(YGFloatsEqual(
                                            childWidth,
                                            (*child).layout.measuredDimensions.width,
                                        )
                                            && YGFloatsEqual(
                                                childHeight,
                                                (*child).layout.measuredDimensions.height,
                                            )) {
                                            YGLayoutNodeInternal(
                                                child,
                                                childWidth,
                                                childHeight,
                                                direction,
                                                MeasureMode::Exactly,
                                                MeasureMode::Exactly,
                                                availableInnerWidth,
                                                availableInnerHeight,
                                                true,
                                                "multiline-stretch",
                                                config,
                                            );
                                        }
                                    }
                                }
                                Align::Baseline => {
                                    (*child).layout.position[Edge::Top as usize] =
                                        currentLead + maxAscentForCurrentLine - YGBaseline(child)
                                            + YGNodeLeadingPosition(
                                                child,
                                                FlexDirection::Column,
                                                availableInnerCrossDim,
                                            );
                                }
                                _ => (),
                            }
                        }
                    }
                }

                currentLead += lineHeight;
            }
        }

        // STEP 9: COMPUTING FINAL DIMENSIONS
        (*node).layout.measuredDimensions.width = YGNodeBoundAxis(
            node,
            FlexDirection::Row,
            availableWidth - marginAxisRow,
            parentWidth,
            parentWidth,
        );
        (*node).layout.measuredDimensions.height = YGNodeBoundAxis(
            node,
            FlexDirection::Column,
            availableHeight - marginAxisColumn,
            parentHeight,
            parentWidth,
        );

        // If the user didn't specify a width or height for the node, set the
        // dimensions based on the children.
        if measureModeMainDim == MeasureMode::Undefined
            || ((*node).style.overflow != YGOverflowScroll
                && measureModeMainDim == MeasureMode::AtMost)
        {
            // Clamp the size to the min/max size, if specified, and make sure it
            // doesn't go below the padding and border amount.
            (*node).layout.measuredDimensions[DIM[mainAxis as usize]] = YGNodeBoundAxis(
                node,
                mainAxis,
                maxLineMainDim,
                mainAxisParentSize,
                parentWidth,
            );
        } else if measureModeMainDim == MeasureMode::AtMost
            && (*node).style.overflow == YGOverflowScroll
        {
            (*node).layout.measuredDimensions[DIM[mainAxis as usize]] = (availableInnerMainDim
                + paddingAndBorderAxisMain)
                .min(YGNodeBoundAxisWithinMinAndMax(
                    node,
                    mainAxis,
                    maxLineMainDim,
                    mainAxisParentSize,
                ))
                .max(paddingAndBorderAxisMain);
        }

        if measureModeCrossDim == MeasureMode::Undefined
            || ((*node).style.overflow != YGOverflowScroll
                && measureModeCrossDim == MeasureMode::AtMost)
        {
            // Clamp the size to the min/max size, if specified, and make sure it
            // doesn't go below the padding and border amount.
            (*node).layout.measuredDimensions[DIM[crossAxis as usize]] = YGNodeBoundAxis(
                node,
                crossAxis,
                totalLineCrossDim + paddingAndBorderAxisCross,
                crossAxisParentSize,
                parentWidth,
            );
        } else if measureModeCrossDim == MeasureMode::AtMost
            && (*node).style.overflow == YGOverflowScroll
        {
            (*node).layout.measuredDimensions[DIM[crossAxis as usize]] = (availableInnerCrossDim
                + paddingAndBorderAxisCross)
                .max(YGNodeBoundAxisWithinMinAndMax(
                    node,
                    crossAxis,
                    totalLineCrossDim + paddingAndBorderAxisCross,
                    crossAxisParentSize,
                ))
                .max(paddingAndBorderAxisCross);
        }

        // As we only wrapped in normal direction yet, we need to reverse the positions on wrap-reverse.
        if performLayout && (*node).style.flexWrap == YGWrapWrapReverse {
            for i in 0..childCount {
                let child = YGNodeGetChild(node, i);
                if (*child).style.positionType == YGPositionTypeRelative {
                    (*child).layout.position[pos[crossAxis as usize] as usize] =
                        (*node).layout.measuredDimensions[DIM[crossAxis as usize]]
                            - (*child).layout.position[pos[crossAxis as usize] as usize]
                            - (*child).layout.measuredDimensions[DIM[crossAxis as usize]];
                }
            }
        }

        if performLayout {
            // STEP 10: SIZING AND POSITIONING ABSOLUTE CHILDREN
            currentAbsoluteChild = firstAbsoluteChild;
            while !currentAbsoluteChild.is_null() {
                YGNodeAbsoluteLayoutChild(
                    node,
                    currentAbsoluteChild,
                    availableInnerWidth,
                    if isMainAxisRow {
                        measureModeMainDim
                    } else {
                        measureModeCrossDim
                    },
                    availableInnerHeight,
                    direction,
                    config,
                );
                currentAbsoluteChild = (*currentAbsoluteChild).nextChild;
            }

            // STEP 11: SETTING TRAILING POSITIONS FOR CHILDREN
            let needsMainTrailingPos =
                mainAxis == FlexDirection::RowReverse || mainAxis == FlexDirection::ColumnReverse;
            let needsCrossTrailingPos =
                crossAxis == FlexDirection::RowReverse || crossAxis == FlexDirection::ColumnReverse;

            // Set trailing position if necessary.
            if needsMainTrailingPos || needsCrossTrailingPos {
                for i in 0..childCount {
                    let child = YGNodeListGet((*node).children, i);
                    if (*child).style.display == Display::None {
                        continue;
                    }
                    if needsMainTrailingPos {
                        YGNodeSetChildTrailingPosition(node, child, mainAxis);
                    }

                    if needsCrossTrailingPos {
                        YGNodeSetChildTrailingPosition(node, child, crossAxis);
                    }
                }
            }
        }
    }
}
