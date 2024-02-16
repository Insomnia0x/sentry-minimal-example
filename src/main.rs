use tracing_subscriber::prelude::*;

fn main() {
    dotenvy::dotenv().ok();
    let dsn = std::env::var("SENTRY_DSN").unwrap();

    let _guard = sentry::init((
        dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_tracing::layer().enable_span_attributes())
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // these do not create breadcrumbs in the transaction.
            tracing_spans_example().await;
            tracing_spans_example_with_manual_breadcrumb().await;
        });
}

/// Example taken from the docs: https://github.com/getsentry/sentry-rust/tree/master/sentry-tracing#tracing-spans
#[tracing::instrument]
async fn tracing_spans_example() {
    for i in 0..10 {
        tracing_spans_example_inner(i).await;
    }
}

// This creates spans inside the outer transaction, unless called directly.
#[tracing::instrument]
async fn tracing_spans_example_inner(i: u32) {
    // Also works, since log events are ingested by the tracing system
    tracing::debug!(number = i, "Generates a breadcrumb");

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

/// Example taken from docs: https://docs.sentry.io/platforms/rust/enriching-events/breadcrumbs/
#[tracing::instrument]
async fn tracing_spans_example_with_manual_breadcrumb() {
    for i in 0..10 {
        tracing_spans_example_inner_with_manual_breadcrumb(i).await;
    }
}

// This creates spans inside the outer transaction, unless called directly.
#[tracing::instrument]
async fn tracing_spans_example_inner_with_manual_breadcrumb(i: u32) {
    use sentry::{add_breadcrumb, Breadcrumb, Level};

    add_breadcrumb(Breadcrumb {
        category: Some("category".into()),
        message: Some(format!("number {}", i)),
        level: Level::Info,
        ..Default::default()
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}
