use crate::routes::AppRouter;
use axum::{Router, routing::get};

pub struct TodosRouter;

pub mod docs;
mod handler;

impl AppRouter for TodosRouter {
    fn build() -> axum::Router {
        Router::new().route("/", get(handler::get_todos))
    }
}
