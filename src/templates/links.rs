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

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Link {
    site: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub links: Vec<Link>,
}

#[perseus::template]
#[sycamore::component]
pub fn links_page<G: Html>(cx: Scope, user: User) -> View<G> {
    let full_name = user.name;
    let content = create_signal(cx, user.links);
    view! {
        cx,
        h2 { (full_name) }
        ul{
            Indexed(iterable=content, view = |cx, link| view!{
                cx,
                li{a(href=link.url, rel="external"){( link.site )}}
            })
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("links")
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
        .incremental_generation()
        .template(links_page)
}

#[perseus::build_paths]
pub async fn get_build_paths(path: String, _locale: String) -> RenderFnResult<Vec<String>> {
    const DB_URL: &str = "http://localhost:8000";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("NS", "weird".parse().unwrap());
    headers.insert("DB", "weird".parse().unwrap());

    // execute request
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{DB_URL}/key/user"))
        .basic_auth("root", Some("root"))
        .headers(headers)
        .send()
        .await?;

    // Deserialize resulting string to a surrealdb response. the response is a [time, status, result]
    let Ok(Value::Array(arr)) = serde_json::from_str(&response.text().await?) else { panic!("couldn't get result array") };

    // Get the result field from the response which also is an array
    let QueryResult { result, .. }: QueryResult = serde_json::from_value(arr[0].clone()).unwrap();
    let Value::Array(arr) = result else { panic!("couldnt get first element") };

    let links_arr = arr
        .into_iter()
        .map(|v| serde_json::from_value::<User>(v).unwrap())
        .map(|u| u.id[5..].to_string())
        .collect::<Vec<String>>();
    Ok(links_arr)
}

#[perseus::build_state]
pub async fn get_build_state(path: String, _locale: String) -> RenderFnResultWithCause<User> {
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
    let Ok(Value::Array(arr)) = serde_json::from_str(&response.text().await?) else { panic!("couldn't get result array") };

    // Get the result field from the response which also is an array
    let QueryResult { result, .. }: QueryResult = serde_json::from_value(arr[0].clone()).unwrap();

    //extract the first result (in this case the query always returns one object so its okay)
    let Value::Array(arr) = result else { panic!("couldnt get first element") };

    Ok(serde_json::from_value(arr[0].clone()).unwrap())
}
