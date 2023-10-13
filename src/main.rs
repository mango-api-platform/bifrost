mod models;
mod proxy;

use models::RouteInfo;
use proxy::proxy_request;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use warp::Filter;

fn build_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let routes_table = Arc::new(RwLock::new(HashMap::new()));

    // Admin endpoint to register a new service
    let register = {
        warp::path("register")
            .and(warp::post())
            .and(warp::body::json())
            .and_then({
                let routes_table_clone = routes_table.clone();
                move |new_route: RouteInfo| {
                    let routes_clone = routes_table_clone.clone();
                    async move {
                        let mut routes_write = routes_clone.write().unwrap();
                        let key = new_route.route.clone();
                        println!("register key: {:?} | value: {:?}", key, new_route);
                        routes_write.insert(key, new_route);
                        Ok::<_, warp::Rejection>("Registered")
                    }
                }
            })
    };

    // Dynamic routing
    let dynamic_routing = warp::any()
        .and(warp::header::headers_cloned())
        .and(warp::path::tail())
        .and(warp::method())
        .and(warp::body::bytes())
        .and_then({
            let routes_table_clone = routes_table.clone();
            move |headers: warp::http::HeaderMap,
                  tail: warp::path::Tail,
                  method: warp::http::Method,
                  body: bytes::Bytes| {
                let routes_clone = routes_table_clone.clone();
                let path = tail.as_str().to_string();
                let routes_read = routes_clone.read().unwrap();
                let route = path.split('/').next().unwrap().to_string();
                let remaining_path: String = path.replacen(&route, "", 1);
                let target = routes_read.get(&route).cloned();

                proxy_request(target, remaining_path, headers, method, body)
            }
        });

    let routes = register.or(dynamic_routing);
    routes
}

#[tokio::main]
async fn main() {
    // Start Server
    println!("Starting the server at 127.0.0.1:3000");
    warp::serve(build_routes())
        .run(([127, 0, 0, 1], 3000))
        .await;
}
