pub struct Pager;

#[derive(serde::Deserialize)]
struct Param {
    pager: crate::Pager,
    #[serde(default)]
    base_url: String,
    #[serde(default = "default_page_param")]
    page_param: String,
    #[serde(default = "default_limit_param")]
    limit_param: String,
    #[serde(default = "default_ellipsis_param")]
    ellipsis: usize,
}

fn default_page_param() -> String {
    "page".to_string()
}

fn default_limit_param() -> String {
    "limit".to_string()
}

fn default_ellipsis_param() -> usize {
    9
}

impl tera::Function<tera::TeraResult<String>> for Pager {
    fn call(&self, kwargs: tera::Kwargs, _: &tera::State<'_>) -> tera::TeraResult<String> {
        let param = kwargs.deserialize::<Param>()?;

        let config = crate::pager::Config {
            base_url: param.base_url,
            page_param: param.page_param,
            limit_param: param.limit_param,
            ellipsis: param.ellipsis,
        };

        let html = crate::html::pager(&param.pager, &config);

        Ok(html)
    }

    fn is_safe(&self) -> bool {
        true
    }
}
