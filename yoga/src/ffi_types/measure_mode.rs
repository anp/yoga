use internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum MeasureMode {
    Undefined = 0,
    Exactly = 1,
    AtMost = 2,
}

impl From<MeasureMode> for internal::YGMeasureMode {
    fn from(m: MeasureMode) -> internal::YGMeasureMode {
        match m {
            MeasureMode::Undefined => internal::YGMeasureModeUndefined,
            MeasureMode::Exactly => internal::YGMeasureModeExactly,
            MeasureMode::AtMost => internal::YGMeasureModeAtMost,
        }
    }
}
