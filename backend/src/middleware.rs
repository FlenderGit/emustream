use axum::{extract::Request, middleware::Next, response::Response};
use uuid::Uuid;

tokio::task_local! {
    pub(crate) static TRACE_ID: Uuid;
}

pub async fn middleware_request_time(req: Request, next: Next) -> Response {
    let now = std::time::Instant::now();
    let uuid = Uuid::new_v4();

    let mut response = TRACE_ID.scope(uuid, async {
        next.run(req).await
    }).await;

    response.headers_mut().insert(
        "X-Request-Time",
        format!("{}ms", now.elapsed().as_millis()).parse().unwrap(),
    );

    response.headers_mut().insert(
        "X-Request-ID",
        uuid.to_string().parse().unwrap(),
    );

    response
}
