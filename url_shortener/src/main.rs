mod storage;

use serde::Deserialize;
use std::fs;
use storage::Storage;

use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post};

#[tokio::main]
async fn main() {
    let storage = Storage::new();
    let state = Arc::new(Mutex::new(storage));

    let app = Router::new()
        .route("/{id}", get(get_handler))
        .with_state(Arc::clone(&state))
        .route("/", post(post_handler))
        .with_state(Arc::clone(&state))
        .route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct CreateUrl {
    url: String,
}

async fn post_handler(
    State(state): State<Arc<Mutex<Storage>>>,
    Form(data): Form<CreateUrl>,
) -> Html<String> {
    let id = state.lock().unwrap().save(data.url.clone());
    println!("{data:?}");

    Html::from(format!("Your url shortened is: localhost:3000/{id}"))
}

async fn get_handler(
    Path(id): Path<usize>,
    State(state): State<Arc<Mutex<Storage>>>,
) -> impl IntoResponse {
    let state = state.lock().unwrap();
    let item = state.get(id);

    if let Some(url) = item {
        Redirect::to(&url).into_response()
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}

async fn index() -> Html<String> {
    let content = fs::read_to_string("templates/index.html").unwrap();
    Html::from(content)
}
