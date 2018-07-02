#[must_use]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) enum Updated {
    Dirty,
    Clean,
}
