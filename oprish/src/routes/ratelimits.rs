use rocket::{serde::json::Json, State};
use rocket_db_pools::Connection;
use todel::{http::ClientIP, models::InstanceRateLimits, Conf};

use crate::{
    ratelimit::{RateLimitedRouteResponse, RateLimiter},
    Cache,
};

#[get("/ratelimits")]
pub async fn ratelimits(
    address: ClientIP,
    mut cache: Connection<Cache>,
    conf: &State<Conf>,
) -> RateLimitedRouteResponse<Json<InstanceRateLimits>> {
    let conf = conf.inner();
    let mut ratelimiter = RateLimiter::new("ratelimits", address, conf);
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(Json(InstanceRateLimits {
        oprish: conf.oprish.ratelimits.clone(),
        pandemonium: conf.pandemonium.ratelimit.clone(),
        effis: conf.effis.ratelimits.clone(),
    }))
}
