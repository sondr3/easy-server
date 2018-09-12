extern crate colored;
extern crate rouille;

use colored::*;

fn main() {
    println!("Listening on {}", "localhost:8118".to_string().blue());

    rouille::start_server("localhost:8118", move |request| {
        {
            let response = rouille::match_assets(&request, ".");

            if response.is_success() {
                return response;
            }
        }

        rouille::Response::html("404!").with_status_code(404)
    });
}
