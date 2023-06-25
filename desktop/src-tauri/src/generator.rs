use crate::{
    error::Error,
    prelude::*,
    state::{Content, Links},
};
use html_editor::{operation::*, parse, Node};
use std::fs;
use tauri::AppHandle;

/// Inject to the `index.html`
pub fn generate(content: Content, handle: &AppHandle) -> Result<()> {
    let index_file = handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("template/index.html");
    let mut dom = parse(fs::read_to_string(&index_file)?.as_str())
        .map_err(Error::HtmlParse)?;

    update_dom(&mut dom, &content)?;

    let html = dom.html();
    fs::write(index_file, html)?;
    Ok(())
}

/// Update parsed dom with the data in `Content`
fn update_dom(dom: &mut Vec<Node>, content: &Content) -> Result<()> {
    if let Some(photo) = content.user.photo.clone() {
        let Some(node) = dom.query_mut(&Selector::from("img#photo")) else {
            return Err(Error::HtmlParse("img#photo not found".to_owned()))
        };
        node.attrs.iter_mut().for_each(|attr| {
            if attr.0.as_str() == "href" {
                attr.1 = photo.clone()
            }
        });
    };
    let Some(mut node) = dom.query_mut(&Selector::from("div#name")) else {
        return Err(Error::HtmlParse("div#name not found".to_owned()))
    };
    node.children = vec![Node::Text(content.user.name.clone())];
    let Some(mut node) = dom.query_mut(&Selector::from("div#username")) else {
        return Err(Error::HtmlParse("div#username not found".to_owned()))
    };
    node.children = vec![Node::Text(content.user.username.clone())];
    let Some(mut node) = dom.query_mut(&Selector::from("div#title")) else {
        return Err(Error::HtmlParse("div#title not found".to_owned()))
    };
    node.children = vec![Node::Text(content.user.title.clone())];
    let Some(mut node) = dom.query_mut(&Selector::from("div#about")) else {
        return Err(Error::HtmlParse("div#about not found".to_owned()))
    };
    node.children = vec![Node::Text(content.user.about.clone())];

    let Some(mut links_node) = dom.query_mut(&Selector::from("div#links")) else {
        return Err(Error::HtmlParse("div#links not found".to_owned()))
    };
    links_node.children = generate_links(content.links.as_ref());
    Ok(())
}

/// Generate vector of link elements to inject into the dom
fn generate_links(links: &Links) -> Vec<Node> {
    let links: Vec<Node> = links
        .iter()
        .map(|l| {
            Node::new_element(
                "div",
                vec![("class", "link")],
                vec![Node::new_element(
                    "a",
                    vec![("href", l.url.as_str()), ("target", "_blank")],
                    vec![
                        Node::new_element(
                            "div",
                            vec![("class", "link-text")],
                            vec![Node::Text(l.text.clone())],
                        ),
                        Node::new_element(
                            "div",
                            vec![("class", "link-url")],
                            vec![Node::Text(l.url.clone())],
                        ),
                    ],
                )],
            )
        })
        .collect();
    links
}
