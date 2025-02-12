use std::fmt::Write;

pub fn pager(pager: &crate::Pager, config: &crate::pager::Config) -> String {
    let mut html = String::new();

    if pager.not_needed() {
        return html;
    }

    let last_page = pager.last_page();
    let bounds = crate::pager::Bounds::new(pager, config);

    html.push_str(r#"<ul class="pagination">"#);
    if pager.page > 1 {
        let url = url(pager.page - 1, pager, config);
        write!(
            html,
            r#"<li class="page-item"><a class="page-link" href="{url}">«</a></li>"#
        )
        .ok();
    } else {
        html.push_str(
            r#"<li class="page-item disabled"><a class="page-link" href="\#">«</a></li>"#,
        );
    }

    if bounds.start > 1 {
        let url = url(1, pager, config);
        write!(
            html,
            r#"
        <li class="page-item">
            <a class="page-link" href="{url}">1</a>
        </li>
        <li class="page-item disabled">
            <a class="page-link" href="\#">…</a>
        </li>"#
        )
        .ok();
    }

    for i in bounds.start..bounds.end + 1 {
        if i == pager.page {
            write!(
                html,
                r#"<li class="page-item active"><a class="page-link" href="\#">{}</a></li>"#,
                pager.page
            )
            .ok();
        } else {
            let url = url(i, pager, config);
            write!(
                html,
                r#"<li class="page-item"><a class="page-link" href="{url}">{i}</a></li>"#
            )
            .ok();
        }
    }

    if bounds.end < last_page {
        let url = url(last_page, pager, config);
        write!(
            html,
            r#"
        <li class="page-item disabled">
            <a class="page-link" href="\#">…</a>
        </li>
        <li class="page-item">
            <a class="page-link" href="{url}">{last_page}</a>
        </li>"#
        )
        .ok();
    }

    if pager.page < last_page {
        let url = url(pager.page + 1, pager, config);
        write!(
            html,
            r#"<li class="page-item"><a class="page-link" href="{url}">»</a></li>"#
        )
        .ok();
    } else {
        write!(
            html,
            r#"<li class="page-item disabled"><a class="page-link" href="\#">»</a></li>"#
        )
        .ok();
    }

    html.push_str("</ul>");

    html
}

fn url(page: usize, pager: &crate::Pager, config: &crate::pager::Config) -> String {
    let mut base_url = config.base_url.clone();

    if base_url.is_empty() {
        base_url = "?".to_string();
    } else if !base_url.contains('?') {
        base_url.push('?');
    } else {
        base_url.push('&');
    }

    format!(
        "{base_url}{}={page}&{}={}",
        config.page_param, config.limit_param, pager.max_per_page
    )
}
