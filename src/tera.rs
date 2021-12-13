pub struct Pager;

macro_rules! get_arg {
    ($args:ident, $name:literal, $ty:ty) => {
        match $args.get($name) {
            Some(value) => tera::from_value::<$ty>(value.clone())?,
            None => {
                return Err(tera::Error::msg(format!(
                    "Function `{}` require pager argument",
                    $name
                )))
            }
        }
    };

    ($args:ident, $name:literal, $ty:ty, $default:expr) => {
        match $args.get($name) {
            Some(value) => tera::from_value::<$ty>(value.clone())?,
            None => $default,
        }
    };
}

impl tera::Function for Pager {
    fn call(
        &self,
        args: &std::collections::HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        let pager = get_arg!(args, "pager", crate::Pager);
        let base_url = get_arg!(args, "base_url", String, String::new());
        let page_param = get_arg!(args, "page_param", String, "page".to_string());
        let limit_param = get_arg!(args, "limit_param", String, "limit".to_string());
        let ellipsis = get_arg!(args, "ellipsis", usize, 9);

        let config = crate::pager::Config {
            base_url,
            page_param,
            limit_param,
            ellipsis,
        };

        let html = crate::html::pager(&pager, &config);

        Ok(html.into())
    }

    fn is_safe(&self) -> bool {
        true
    }
}
