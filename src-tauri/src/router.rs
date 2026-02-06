use axum::{Router, response::{Html, IntoResponse, Response}, routing::get};
use askama::Template;
use chrono::Utc;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    gen_ts: String
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        Html(self.render().unwrap()).into_response()
    }
}


async fn index() -> impl IntoResponse {
    IndexTemplate {
        gen_ts: Utc::now().to_rfc3339()
    }
}
