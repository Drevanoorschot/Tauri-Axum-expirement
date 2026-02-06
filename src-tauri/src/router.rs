use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use askama::Template;
use axum::{
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

const PERSISTENCE_FILE: &str = "/tmp/persisted_data.txt";

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/persist", post(persist))
        .route("/fetch", get(fetch))
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    gen_ts: String,
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        Html(self.render().unwrap()).into_response()
    }
}

async fn index() -> IndexTemplate {
    IndexTemplate {
        gen_ts: Utc::now().to_rfc3339(),
    }
}

#[derive(Deserialize)]
struct PersistRequest {
    number: u32,
}

#[derive(Serialize)]
struct PersistResponse {
    persisted_number: u32,
}

async fn persist(Json(payload): Json<PersistRequest>) -> Json<PersistResponse> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(PERSISTENCE_FILE)
        .unwrap();

    file.write_all(format!("{0}\n", payload.number).as_bytes())
        .unwrap();

    Json(PersistResponse {
        persisted_number: payload.number,
    })
}

#[derive(Serialize)]
struct FetchResponse {
    numbers: Vec<u32>,
}

async fn fetch() -> Json<FetchResponse> {
    let numbers = fs::read_to_string(PERSISTENCE_FILE)
        .unwrap()
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    Json(FetchResponse { numbers })
}
