use crate::handler::*;
use crate::middleware::session::require_session;
use crate::common::*;
use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use database::init_pool;
use std::sync::Arc;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::add_extension::AddExtensionLayer;

pub async fn app() -> Router {
    let db_pool = init_pool().await;

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    Router::new()
        .route("/", get(public::public_view_handler))
        // /-> session authentication
        // user creation & login management
        .nest(
            "/api/session",
            Router::new()
                .route("/register", post(session::register::register_handler))
                .route("/login", post(session::login::login_handler)),
        )
        // session protected routes
        .nest(
            "/session", 
            Router::new()
                .route("/user/:user_id", get(session::getuser::get_profile_handler))
                .route("/logout", post(session::logout::logout_handler))
                .layer(axum_middleware::from_fn(require_session))
        )
        .layer(AddExtensionLayer::new(db_pool))
        .layer(GovernorLayer {
            config: governor_conf,
        })
}