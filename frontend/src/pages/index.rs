use sycamore::prelude::*;

#[component]
pub fn IndexPage<G:Html>(cx: Scope) -> View<G> {
    view!{
        cx,
        h2 { "Welcome to Weird!" }
        p { "description text" }
        a(href="/create") { "Create a page" }
    } 
}
