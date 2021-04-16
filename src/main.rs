use std::sync::Arc;

use warp::Filter;
mod trie;
use serde::{Deserialize, Serialize};
use trie::Umbrella;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let maybe_file_name = std::env::args().nth(1);
    let umbrella = if let Some(file_name) = maybe_file_name {
        Umbrella::seed(&file_name)
    } else {
        Umbrella::new()
    };
    let umbrella = Arc::new(umbrella);
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

let cors = warp::cors()
    .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Content-Type"])

    .allow_origin("http://localhost:3000")
    .allow_methods(vec!["GET", "POST", "DELETE"]);

    warp::serve(insert.or(suggest.with(&cors).with(cors)))
        .run(([127, 0, 0, 1], 3031))
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
