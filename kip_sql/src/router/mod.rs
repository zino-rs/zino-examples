use crate::{
    controller::{bench, user},
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Vec<Router> {
    let mut routes = Vec::new();

    // User controller.
    let router = Router::new()
        .route("/user/new", post(user::new))
        .route("/user/:id/view", get(user::view));
    routes.push(router);

    // Bench controller.
    let router = Router::new().route("/bench/kip_sql/user/:id/view", get(bench::kip_sql_user_view));
    routes.push(router);

    routes
}
