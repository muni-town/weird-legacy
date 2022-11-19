use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// Only used for sycamore rendering
#[derive(Serialize, Debug, Deserialize)]
struct State {
    #[serde(skip_serializing)]
    pub username: RcSignal<String>,
    pub name: RcSignal<String>,
    pub links: RcSignal<Vec<InputLink>>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
struct InputLink {
    #[serde(skip_serializing)]
    id: usize,
    site: RcSignal<String>,
    url: RcSignal<String>,
}

#[perseus::template_rx]
pub fn create_page<G: Html>(cx: Scope) -> View<G> {
    let state = State {
        username: create_rc_signal("".into()),
        name: create_rc_signal("".into()),
        links: create_rc_signal(Vec::new()),
    };
    provide_context(cx, state);

    // add a new links to the "form"
    let add = move |_| {
        let state = use_context::<State>(cx);

        // index logic used for rendering the list of inputfields
        let last_ind = if state.links.get().is_empty() {
            0
        } else {
            (*state.links.get()).last().unwrap().id + 1
        };

        // instanciate and push a new link to the list
        let new_link = InputLink {
            id: last_ind,
            site: create_rc_signal("".into()),
            url: create_rc_signal("".into()),
        };
        state.links.modify().push(new_link);
    };

    // creating a new page
    let submit = move |_| {
        #[cfg(target_arch = "wasm32")]
        let state = use_context::<State>(cx);

        #[cfg(target_arch = "wasm32")]
        perseus::spawn_local_scoped(cx, async move {
            const DB_URL: &str = "http://localhost:8000";

            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Accept", "application/json".parse().unwrap());
            headers.insert("NS", "weird".parse().unwrap());
            headers.insert("DB", "weird".parse().unwrap());

            // execute request
            let client = reqwest::Client::new();
            let _ = client
                .post(format!("{DB_URL}/key/user/{}", state.username.get()))
                .basic_auth("root", Some("root"))
                .headers(headers)
                .body(serde_json::to_value(state).unwrap().to_string())
                .send()
                .await
                .unwrap();

            perseus::navigate(&format!("/links/{}", state.username.get()));
        });
    };

    let state = use_context::<State>(cx);
    view! { cx,
        h2 { "Form" }
        input(bind:value = state.username, placeholder="Github Username")
        br{}
        input(bind:value = state.name, placeholder="Display Name")
        ul {
            Indexed(iterable=&state.links, view = |cx, x| {
                let current_elem = create_ref(cx, x);
                let state = use_context::<State>(cx);
                let delete = move |_| {
                    state.links.modify().retain(|item| item.id != current_elem.id);
                };
                view!{ cx,
                    li {
                        input(bind:value = current_elem.site)
                        input(bind:value = current_elem.url)
                        button(on:click = delete) {"delete"}
                    }

                }
            })
        }
        button(on:click=add) {"+"}
        button(on:click=submit) {"submit"}

    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("create").template(create_page)
}
