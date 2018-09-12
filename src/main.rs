extern crate colored;
extern crate rouille;
extern crate notify;

use colored::*;
use rouille::Response;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn files() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher.watch(".", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    println!("Listening on {}", "localhost:8118".to_string().blue());

    rouille::start_server("localhost:8118", move |request| {
        {
            let response = rouille::match_assets(&request, ".");

            files();

            if response.is_success() {
                return response;
            }
        }

        Response::html("404!").with_status_code(404)
    });
}
