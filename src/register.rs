// src/register.rs

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use warp::Filter;

use crate::models::RouteInfo;

pub fn register_route(
    routes_table: Arc<RwLock<HashMap<String, RouteInfo>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let register_post = warp::path("register")
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
        });

    let register_get = warp::path("register").and(warp::get()).map({
        let routes_table_clone = routes_table.clone();
        move || {
            let routes_read = routes_table_clone.read().unwrap();
            let route_list: Vec<RouteInfo> = routes_read.values().cloned().collect();
            warp::reply::json(&route_list)
        }
    });

    register_post.or(register_get)
}
