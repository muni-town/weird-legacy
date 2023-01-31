use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::{futures::spawn_local_scoped, prelude::*, suspense::Suspense, view};
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
                                    CreatePage()
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
    view! {
        cx,
        h2 { (github_username) }
        ul{
            Indexed(iterable=links, view = |cx, x| view!{cx,
                li { ( x.title ) " -> " (x.url) }
            })
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct LinkData {
    title: RcSignal<String>,
    url: RcSignal<String>,
    #[serde(skip_serializing)]
    id: u32,
}
impl LinkData {
    fn to_link(&self, github_username: String) -> Link {
        Link{
            title: (*self.title.get()).clone(),
            url: (*self.url.get()).clone(),
            github_username,
        }
    }
}

type LinkDataRX = RcSignal<Vec<LinkData>>;
#[component]
fn CreatePage<G: Html>(cx: Scope) -> View<G> {
    let data: LinkDataRX = create_rc_signal(Vec::new());
    let github_username = create_signal(cx, String::new());
    provide_context(cx, data);
    let add = move |_| {
        let data = use_context::<LinkDataRX>(cx);
        let id = if let Some(last) = data.get().last() {
            last.id + 1
        } else {
            0
        };
        data.modify().push(LinkData {
            title: create_rc_signal(String::new()),
            url: create_rc_signal(String::new()),
            id,
        })
    };
    let create_page = move |_| {
        let data = use_context::<LinkDataRX>(cx);
        spawn_local_scoped(cx, async {
            let filtered_data = data
                .get()
                .as_ref()
                .iter()
                .filter(|x| !x.title.get().is_empty())
                .filter(|x| !x.url.get().is_empty())
                .map(|x| x.to_link(github_username.get().clone().to_string())).collect::<Vec<Link>>();
            Request::post("http://127.0.0.1:3000/create")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&filtered_data).unwrap())
                .send()
                .await
                .unwrap();
        });
    };
    let data = use_context::<LinkDataRX>(cx);
    view! {
        cx,
        h2 { "Create a page" }
        input(placeholder="github username ...", bind:value = github_username)
        button(on:click=add) { "new" }
        div(class="data-container") {
            Keyed(iterable = &data, view = |cx, x| {
                let item = create_ref(cx, x);
                let delete = move |_| {
                    let data = use_context::<LinkDataRX>(cx);
                    data.modify().retain(|y| y.id != item.id)
                };
                view!{ cx,
                    span{
                        input(placeholder="title...", bind:value=item.title)
                        input(placeholder="url...", bind:value=item.url)
                        button(on:click = delete) {"remove"}
                    }
                }
            },
            key = |x| x.id)
        }
        button(on:click=create_page){"Create"}
    }
}
