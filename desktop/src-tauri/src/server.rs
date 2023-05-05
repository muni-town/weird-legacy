use log::{debug, error};
use mime_guess::from_path;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::thread;
use std::{fs::File, sync::mpsc::Receiver, sync::Arc};
use tiny_http::{Header, Request, Response, Server};

use crate::prelude::*;

fn serve_static_file(req: Request, path: PathBuf) -> Result<()> {
    // map uri path to file path
    let mut path = path.join(req.url().splitn(2, '/').collect::<Vec<&str>>()[1]);

    // server the index.html file for directory
    if path.is_dir() {
        path.push("index.html");
    }
    debug!("serving file path: {}", path.to_str().unwrap());

    let res = match File::open(&path) {
        Ok(file) => Response::from_file(file),
        Err(_) => {
            // return 404 Not Found if the file doesn't exist
            req.respond(Response::from_string("Not Found").with_status_code(404))?;
            return Ok(());
        }
    };

    let mime_type = from_path(&path).first_or_octet_stream();
    let mut res = res.with_status_code(200);
    res.add_header(
        Header::from_bytes(
            &b"Content-Type"[..],
            mime_type.to_string().bytes().collect::<Vec<u8>>(),
        )
        .unwrap(),
    );
    req.respond(res)?;
    Ok(())
}

pub fn start_server(receiver: Receiver<i32>, app: &mut tauri::App) {
    let addr: SocketAddr = ([127, 0, 0, 1], 7878).into();
    let path = app
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("template/")
        .to_str()
        .unwrap()
        .to_owned();

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
                Err(_) => {
                    error!("Disconnected");
                    server.unblock();
                    break;
                }
                _ => (),
            }
        }
    });

    // Spawn a thread to run the http server.
    thread::spawn(move || loop {
        match server.recv() {
            Ok(rq) => {
                if let Err(e) = serve_static_file(rq, PathBuf::from(&path)) {
                    error!("Request failed: {e}");
                };
            }
            Err(e) => error!("{e}"),
        }
    });
}
