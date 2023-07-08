use libs::percent_encoding;
use libs::relative_path::RelativePathBuf;
use log::{debug, error};
use mime_guess::from_path;
use site::{Site, SITE_CONTENT};
use std::io::Cursor;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::{fs, thread};
use std::{sync::mpsc::Receiver, sync::Arc};
use tiny_http::{Header, Method, Request, Response, Server};

use crate::prelude::*;

// This is dist/livereload.min.js from the LiveReload.js v3.2.4 release
const LIVE_RELOAD: &str = include_str!("livereload.js");

fn create_new_site(
    root_dir: &Path,
    port: u16,
    output_dir: &PathBuf,
    base_url: &str,
    config_file: &Path,
    ws_port: Option<u16>,
) -> Result<(Site, String)> {
    SITE_CONTENT.write().unwrap().clear();

    let mut site = Site::new(root_dir, config_file)?;
    let address = format!("localhost:{}", port);

    let base_url = if base_url == "/" {
        String::from("/")
    } else {
        let base_address = format!("{}:{}", base_url, port);

        if site.config.base_url.ends_with('/') {
            format!("http://{}/", base_address)
        } else {
            format!("http://{}", base_address)
        }
    };

    site.enable_serve_mode();
    site.set_base_url(base_url);
    site.set_output_path(output_dir);
    site.load()?;
    if let Some(p) = ws_port {
        site.enable_live_reload_with_port(p);
    } else {
        site.enable_live_reload(port);
    }
    site.build()?;
    Ok((site, address))
}

fn not_found() -> Response<Cursor<Vec<u8>>> {
    Response::from_string("Not Found").with_status_code(404)
}

fn method_not_allowed() -> Response<Cursor<Vec<u8>>> {
    Response::from_string("Method Not Allowed").with_status_code(405)
}

fn livereload_js() -> Response<Cursor<Vec<u8>>> {
    let mut res = Response::from_string(LIVE_RELOAD).with_status_code(200);
    res.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/javascript"[..]).unwrap());
    res
}

fn io_error(err: std::io::Error) -> Response<Cursor<Vec<u8>>> {
    match err.kind() {
        std::io::ErrorKind::NotFound => not_found(),
        std::io::ErrorKind::PermissionDenied => {
            Response::from_string("Forbidden").with_status_code(403)
        }
        _ => panic!("{}", err),
    }
}

fn in_memory_content(path: &RelativePathBuf, content: &str) -> Response<Cursor<Vec<u8>>> {
    let content_type = match path.extension() {
        Some(ext) => match ext {
            "xml" => "text/xml",
            "json" => "application/json",
            _ => "text/html",
        },
        None => "text/html",
    };
    let mut res = Response::from_string(content).with_status_code(200);
    res.add_header(Header::from_bytes(&b"Content-Type"[..], content_type).unwrap());
    res
}

fn serve_static_file(req: &Request, mut root: PathBuf) -> Result<Response<Cursor<Vec<u8>>>> {
    // map uri path to file path
    let original_root = root.clone();
    let mut path = RelativePathBuf::new();

    debug!("{} {}", req.method().as_str(), req.url());

    let decoded = match percent_encoding::percent_decode_str(req.url()).decode_utf8() {
        Ok(d) => d,
        Err(_) => return Ok(not_found()),
    };
    for c in decoded.split('/') {
        path.push(c);
    }

    // livereload.js is served using the LIVE_RELOAD str, not a file
    if path == "livereload.js" {
        match req.method() {
            Method::Get => return Ok(livereload_js()),
            _ => return Ok(method_not_allowed()),
        }
    }

    if let Some(content) = SITE_CONTENT.read().unwrap().get(&path) {
        return Ok(in_memory_content(&path, content));
    }
    // Remove the first slash from the request path
    root.push(&decoded[1..]);

    // Ensure we are only looking for things in our public folder
    if !root.starts_with(original_root) {
        return Ok(not_found());
    }

    let metadata = match std::fs::metadata(root.as_path()) {
        Err(err) => return Ok(io_error(err)),
        Ok(metadata) => metadata,
    };
    if metadata.is_dir() {
        // if root is a directory, append index.html to try to read that instead
        root.push("index.html");
    };

    let result = std::fs::read(&root);

    let contents = match result {
        Err(err) => return Ok(io_error(err)),
        Ok(contents) => contents,
    };

    let mut res = Response::from_data(contents).with_status_code(200);
    let mime_type = from_path(&root).first_or_octet_stream();
    res.add_header(
        Header::from_bytes(
            &b"Content-Type"[..],
            mime_type.to_string().bytes().collect::<Vec<u8>>(),
        )
        .unwrap(),
    );
    Ok(res)
}

pub fn start_server(receiver: Receiver<i32>, app: &mut tauri::App) {
    let addr: SocketAddr = ([127, 0, 0, 1], 7878).into();

    let server = Arc::new(Server::http(addr).unwrap());

    // Spawn a thread to listen for the shutdown signal, and unblock ( shutdown ) the http server.
    let server_ = server.clone();
    std::thread::spawn(move || {
        let server = server_;

        loop {
            match receiver.recv() {
                Ok(c) if c == 1 => {
                    server.unblock();
                    break;
                }
                Err(e) => {
                    error!("Disconnected: {e}");
                    server.unblock();
                    break;
                }
                _ => (),
            }
        }
    });

    let cache_path = app.path_resolver().app_cache_dir().unwrap().join("dist/");
    fs::create_dir_all(&cache_path).expect("fixme");
    create_new_site(
        &app.path_resolver()
            .app_local_data_dir()
            .unwrap()
            .join("template/"),
        7878,
        &cache_path,
        "/",
        &app.path_resolver()
            .app_local_data_dir()
            .unwrap()
            .join("template/config.toml"),
        Some(7879),
    ).expect("could not build zola site");

    let cache = cache_path.to_str().unwrap().to_owned();

    // Spawn a thread to run the http server.
    thread::spawn(move || loop {
        match server.recv() {
            Ok(rq) => {
                match serve_static_file(&rq, PathBuf::from(&cache)) {
                    Err(e) => {
                        error!("Request failed: {e}");
                    }
                    Ok(res) => {
                        if let Err(e) = rq.respond(res) {
                            error!("Sending response failed: {e}");
                        };
                    }
                };
            }
            Err(e) => error!("{e}"),
        }
    });
}
