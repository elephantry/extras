pub fn pager(pager: &crate::Pager, config: &crate::pager::Config) -> String {
    let mut html = String::new();

    let last_page = (pager.count as f32 / pager.max_per_page as f32).ceil() as usize;

    if last_page == 0 {
        return html;
    }

    let (start, end) = if pager.page <= config.ellipsis {
        (1, (config.ellipsis + 1).min(last_page))
    } else if pager.page >= last_page - config.ellipsis {
        (last_page - (config.ellipsis + 1), last_page)
    } else {
        let half = (config.ellipsis - 1) / 2;
        (pager.page - half, pager.page + half)
    };

    html.push_str(r#"<ul class="pagination">"#);
    if pager.page > 1 {
        let url = url(pager.page - 1, pager, config);
        html.push_str(&format!(
            r#"<li class="page-item"><a class="page-link" href="{}">«</a></li>"#,
            url
        ));
    } else {
        html.push_str(
            &r#"<li class="page-item disabled"><a class="page-link" href="\#">«</a></li>"#,
        );
    }

    if start > 1 {
        let url = url(1, pager, config);
        html.push_str(&format!(
            r#"
        <li class="page-item">
            <a class="page-link" href="{}">1</a>
        </li>
        <li class="page-item disabled">
            <a class="page-link" href="\#">…</a>
        </li>"#,
            url
        ));
    }

    for i in start..end + 1 {
        if i == pager.page {
            html.push_str(&format!(r#"<li class="page-item active"><a class="page-link" href="\#">{} <span class="sr-only">(current)</span></a></li>"#, pager.page));
        } else {
            let url = url(i, pager, config);
            html.push_str(&format!(
                r#"<li class="page-item"><a class="page-link" href="{}">{}</a></li>"#,
                url, i
            ));
        }
    }

    if end < last_page {
        let url = url(last_page, pager, config);
        html.push_str(&format!(
            r#"
        <li class="page-item disabled">
            <a class="page-link" href="\#">…</a>
        </li>
        <li class="page-item">
            <a class="page-link" href="{}">{}</a>
        </li>"#,
            url, last_page
        ));
    }

    if pager.page < last_page {
        let url = url(pager.page + 1, pager, config);
        html.push_str(&format!(
            r#"<li class="page-item"><a class="page-link" href="{}">»</a></li>"#,
            url
        ));
    } else {
        html.push_str(
            r#"<li class="page-item disabled"><a class="page-link" href="\#">»</a></li>"#,
        );
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
        "{}{}={}&{}={}",
        base_url, config.page_param, page, config.limit_param, pager.max_per_page
    )
}
