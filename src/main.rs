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
    // open user-provided json file
    let mut data_file = fs::File::open("data.json").unwrap();

    // Deserialize file content
    let mut user_data_json = String::new();
    data_file.read_to_string(&mut user_data_json).unwrap();
    let user_data: UserData = serde_json::from_str(&user_data_json).unwrap();

    // Feed data to Sycamore
    let cloned_user_data = user_data.clone();
    let html = sycamore::render_to_string(|cx| view! {cx, App(cloned_user_data)});

    // Open the index file
    let mut html_file = fs::File::open("index.html").unwrap();
    let mut index_template = String::new();
    html_file.read_to_string(&mut index_template).unwrap();

    // Create a new page from the template
    let mut generated_page =
        fs::File::create(format!("{}_data.html", user_data.github_username)).unwrap();
    write!(
        generated_page,
        "{}",
        index_template
            .replace("%sycamore-body%", &html)
            .replace("%sycamore-title%", &user_data.full_name)
    )
    .unwrap();

    println!(
        "Page: {}_data.html has been generated.",
        user_data.github_username
    );
}
