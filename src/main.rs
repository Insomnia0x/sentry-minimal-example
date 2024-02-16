use sentry_tracing::EventFilter;
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
        .with(
            sentry_tracing::layer()
                .event_filter(|md| match *md.level() {
                    tracing::Level::ERROR => EventFilter::Exception,
                    tracing::Level::INFO => EventFilter::Event,
                    tracing::Level::DEBUG => EventFilter::Breadcrumb,
                    _ => EventFilter::Ignore,
                })
                .enable_span_attributes(),
        )
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // creates an issue with breadcrumbs.
            creates_issue_example().await;
        });
}

/// Example taken from the docs: https://github.com/getsentry/sentry-rust/tree/master/sentry-tracing#tracing-spans
#[tracing::instrument]
async fn creates_issue_example() {
    for i in 0..10 {
        creates_issue_example_inner(i).await;
    }

    // this creates the issue.
    tracing::info!("completed");
}

// This creates spans inside the outer transaction, unless called directly.
#[tracing::instrument]
async fn creates_issue_example_inner(i: u32) {
    // Also works, since log events are ingested by the tracing system
    tracing::debug!(number = i, "Generates a breadcrumb");

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}
