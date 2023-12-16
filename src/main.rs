mod views;

use views::layout::base;

use axum::{routing::get, Router};
use maud::{html, Markup};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn root() -> Markup {
    base(
        None,
        html! {
            h1 {
                "Welcome to Pathbase"
            }
        },
    )
}
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pathbase=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let assets = Router::new().nest_service("/assets", ServeDir::new("static"));
    let app = Router::new()
        // Serve the main page
        .route("/", get(root));

    let router = Router::new()
        .merge(app)
        .merge(assets)
        .layer(TraceLayer::new_for_http());

    // Run our sever listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
