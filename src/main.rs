extern crate actix_web;

use actix_web::{App, HttpResponse, Path, server};

const PAGE_LIMIT: usize = 50;

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

fn count_guard(count: usize) -> usize {
    match count {
        0 => 1,
        c if path.1 > PAGE_LIMIT => PAGE_LIMIT,
        _ => count
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
            .resource("/{base_idx}/{count}", |r| {
                let items = init_json();
                r.get().with(move |path: Path<(usize, usize)>| {
                    let base_idx: usize = path.0;
                    let count: usize = count_guard(page.1);

                    let end_idx: usize = base_idx + count;

                    let slicy = &items[base_idx..end_idx];

                    let value = slicy.join(", ");
                    let value = format!("[ {} ]", value);
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