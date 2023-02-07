use common::Link;
use reqwasm::http::Request;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

#[component(inline_props)]
pub fn LinkPage<G: Html>(cx: Scope<'_>, github_username: String) -> View<G> {
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
    view! {
        cx,
        Suspense(fallback=view!{cx, "loading..."})  {
            PageDataChild(data=data)
        }
    }
}

#[component(inline_props)]
async fn PageDataChild<G: Html>(cx: Scope<'_>, data: Vec<Link>) -> View<G> {
    let github_username = data[0].github_username.clone();
    let gh_profile: GithubData =
        Request::get(&format!("http://api.github.com/users/{github_username}"))
            .header("accept", "application/vnd.github+json")
            .send()
            .await
            .unwrap()
            .json::<GithubData>()
            .await
            .expect("failed to parse data");
    let links = create_signal(cx, data);
    let avatar = gh_profile.avatar_url.unwrap_or("http://picsum.photos/400/400".into()).clone();
    let name = gh_profile.name.unwrap_or("failed to get name".into()).clone();
    view! {
        cx,
        div(class="container"){ 
            img(src=avatar, class="img-avatar") {}
            h2(class="linkpage-name") { (name) }
            h3(class="linkpage-username") { (github_username) }
            fieldset(class="links-container") {
                legend{"Links"}
                Indexed(iterable=links, view = |cx, item| {
                    let title = item.title;
                    let url = create_signal(cx, item.url );
                    view!{
                        cx,
                        div(class="link"){ 
                            div(class="link-title"){(title)}
                            a(href=url.get(), rel="external", class="link-url"){(url.get())}
                        }
                    }
                })
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubData {
    #[serde(rename = "type")]
    pub  type_field:           Option<String>,
    pub  login:                Option<String>,
    pub  id:                   Option<i64>,
    pub  node_id:              Option<String>,
    pub  avatar_url:           Option<String>,
    pub  gravatar_id:          Option<String>,
    pub  url:                  Option<String>,
    pub  html_url:             Option<String>,
    pub  followers_url:        Option<String>,
    pub  following_url:        Option<String>,
    pub  gists_url:            Option<String>,
    pub  starred_url:          Option<String>,
    pub  subscriptions_url:    Option<String>,
    pub  organizations_url:    Option<String>,
    pub  repos_url:            Option<String>,
    pub  events_url:           Option<String>,
    pub  received_events_url:  Option<String>,
    pub  site_admin:           Option<bool>,
    pub  name:                 Option<String>,
    pub  company:              Option<Value>,
    pub  blog:                 Option<String>,
    pub  location:             Option<String>,
    pub  email:                Option<Value>,
    pub  hireable:             Option<Value>,
    pub  bio:                  Option<String>,
    pub  twitter_username:     Option<Value>,
    pub  public_repos:         Option<i64>,
    pub  public_gists:         Option<i64>,
    pub  followers:            Option<i64>,
    pub  following:            Option<i64>,
    pub  created_at:           Option<String>,
    pub  updated_at:           Option<String>,
}
