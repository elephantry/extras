#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Pager {
    pub count: usize,
    pub page: usize,
    pub max_per_page: usize,
}

pub struct Config {
    pub base_url: String,
    pub page_param: String,
    pub limit_param: String,
    pub ellipsis: usize,
}
