use serde::Deserialize;
use std::{
    fs,
    io::{Read, Write},
};
use sycamore::prelude::*;

#[derive(Prop, Deserialize, Debug, Clone)]
struct UserData {
    github_username: String,
    full_name: String,
    links: Vec<String>,
}

#[component]
fn App<G: Html>(cx: Scope, user_data: UserData) -> View<G> {
    let links = create_signal(cx, user_data.links);
    view! {
        cx,
        h1 { (user_data.github_username) }
        h3 { (user_data.full_name) }
        ul {
            Indexed(iterable=links, view = |cx, x|{
                let x = create_ref(cx, x);
                view!{
                    cx,
                    li{
                        a(href=x.clone()){( x.clone() )}
                    }
                }
            })
        }
    }
}

fn main() {
    // Open from user-provided json file
    let mut data_file = fs::File::open("data.json").unwrap();

    // Deserialize file content
    let mut user_data_json = String::new();
    data_file.read_to_string(&mut user_data_json).unwrap();
    let user_data: UserData = serde_json::from_str(&user_data_json).unwrap();
    dbg!(&user_data);

    // Feed data to Sycamore
    let cloned_user_data = user_data.clone();
    let html = sycamore::render_to_string(|cx| view! {cx, App(cloned_user_data)});
    dbg!(&html);

    // Write to a file
    let mut html_file =
        fs::File::create(format!("{}_data.html", user_data.github_username)).unwrap();
    write!(html_file, "{}", html).unwrap();
}
