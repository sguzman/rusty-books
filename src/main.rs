fn main() {
    println!("start");
    let paths = std::fs::read_dir("./data").unwrap();

    let coll = {
        let mut collection: Vec<String> = Vec::new();
        for path in paths {
            let full_path = path.unwrap().path();
            println!("Found {}", full_path.display());

            let contents = std::fs::read_to_string(full_path).unwrap();
            collection.push(contents);
        }
        collection
    };

    println!("done {}", coll.len());
}