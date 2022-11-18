use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sycamore::prelude::*;

#[derive(Deserialize)]
pub struct QueryResult {
    pub time: Value,
    pub status: String,
    pub result: Value,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub links: Vec<String>,
}

#[perseus::template]
#[sycamore::component]
pub fn greet_page<G: Html>(cx: Scope, user: User) -> View<G> {
    let full_name = user.name;
    let content = create_signal(cx, user.links);
    view! {
        cx,
        h2 { "Hello " (full_name) }
        div{
            Indexed(iterable=content, view = |cx, link| view!{
                cx,
                p {(link)}
            })
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("greet")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .incremental_generation()
        .template(greet_page)
}

#[perseus::build_paths]
pub async fn get_build_paths() -> RenderFnResult<Vec<String>> {
    Ok(Vec::new())
}

#[perseus::build_state]
pub async fn get_build_state(path: String, _locale: String) -> RenderFnResultWithCause<User> {
    // url to the db server instance
    const DB_URL: &str = "http://localhost:8000";

    // extract username from path
    let username = path.split('/').last().unwrap();

    // build request headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("NS", "weird".parse().unwrap());
    headers.insert("DB", "weird".parse().unwrap());

    // execute request
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{DB_URL}/key/user/{username}"))
        .basic_auth("root", Some("root"))
        .headers(headers)
        .send()
        .await?;

    // Deserialize resulting string to a surrealdb response. the response is a [time, status, result]
    let Ok(Value::Array(arr)) = serde_json::from_str(&response.text().await?) else { panic!("oopsie woop") };
    let QueryResult { result, .. }: QueryResult = serde_json::from_value(arr[0].clone()).unwrap();

    // extract the result
    let Value::Array(arr) = result else { panic!("couldnt get first thing") };

    Ok(serde_json::from_value(arr[0].clone()).unwrap())
}
