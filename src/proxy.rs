use crate::models::RouteInfo;

use hyper::Client;
use warp::http::Response as HttpResponse;
use warp::Reply;

pub async fn proxy_request(
    target: Option<RouteInfo>,
    path: String,
    headers: warp::http::HeaderMap,
    method: warp::http::Method,
    body: bytes::Bytes,
) -> Result<impl Reply, warp::Rejection> {
    if let Some(target) = target {
        let client = Client::new();

        let target_uri: String = target.internal_endpoint.clone() + &path;

        let mut request_builder = hyper::Request::builder().method(method).uri(target_uri);

        if let Some(headers_mut) = request_builder.headers_mut() {
            headers_mut.extend(headers);
        }

        let request = request_builder.body(hyper::Body::from(body)).unwrap();

        let res = client.request(request).await.unwrap();

        let mut response = HttpResponse::builder().status(res.status());
        for (key, value) in res.headers().iter() {
            response = response.header(key, value);
        }

        let body = hyper::body::to_bytes(res).await.unwrap();
        let response = response.body(body).unwrap();

        Ok(warp::reply::with_header(response, "X-Proxy", "Warp"))
    } else {
        let response = HttpResponse::builder()
            .status(warp::http::StatusCode::NOT_FOUND)
            .body(hyper::body::Bytes::from_static(b"Not Found"))
            .unwrap();
        Ok(warp::reply::with_header(response, "X-Proxy", "Warp"))
    }
}
