use crate::models::RouteInfo;

use hyper::Client;
use warp::http::Response as HttpResponse;
use warp::Reply;

pub async fn proxy_request(
    target: Option<RouteInfo>,
    path: String,
    headers: warp::http::HeaderMap,
) -> Result<impl Reply, warp::Rejection> {
    if let Some(target) = target {
        let client = Client::new();

        let target_uri: String = target.internal_endpoint.clone() + &path;
        let res = client.get(target_uri.parse().unwrap()).await.unwrap();

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
