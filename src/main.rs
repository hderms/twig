use std::sync::{Arc, RwLock};

use warp::Filter;
mod trie;
use serde::{Deserialize, Serialize};
use trie::Node;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let root = Arc::new(RwLock::new(Node::new()));
    let insert = {
        let root = root.clone();

        warp::post()
            .and(warp::path("insert"))
            .and(warp::body::content_length_limit(1024))
            .and(warp::body::json())
            .map(move |insert_request: InsertRequest| {
                let root = root.clone();
                let mut root = root.write().unwrap();
                root.insert(insert_request.string.as_str());
                warp::reply::json(&Success::new())
            })
    };

    let full = {
        let root = root.clone();
        warp::get().and(warp::path("full")).map(move || {
            let root = root.clone();
            let root = root.read().unwrap();
            warp::reply::json(&&*root)
        })
    };

    let suggest = {
        let root = root.clone();
        warp::get()
            .and(warp::path("suggestions"))
            .and(warp::query::<SuggestRequest>())
            .map(move |suggest_request: SuggestRequest| {
                let root = root.clone();
                let suggestions = root
                    .read()
                    .unwrap()
                    .suggest(suggest_request.string.as_str(), suggest_request.limit)
                    .unwrap_or_default();
                let response = SuggestResponse::new(suggestions);
                warp::reply::json(&response)
            })
    };

    warp::serve(insert.or(full).or(suggest))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Deserialize, Serialize)]
struct Success {
    ok: bool,
}
impl Success {
    fn new() -> Success {
        Success { ok: true }
    }
}

#[derive(Deserialize, Serialize)]
struct InsertRequest {
    string: String,
}

#[derive(Deserialize, Serialize)]
struct SuggestRequest {
    string: String,
    limit: usize,
}

#[derive(Deserialize, Serialize)]
struct SuggestResponse {
    suggestions: Vec<String>,
}
impl SuggestResponse {
    fn new(suggestions: Vec<String>) -> SuggestResponse {
        SuggestResponse { suggestions }
    }
}
