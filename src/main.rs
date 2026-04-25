use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::routes::{AppRouter, todos::TodosRouter};

mod api_doc;
mod models;
mod routes;
mod services;
mod wrappers;

fn build_app() -> Router {
    Router::new()
        .nest("/todos", TodosRouter::build())
        .merge(Scalar::with_url("/scalar", api_doc::ApiDoc::openapi()))
}

#[tokio::main]
async fn main() {
    let app = build_app();
    let addr = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
