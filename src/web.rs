use std::net::SocketAddr;
use std::sync::{Arc, LazyLock};

use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

use crate::bot::State;
use crate::model::Experience;

static SHARED_STATE: LazyLock<Arc<Mutex<State>>> =
    LazyLock::new(|| Arc::new(Mutex::new(State::default())));

pub async fn create_web_server(state: State) {
    update_state(state).await;

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/api/xp", get(experience_handler))
        .route("/", get(bar_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn update_state(state: State) {
    *SHARED_STATE.lock().await = state;
}

async fn experience_handler() -> Json<Experience> {
    Json(*SHARED_STATE.lock().await.experience.lock().await)
}

async fn bar_handler() -> Html<&'static str> {
    Html(include_str!("bar.html"))
}
