use axum::{
    Router,
    middleware::{self, Next},
    extract::Request,
    response::Response,
    routing::get,
};
use tower_http::trace::TraceLayer;
use tracing::{info, Span};
use uuid::Uuid;


//Next<B> -> Next - they abstracted generics in axum 0.8
//middleware functions work directly with Request and don't need generic body parameter
async fn correlation_id_middleware(mut req: Request, next: Next) -> Response {
    // Generate correlation ID
    let correlation_id = Uuid::new_v4().to_string();

    // Add to request extensions
    req.extensions_mut().insert(correlation_id.clone());

    // Attach to the tracing span
    Span::current().record("correlation_id", &tracing::field::display(&correlation_id));
    info!(%correlation_id, "📥 Incoming request");

    // Call next middleware/handler
    next.run(req).await

}

async fn hello_handler() -> &'static str {
    "👋 Hello from axum 0.8!"
}

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(hello_handler))
        .layer(middleware::from_fn(correlation_id_middleware))
        .layer(TraceLayer::new_for_http())
}
