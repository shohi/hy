use actix_web::{http, server, App, HttpRequest};
use std::cell::Cell;

// This struct represents state
struct AppState {
    counter: Cell<usize>,
}

fn index(req: &HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1; // <- get count
    req.state().counter.set(count); // <- store new count in state

    format!("Request number: {}", count) // <- response with count
}

fn new_server() {
    server::new(|| {
        App::with_state(AppState {
            counter: Cell::new(0),
        })
        .resource("/", |r| r.method(http::Method::GET).f(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}
