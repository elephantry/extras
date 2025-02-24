#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "salvo", derive(salvo::macros::Extractible))]
#[cfg_attr(feature = "salvo", salvo(extract(default_source(from = "query"))))]
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

fn default_page() -> usize {
    1
}

fn default_limit() -> usize {
    20
}

impl Pagination {
    #[must_use]
    pub fn new() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
        }
    }

    #[must_use]
    pub fn to_sql(&self) -> String {
        format!(
            "offset {} fetch first {} rows only",
            (self.page - 1) * self.limit,
            self.limit,
        )
    }

    #[must_use]
    pub fn to_query(self) -> String {
        format!("page={}&limit={}", self.page, self.limit)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "serde")]
fn parse<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    use serde::de::Error;

    let s = String::deserialize(deserializer)?;

    s.parse().map_err(D::Error::custom)
}
