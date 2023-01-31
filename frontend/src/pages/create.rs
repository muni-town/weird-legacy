use common::Link;
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use sycamore::{prelude::*, futures::spawn_local_scoped};


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
pub fn CreatePage<G: Html>(cx: Scope) -> View<G> {
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