#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pagination {
    #[cfg_attr(
        feature = "serde",
        serde(default = "default_page", deserialize_with = "parse")
    )]
    pub page: usize,
    #[cfg_attr(
        feature = "serde",
        serde(default = "default_limit", deserialize_with = "parse")
    )]
    pub limit: usize,
}

#[cfg(feature = "serde")]
fn default_page() -> usize {
    1
}

#[cfg(feature = "serde")]
fn default_limit() -> usize {
    20
}

impl Pagination {
    pub fn to_sql(&self) -> String {
        format!(
            "offset {} fetch first {} rows only",
            (self.page - 1) * self.limit,
            self.limit,
        )
    }
}

#[cfg(feature = "serde")]
fn parse<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;

    let s = String::deserialize(deserializer)?;

    s.parse().map_err(D::Error::custom)
}
