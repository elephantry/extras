impl<E: elephantry::Entity> From<&elephantry::Pager<E>> for crate::Pager {
    fn from(value: &elephantry::Pager<E>) -> crate::Pager {
        Self {
            count: value.count(),
            page: value.page(),
            max_per_page: value.max_per_page(),
        }
    }
}

pub fn pager<E: elephantry::Entity>(pager: &elephantry::Pager<E>) -> String {
    pager_with_config(pager, &crate::pager::Config::default())
}

pub fn pager_with_config<E: elephantry::Entity>(
    pager: &elephantry::Pager<E>,
    config: &crate::pager::Config,
) -> String {
    crate::html::pager(&pager.into(), config)
}
