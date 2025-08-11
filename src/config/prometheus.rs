use lazy_static::lazy_static;
use prometheus::{register_histogram_vec, register_int_counter_vec, HistogramVec, IntCounterVec};

lazy_static! {
    // HTTP request metrics
    pub static ref HTTP_REQUEST_COUNTER: IntCounterVec = register_int_counter_vec!(
        "http_requests_total",
        "Total number of HTTP requests by endpoint and status code",
        &["endpoint", "status"]
    )
    .unwrap();

    pub static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds for all endpoints",
        &["endpoint"]
    )
    .unwrap();

    // Database metrics
    pub static ref DB_QUERY_COUNTER: IntCounterVec = register_int_counter_vec!(
        "db_queries_total",
        "Total number of database queries by table and operation",
        &["table", "operation"]
    )
    .unwrap();

    pub static ref DB_QUERY_DURATION: HistogramVec = register_histogram_vec!(
        "db_query_duration_seconds",
        "Database query duration in seconds",
        &["table", "operation"]
    )
    .unwrap();
}
