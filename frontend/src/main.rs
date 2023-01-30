use reqwasm::http::Request;
use sycamore::{prelude::*, view, suspense::Suspense};
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
    view!{
        cx,
        Suspense(fallback = view!{cx, "loading... "}) {
            LinkPageChild(github_username = github_username)
        }
    }
}

#[component(inline_props)]
async fn LinkPageChild<G: Html> (cx: Scope<'_>, github_username: String) -> View <G> {
    match Request::get(&format!("http://127.0.0.1:3000/{github_username}"))
            .send()
            .await
    {
        Ok(data) => {
            match data.json::<Vec<Link>>().await {
                Ok(text) => view!{ cx, samp { (format!("{:?}",text)) } } ,
                Err(e) => view!{ cx, samp { (e) } }
            }
        },
        Err(r) => view!{ cx, samp { (r) } }

    }
     
}
