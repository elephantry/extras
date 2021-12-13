pub enum Message {
    Click((yew::MouseEvent, usize)),
}

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Properties {
    pub value: crate::Pager,
    #[prop_or_default]
    pub base_url: String,
    #[prop_or_default]
    pub page_param: String,
    #[prop_or_default]
    pub limit_param: String,
    #[prop_or_default]
    pub ellipsis: usize,
    #[prop_or_default]
    pub onclick: Option<yew::Callback<usize>>,
}

pub struct Pager {
    pager: crate::Pager,
    config: crate::pager::Config,
    onclick: Option<yew::Callback<usize>>,
}

impl Pager {
    fn url(&self, page: usize, limit: usize) -> String {
        let mut url = self.config.base_url.clone();

        if url.is_empty() {
            url = "?".to_string();
        } else if !url.contains('?') {
            url.push('?');
        } else {
            url.push('&');
        }

        format!(
            "{}{}={}&{}={}",
            url, self.config.page_param, page, self.config.limit_param, limit
        )
    }
}

impl yew::Component for Pager {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let props = ctx.props().clone();

        let config = crate::pager::Config {
            base_url: props.base_url,
            page_param: if props.page_param.is_empty() {
                "page".to_string()
            } else {
                props.page_param
            },
            limit_param: if props.limit_param.is_empty() {
                "limit".to_string()
            } else {
                props.limit_param
            },
            ellipsis: if props.ellipsis == 0 {
                9
            } else {
                props.ellipsis
            },
        };

        Self {
            config,
            pager: props.value,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        let Self::Message::Click((event, page)) = msg;
        if let Some(onclick) = &self.onclick {
            event.prevent_default();
            onclick.emit(page);
        }

        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let last_page = (self.pager.count as f32 / self.pager.max_per_page as f32).ceil() as usize;

        if last_page <= 1 {
            return "".into();
        }

        let (start, end) = if self.pager.page <= self.config.ellipsis {
            (1, (self.config.ellipsis + 1).min(last_page))
        } else if self.pager.page >= last_page - self.config.ellipsis {
            (last_page - (self.config.ellipsis + 1), last_page)
        } else {
            let half = (self.config.ellipsis - 1) / 2;
            (self.pager.page - half, self.pager.page + half)
        };

        yew::html! {
            <ul class="pagination justify-content-center">
            {
                if self.pager.page > 1 {
                    let page = self.pager.page - 1;

                    yew::html! {
                        <li class="page-item">
                            <a
                                class="page-link"
                                href={ self.url(page, self.pager.max_per_page) }
                                onclick={ ctx.link().callback(move |e| Self::Message::Click((e, page))) }
                            >{ "«" }</a>
                        </li>
                    }
                } else {
                    yew::html! {
                        <li class="page-item disabled">
                            <a class="page-link" href="#">{ "«" }</a>
                        </li>
                    }
                }
            }
            {
                if start > 1 {
                    yew::html! {
                        <>
                            <li class="page-item">
                                <a
                                    class="page-link"
                                    href={ self.url(1, self.pager.max_per_page) }
                                    onclick={ ctx.link().callback(|e| Self::Message::Click((e, 1))) }
                                >{ "1" }</a>
                            </li>
                            <li class="page-item disabled">
                                <a class="page-link" href="#">{ "…" }</a>
                            </li>
                        </>
                    }
                } else {
                    "".into()
                }
            }
            {
                for (start..end + 1).map(|i| if i == self.pager.page {
                        yew::html! {
                            <li class="page-item active"><a class="page-link" href="#">{ self.pager.page } <span class="sr-only">{ "(current)" }</span></a></li>
                        }
                    } else {
                        yew::html! {
                            <li class="page-item">
                                <a
                                    class="page-link"
                                    href={ self.url(i, self.pager.max_per_page) }
                                    onclick={ ctx.link().callback(move |e| Self::Message::Click((e, i))) }
                                >{ i }</a></li>
                        }
                    })
            }
            {
                if end < last_page {
                    yew::html! {
                        <>
                            <li class="page-item disabled">
                                <a class="page-link" href="#">{ "…" }</a>
                            </li>
                            <li class="page-item">
                                <a
                                    class="page-link"
                                    href={ self.url(last_page, self.pager.max_per_page) }
                                    onclick={ ctx.link().callback(move |e| Self::Message::Click((e, last_page))) }
                                >{ last_page }</a>
                            </li>
                        </>
                    }
                } else {
                    "".into()
                }
            }
            {
                if self.pager.page < last_page {
                    let page = self.pager.page + 1;

                    yew::html! {
                        <li class="page-item">
                            <a
                                class="page-link"
                                href={ self.url(page, self.pager.max_per_page) }
                                onclick={ ctx.link().callback(move |e| Self::Message::Click((e, page))) }
                            >{ "»" }</a>
                        </li>
                    }
                } else {
                    yew::html! {
                        <li class="page-item disabled">
                            <a class="page-link" href="#">{ "»" }</a>
                        </li>
                    }
                }
            }
            </ul>
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        self.pager = ctx.props().value.clone();

        true
    }
}
