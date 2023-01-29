use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

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
                                    "This is the index page"
                                },
                                AppRoutes::Create => view! { cx,
                                    "In construction..."
                                },
                                AppRoutes::Link(id) => view! { cx,
                                    LinkPage(id = id.clone())
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

#[component(inline_props)]
fn LinkPage<G: Html>(cx: Scope, id: String) -> View<G> {
    view! {
        cx,
        h2 { (id) "'s page" }
        p { "Some content" }
    }
}
