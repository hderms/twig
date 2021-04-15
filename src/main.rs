use std::sync::Arc;

use warp::Filter;
mod trie;
use serde::{Deserialize, Serialize};
use trie::Umbrella;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let umbrella = Arc::new(Umbrella::new());
    let insert = {
        let root = umbrella.clone();

        warp::post()
            .and(warp::path("insert"))
            .and(warp::body::content_length_limit(1024))
            .and(warp::body::json())
            .map(move |insert_request: InsertRequest| {
                let mut node = root.get(&insert_request.string).write().unwrap();
                node.insert(insert_request.string.as_str());
                warp::reply::json(&Success::new())
            })
    };

    let suggest = {
        let root = umbrella.clone();
        warp::get()
            .and(warp::path("suggestions"))
            .and(warp::query::<SuggestRequest>())
            .map(move |suggest_request: SuggestRequest| {
                let node = root.get(&suggest_request.string).read().unwrap();
                let suggestions = node
                    .suggest(suggest_request.string.as_str(), suggest_request.limit)
                    .unwrap_or_default();
                let response = SuggestResponse::new(suggestions);
                warp::reply::json(&response)
            })
    };

    warp::serve(insert.or(suggest))
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
