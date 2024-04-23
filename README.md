# Weird web page generator

![sample-linkpage](https://user-images.githubusercontent.com/583842/217551090-19bdf42a-8f5f-44be-a343-ef9b9891a759.png)

https://blog.erlend.sh/weird-web-pages

> #### Websites as the atomic matter of the internet
> 
> I consider the personal website to be the smallest possible building block of web identity. Once you wanna go past the observer (READ) level to the contributor (WRITE) level as a netizen, you’re gonna need a material web-persona to make yourself known.

> The size of the internet can be measured in the atomic mass of the websites it's made up of. We collectively materialize the internet with every additional web page we create.

## Current state

'Project Weird' has gone through several prototypical iterations over the past year. Our latest iteration – with which it feels like we're finally building on a firm foundation – is a local-first application built with Tauri + Svelte for the local-first frontend, and Zola for site generation.

This work is happening within the [`desktop`](https://github.com/commune-os/weird/tree/main/desktop) folder.

`common`, `backend` and `frontend` are all part of an inactive legacy app ([demo](https://nate-sys.github.io/links-app-perseus/)) that was cloud-first and made with Axum + Sycamore. Both the cloud and the Rust web dev stack have a place in Weird's future, but not in the MVP stage.
