pub mod messages;
pub mod ratelimits;

use rocket::{serde::json::Json, Route, State};
use rocket_db_pools::Connection;
use todel::{http::ClientIP, models::InstanceInfo, Conf};

use crate::{
    ratelimit::{RateLimitedRouteResponse, RateLimiter},
    Cache, VERSION,
}; // poggers

#[get("/")]
pub async fn index(
    address: ClientIP,
    mut cache: Connection<Cache>,
    conf: &State<Conf>,
) -> RateLimitedRouteResponse<Json<InstanceInfo>> {
    let mut ratelimiter = RateLimiter::new("info", address, conf.inner());
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(Json(InstanceInfo {
        instance_name: conf.instance_name.clone(),
        version: VERSION,
        description: conf.description.clone(),
        message_limit: conf.oprish.message_limit,
        oprish_url: &conf.oprish.url,
        pandemonium_url: &conf.pandemonium.url,
        effis_url: &conf.effis.url,
        file_size: conf.effis.file_size,
        attachment_file_size: conf.effis.attachment_file_size,
    }))
}

pub fn get_routes() -> Vec<Route> {
    routes![index, ratelimits::ratelimits]
}
