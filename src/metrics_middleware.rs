use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use lazy_static::lazy_static;
use prometheus::{register_histogram_vec, register_int_counter_vec, HistogramVec, IntCounterVec};
use std::future::{ready, Ready};
use std::time::Instant;

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
}

pub struct PrometheusMetrics;

impl<S, B> Transform<S, ServiceRequest> for PrometheusMetrics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PrometheusMetricsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PrometheusMetricsMiddleware { service }))
    }
}

pub struct PrometheusMetricsMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for PrometheusMetricsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let path = req.path().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let elapsed = start.elapsed().as_secs_f64();
            let status = res.status().as_u16().to_string();

            // Record request count by endpoint and status
            HTTP_REQUEST_COUNTER
                .with_label_values(&[&path, &status])
                .inc();

            // Record request duration by endpoint
            HTTP_REQUEST_DURATION
                .with_label_values(&[&path])
                .observe(elapsed);

            Ok(res)
        })
    }
}
