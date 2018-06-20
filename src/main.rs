extern crate rayon;

#[macro_use]
extern crate serde_json;

use rayon::prelude::*;
use std::collections::HashMap;

const LIMIT: u16 = 1307;

mod network {
    extern crate reqwest;

    pub fn request(page_id: u16) -> String {
        let url = if page_id == 1 {
            String::from("http://23.95.221.108/")
        } else {
            format!("http://23.95.221.108/page/{}/", page_id)
        };
        let resp = reqwest::get(&url);
        return if let Err(_e) = resp {
            println!("Failed {}", url);
            request(page_id)
        } else {
            resp.unwrap().text().unwrap()
        }
    }

    pub fn request_path(path: &str) -> String {
        let url = format!("http://23.95.221.108{}", path);
        let resp = reqwest::get(&url);
        return if let Err(_e) = resp {
            println!("Failed {}", url);
            request_path(path)
        } else {
            resp.unwrap().text().unwrap()
        }
    }
}

mod dom {
    extern crate scraper;

    pub fn get_single_text<'a>(html: &'a scraper::Html, select: &str) -> &'a str {
        let selector = scraper::Selector::parse(select).unwrap();
        let text = html.select(&selector).next().unwrap().text().collect::<Vec<_>>();
        return if text.is_empty() {
            ""
        } else {
            text[0]
        }
    }

    pub fn get_single<'a>(html: &'a scraper::Html, select: &str, att: &str) -> &'a str {
        let selector = scraper::Selector::parse(select).unwrap();
        &html.select(&selector).next().unwrap().value().attr(att).unwrap()
    }

    pub fn get_html_from_id(page_id: u16) -> scraper::Html {
        let html = super::network::request(page_id);
        scraper::Html::parse_document(&html)
    }

    pub fn get_html_from_path(path: &str) -> scraper::Html {
        let html = super::network::request_path(path);
        scraper::Html::parse_document(&html)
    }

    pub fn selector(html: &str) -> scraper::Selector {
        scraper::Selector::parse(html).unwrap()
    }
}

fn main() {
    println!("start");

    let collection: Vec<serde_json::Value> =
        (1..LIMIT).into_par_iter().flat_map(|page_id: u16| {
            let selector = dom::selector("h2.post-title a[href]");
            let html = dom::get_html_from_id(page_id);
            let a_href = {
                html.select(&selector)
            };

            let mut list: Vec<serde_json::Value> = Vec::new();
            for element in a_href {
                let url = {
                    let url = element.value().attr("href").unwrap();
                    url.trim_left_matches("https://it-eb.com")
                };
                let full_path = format!("./data{}.json", url.trim_right_matches("/"));

                if std::fs::metadata(&full_path).is_ok() {
                    let contents = std::fs::read_to_string(full_path).unwrap();

                    list.push(serde_json::from_str(&contents).unwrap());
                } else {
                    println!("Downloading {}", url);

                    let html = dom::get_html_from_path(url);

                    let title = dom::get_single_text(&html, "h1.post-title");
                    let img = {
                        let img = dom::get_single(&html, "div.book-cover img[src]", "src");
                        let img = img.trim_left_matches("https://it-eb.com");
                        format!("http://23.95.221.108{}", img)
                    };
                    let link = {
                        let id = dom::get_single(&html, "input[name=\"comment_post_ID\"]", "value");
                        let url = format!("/download.php?id={}", id);
                        network::request_path(&url)
                    };

                    let desc = dom::get_single_text(&html, "div.entry-inner");

                    let cats = {
                        let select = dom::selector("p.post-btm-cats a[href]");
                        let categories = html.select(&select);
                        let mut cats: Vec<&str> = Vec::new();
                        for c in categories {
                            let t = c.text().collect::<Vec<_>>()[0];
                            cats.push(t);
                        }
                        cats
                    };

                    let details = {
                        let key_text = {
                            let select = dom::selector("div.book-details li span");
                            let keys = html.select(&select);
                            let mut vec_keys: Vec<&str> = Vec::new();
                            for k in keys {
                                let t = k.text().collect::<Vec<_>>()[0];
                                vec_keys.push(t);
                            }

                            vec_keys
                        };

                        let val_text = {
                            let select = dom::selector("div.book-details li");
                            let val = html.select(&select);
                            let mut vec_val: Vec<&str> = Vec::new();
                            for k in val {
                                let t = k.text().collect::<Vec<_>>()[1];
                                vec_val.push(t);
                            }

                            vec_val
                        };

                        let len = val_text.len();
                        let mut vec = HashMap::new();
                        for i in 0..len {
                            let key = {
                                let key = key_text[i];
                                let key = key.trim_right();
                                let key = key.trim_right_matches(":");
                                key.to_lowercase()
                            };
                            let val = val_text[i];
                            vec.insert(key, val);
                        }

                        json!({
                            "isbn-10": vec.get("isbn-10"),
                            "isbn-13": vec.get("isbn-13"),
                            "format": vec.get("format"),
                            "authors": vec.get("authors"),
                            "publication date": vec.get("publication date"),
                            "publisher": vec.get("publisher"),
                            "pages": vec.get("pages"),
                            "size": vec.get("size")
                        })
                    };

                    let value = json!({
                      "title": title,
                      "img": img,
                      "link": link,
                      "desc": desc,
                      "categories": cats,
                      "details": details
                    });
                    std::fs::write(full_path, value.to_string()).expect("Could not write");
                    list.push(value);
                }
            }

            list
        }).collect();

    println!("done {}", collection.len());
}