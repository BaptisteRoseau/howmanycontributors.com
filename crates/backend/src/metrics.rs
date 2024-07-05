use tracing::debug;

// TODO: When splitting the codebase into multiple crates (cache/database),
// this should be moved to the crates with a feature flag and within the "init" function
// enableb by default, with an option to opt-out in the builder.

pub fn init() {
    debug!("Initializing metrics: cache_hit");
    metrics::describe_counter!(
        "cache_hit",
        metrics::Unit::Count,
        "Wether or not the cache was hit. Labels:
            - status: hit/miss
            - from: dependencies, contributors
        "
    );

    debug!("Initializing metrics: ws_sent");
    metrics::describe_counter!(
        "ws_sent",
        metrics::Unit::Count,
        "Count of WS messages sent."
    );

    debug!("Initializing metrics: errors");
    metrics::describe_counter!(
        "errors",
        metrics::Unit::Count,
        "Count of errors."
    );

    debug!("Initializing metrics: rate_limiter_size_mb");
    metrics::describe_gauge!(
        "rate_limiter_size_mb",
        metrics::Unit::Count,
        "The size in MB of the rate limiter."
    );
}
