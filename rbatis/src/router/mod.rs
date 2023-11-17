use crate::{
    controller::{bench, user},
    model::{Tag, User},
};
use axum::{
    routing::{get, post},
    Router,
};
use zino::DefaultController;

pub fn routes() -> Vec<Router> {
    let mut routes = Vec::new();

    // User controller.
    let router = Router::new()
        .route("/user/new", post(user::new))
        .route("/user/:id/delete", post(User::delete))
        .route("/user/:id/update", post(User::update))
        .route("/user/:id/view", get(user::view))
        .route("/user/list", get(User::list));
    routes.push(router);

    // Tag controller.
    let router = Router::new()
        .route("/tag/new", post(Tag::new))
        .route("/tag/:id/delete", post(Tag::delete))
        .route("/tag/:id/update", post(Tag::update))
        .route("/tag/:id/view", get(Tag::view))
        .route("/tag/list", get(Tag::list));
    routes.push(router);

    // Bench controller.
    let router = Router::new().route("/bench/rbatis/user/:id/view", get(bench::rbatis_user_view));
    routes.push(router);

    routes
}
