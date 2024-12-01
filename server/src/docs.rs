use axum::Router;

pub fn router() -> Router {
    #[cfg(debug_assertions)]
    {
        use axum::{response::Redirect, routing::get};
        Router::new().route(
            "/",
            get(|| async { Redirect::permanent("http://localhost:5000") }),
        )
    }
    #[cfg(not(debug_assertions))]
    {
        use tower_http::services::{ServeDir, ServeFile};
        Router::new().nest_service(
            "/",
            ServeDir::new("/docs").not_found_service(ServeFile::new("/docs/index.html")),
        )
    }
}
