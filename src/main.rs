extern crate actix_web;

use actix_web::{App, HttpResponse, Path, server};

fn init_json() -> Vec<String> {
    let paths = std::fs::read_dir("./data").unwrap();
    let mut collection: Vec<String> = Vec::new();
    for path in paths {
        let full_path = path.unwrap().path();

        let contents = std::fs::read_to_string(full_path).unwrap();
        collection.push(contents);
    }
    collection
}

fn get_port() -> String {
    let mut port: String = String::from("");
    for (key, value) in std::env::vars() {
        if key == "PORT" {
            port = value;
        }
    }

    return if port.is_empty() {
        String::from("8080")
    } else {
        port
    }
}

fn init() {
    println!("start");

    let port = get_port();
    let addr = format!("127.0.0.1:{}", port);
    println!("Binding at {}", addr);

    server::new(|| {
        App::new()
            .resource("/", |r| r.get().f( |_|HttpResponse::Ok().body("<h1>test</h1>")))
            .resource("/{name}", |r| {
                let items = init_json();
                r.get().with(move |path: Path<usize>| {
                    let value: usize = *path;
                    let value = &items[value];
                    value.clone()
                }
            )})
    })
        .bind(addr)
        .expect("Can not bind to port 8000")
        .run();
}

fn main() {
    init();
}