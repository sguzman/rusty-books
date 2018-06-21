extern crate actix_web;

use actix_web::{App, Path, server};

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

fn init() {
    println!("start");

    server::new(|| {
        App::new()
            .resource("/{name}", |r| {
                let items = init_json();
                r.get().with(move |path: Path<usize>| {
                    let value: usize = *path;
                    let value = &items[value];
                    value.clone()
                }
            )})
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}

fn main() {
    init();
}