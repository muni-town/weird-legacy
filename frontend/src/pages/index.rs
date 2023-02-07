use sycamore::prelude::*;

#[component]
pub fn IndexPage<G:Html>(cx: Scope) -> View<G> {
    view!{
        cx,
        div(class="container"){ 
            h2 { "Welcome to Weird!" }
            p { "Local-first linkspage generator" }
            a(href="/create") { "Create a page" }
        }
    } 
}
