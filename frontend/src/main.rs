use reqwasm::http::Request;
use sycamore::{prelude::*, suspense::Suspense, view};
use sycamore_router::{HistoryIntegration, Route, Router};

use common::Link;

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
                                AppRoutes::Link(github_username) => view! { cx,
                                    LinkPage(github_username = github_username.clone())
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
fn LinkPage<G: Html>(cx: Scope<'_>, github_username: String) -> View<G> {
    view! {
        cx,
        Suspense(fallback = view!{cx, "loading... "}) {
            LinkPageChild(github_username = github_username)
        }
    }
}

#[component(inline_props)]
async fn LinkPageChild<G: Html>(cx: Scope<'_>, github_username: String) -> View<G> {
    match Request::get(&format!("http://127.0.0.1:3000/{github_username}"))
        .send()
        .await
    {
        Ok(data) => {
            if data.status() == 200 {
                match data.json::<Vec<Link>>().await {
                    Ok(d) => view! {cx, PageData(data=d)},
                    Err(e) => view! { cx, samp { (e) } },
                }
            } else {
                view! { cx, samp { "Error: failed to get user" } }
            }
        }
        Err(_) => view! { cx, samp { "Error: failed to get page" } },
    }
}

#[component(inline_props)]
fn PageData<G: Html>(cx: Scope, data: Vec<Link>) -> View<G> {
    let github_username = data[0].github_username.clone();
    let links = create_signal(cx, data);
    view!{ 
        cx,
        h2 { (github_username) }
        ul{ 
            Indexed(iterable=links, view = |cx, x| view!{cx,
                li { ( x.title ) " -> " (x.url) }
            })
        }
    }
}
