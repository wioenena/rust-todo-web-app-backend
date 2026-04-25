use axum::Router;

pub trait AppRouter {
    fn build() -> Router;
}

pub mod todos;
