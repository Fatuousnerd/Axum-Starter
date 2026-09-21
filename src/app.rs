use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::TraceLayer,
};
use uuid::Uuid;

use crate::routes;
use crate::state::AppState;

#[derive(Clone)]
struct RequestIdMaker;

impl MakeRequestId for RequestIdMaker {
    fn make_request_id<B>(&mut self, _: &axum::http::Request<B>) -> Option<RequestId> {
        Uuid::new_v4().to_string().parse().ok().map(RequestId::new)
    }
}

pub fn create_app(state: AppState) -> Router {
    let cfg = crate::config::get_config();

    let cors = CorsLayer::new()
        .allow_origin(
            if cfg.cors.allowed_origins == ["*"] {
                tower_http::cors::AllowOrigin::any()
            } else {
                tower_http::cors::AllowOrigin::list(
                    cfg.cors
                        .allowed_origins
                        .iter()
                        .map(|o| o.parse().unwrap())
                        .collect::<Vec<_>>(),
                )
            },
        )
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(routes::router())
        .layer(PropagateRequestIdLayer::new(
            axum::http::header::HeaderName::from_static("x-request-id"),
        ))
        .layer(SetRequestIdLayer::new(
            axum::http::header::HeaderName::from_static("x-request-id"),
            RequestIdMaker,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}
