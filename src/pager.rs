#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Pager {
    pub count: usize,
    pub page: usize,
    pub max_per_page: usize,
}

impl Pager {
    pub(crate) fn last_page(&self) -> usize {
        (self.count as f32 / self.max_per_page as f32).ceil() as usize
    }

    pub(crate) fn not_needed(&self) -> bool {
        self.last_page() <= 1
    }
}

#[derive(Debug, Default)]
pub struct Config {
    pub base_url: String,
    pub page_param: String,
    pub limit_param: String,
    pub ellipsis: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Bounds {
    pub start: usize,
    pub end: usize,
}

impl Bounds {
    pub fn new(pager: &Pager, config: &Config) -> Self {
        let last_page = pager.last_page();

        let (start, end) = if pager.page <= config.ellipsis {
            (1, (config.ellipsis + 1).min(last_page))
        } else if pager.page >= last_page - config.ellipsis {
            ((last_page - config.ellipsis).max(1), last_page)
        } else {
            let half = (config.ellipsis - 1) / 2;
            (pager.page - half, pager.page + half)
        };

        Self { start, end }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn bounds() {
        let pager = crate::Pager {
            count: 237,
            page: 10,
            max_per_page: 25,
        };
        let config = crate::pager::Config {
            ellipsis: 9,

            ..Default::default()
        };
        let bounds = crate::pager::Bounds::new(&pager, &config);

        assert_eq!(bounds, crate::pager::Bounds { start: 1, end: 10 });
    }
}
