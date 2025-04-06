use axum::{extract::Request, middleware::Next, response::Response};

pub async fn middleware_request_time(req: Request, next: Next) -> Response {
    let now = std::time::Instant::now();

    let mut response = next.run(req).await;

    response.headers_mut().insert(
        "X-Request-Time",
        format!("{}ms", now.elapsed().as_millis()).parse().unwrap(),
    );

    response
}
