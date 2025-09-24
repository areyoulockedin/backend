use axum::{Router, extract::Request, routing::get};
use dotenvy::dotenv;
use rustls::crypto::ring::default_provider as tls_ring_provider;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    CompressionLevel,
    compression::{
        CompressionLayer,
        predicate::{NotForContentType, Predicate, SizeAbove},
    },
    decompression::RequestDecompressionLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::info;
use tracing_subscriber::{
    field::MakeExt,
    fmt::{Subscriber, format::debug_fn},
};

// TODO: Implement functionality.

use areyoulocked_in_backend::IoResult;

mod areyoulocked_in_backend;

fn get_log_level() -> tracing::Level {
    dotenv().ok();

    match std::env::var("LOG_LEVEL") {
        Ok(level) => level.parse().unwrap(),
        Err(_) => tracing::Level::INFO,
    }
}

#[tokio::main]
async fn main() -> IoResult<()> {
    tls_ring_provider()
        .install_default()
        .expect("Failed to install TLS provider");

    let formatter =
        debug_fn(|writer, field, value| write!(writer, "{}: {:?}", field, value)).delimited(",");

    Subscriber::builder()
        .with_max_level(get_log_level())
        .fmt_fields(formatter)
        .with_ansi(true)
        .init();

    let compression_predicate = SizeAbove::new(256).and(NotForContentType::IMAGES);

    let app = Router::new()
        .route("/", get(|| async { "Hi, World!" }))
        .fallback(get(|_req: Request| async move {
            let route = _req.uri().path().to_string();
            format!(
                "Ooo, a curious critter!, Feel free to explore other places than {route}, though, it wont be any different from what you're seeing here.",
            )
        }))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http().make_span_with(
                        DefaultMakeSpan::new()
                            .level(tracing::Level::INFO)
                            .include_headers(false),
                    ),
                )
                .layer(RequestDecompressionLayer::new())
                .layer(
                    CompressionLayer::new()
                        .no_br()
                        .no_deflate()
                        .gzip(true)
                        .zstd(true)
                        .quality(CompressionLevel::Fastest)
                        .compress_when(compression_predicate),
                ),
        );

    // TODO: Configurable bind address.

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Listening on http://{}", addr);

    // TODO: Add TLS bind.

    axum_server::bind(addr).serve(app.into_make_service()).await
}
