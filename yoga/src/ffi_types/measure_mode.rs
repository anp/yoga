#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum MeasureMode {
    // undefined was 1
    Exactly, // 1
    AtMost,  // 2
}

pub unsafe fn MeasureModeNewMeasureSizeIsStricterAndStillValid(
    mut sizeMode: MeasureMode,
    mut size: c_float,
    mut lastSizeMode: MeasureMode,
    mut lastSize: c_float,
    mut lastComputedSize: c_float,
) -> bool {
    return lastSizeMode == MeasureMode::AtMost
        && sizeMode == MeasureMode::AtMost
        && lastSize > size
        && (lastComputedSize <= size || YGFloatsEqual(size, lastComputedSize));
}
pub unsafe fn MeasureModeOldSizeIsUnspecifiedAndStillFits(
    mut sizeMode: MeasureMode,
    mut size: c_float,
    mut lastSizeMode: MeasureMode,
    mut lastComputedSize: c_float,
) -> bool {
    return sizeMode == MeasureMode::AtMost
        && lastSizeMode == MeasureMode::Undefined
        && (size >= lastComputedSize || YGFloatsEqual(size, lastComputedSize));
}
pub unsafe fn MeasureModeSizeIsExactAndMatchesOldMeasuredSize(
    mut sizeMode: MeasureMode,
    mut size: c_float,
    mut lastComputedSize: c_float,
) -> bool {
    return sizeMode == MeasureMode::Exactly && YGFloatsEqual(size, lastComputedSize);
}
