use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};


mod pages;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/create")]
    Create,
    #[to("/link/<id>")]
    Link(String),
    #[not_found]
    NotFound,
}

fn main() {
    sycamore::render(|cx| {
        view! {
            cx,
            Router(
                integration=HistoryIntegration::new(),
                view=|cx, route: &ReadSignal<AppRoutes>| {
                    view! {
                        cx,
                        div(class="app") {
                            (match route.get().as_ref() {
                                AppRoutes::Index => view! { cx,
                                    pages::index::IndexPage()
                                },
                                AppRoutes::Create => view! { cx,
                                    pages::create::CreatePage()
                                },
                                AppRoutes::Link(github_username) => view! { cx,
                                    pages::linkpage::LinkPage(github_username = github_username.clone())
                                },
                                AppRoutes::NotFound => view! { cx,
                                    "404 Not Found"
                                },
                            })
                        }
                    }
                }
            )
        }
    });
}


