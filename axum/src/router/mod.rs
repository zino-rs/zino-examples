use crate::model::{Tag, User};
use axum::{
    routing::{get, post},
    Router,
};
use zino::DefaultController;

pub fn routes() -> Vec<Router> {
    let mut routes = Vec::new();

    // User controller.
    let router = Router::new()
        .route("/user/new", post(User::new))
        .route("/user/:id/delete", post(User::soft_delete))
        .route("/user/:id/update", post(User::update))
        .route("/user/:id/view", get(User::view))
        .route("/user/list", get(User::list))
        .route("/user/import", post(User::import))
        .route("/user/export", get(User::export))
        .route("/user/schema", get(User::schema))
        .route("/user/definition", get(User::definition));
    routes.push(router);

    // Tag controller.
    let router = Router::new()
        .route("/tag/new", post(Tag::new))
        .route("/tag/:id/delete", post(Tag::soft_delete))
        .route("/tag/:id/update", post(Tag::update))
        .route("/tag/:id/view", get(Tag::view))
        .route("/tag/list", get(Tag::list))
        .route("/tag/tree", get(Tag::tree))
        .route("/tag/schema", get(Tag::schema))
        .route("/tag/definition", get(Tag::definition));
    routes.push(router);

    routes
}
