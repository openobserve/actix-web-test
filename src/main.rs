use axum::routing::{get, put};
use axum::Router;
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/:org_id/",
            get(api::org_es_index).head(api::org_es_index),
        )
        .route("/api/:org_id/_license", get(api::org_es_license))
        .route("/api/:org_id/_xpack", get(api::org_es_xpack))
        .route(
            "/api/:org_id/_index_template/:name",
            get(api::org_es_index_template).head(api::org_es_index_template),
        )
        .route(
            "/api/:org_id/_index_template/:name",
            put(api::org_es_index_template_create),
        )
        .route(
            "/api/:org_id/_data_stream/:name",
            get(api::org_es_data_stream).head(api::org_es_data_stream),
        )
        .route(
            "/api/:org_id/_data_stream/:name",
            put(api::org_es_data_stream_create),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 5080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
