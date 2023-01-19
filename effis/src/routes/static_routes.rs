use std::{io::ErrorKind, path::Path};

use crate::{
    rate_limit::{RateLimitedRouteResponse, RateLimiter},
    Cache,
};
use rocket::{
    http::{ContentType, Header},
    State,
};
use rocket_db_pools::Connection;
use todel::{
    http::ClientIP,
    models::{
        ErrorResponse, ErrorResponseData, FetchResponse, NotFoundError, ServerError,
        ValidationError,
    },
    Conf,
};
use tokio::fs::File;

#[get("/<name>", rank = 1)]
pub async fn fetch_static_file<'a>(
    name: &'a str,
    ip: ClientIP,
    mut cache: Connection<Cache>,
    conf: &State<Conf>,
) -> RateLimitedRouteResponse<FetchResponse<'a>> {
    let mut rate_limiter = RateLimiter::new("fetch_file", "static", ip, conf.inner());
    rate_limiter.process_rate_limit(0, &mut cache).await?;
    let path = Path::new(name).file_name().map(Path::new).ok_or_else(|| {
        rate_limiter
            .wrap_response::<_, ()>(
                ValidationError {
                    field_name: "name".to_string(),
                    error: "Could not find a valid file name".to_string(),
                }
                .to_error_response(),
            )
            .unwrap()
    })?;
    let extension = path.extension();
    let content_type = match extension {
        Some(extension) => ContentType::from_extension(extension.to_str().ok_or_else(|| {
            rate_limiter
                .wrap_response::<_, ()>(
                    ValidationError {
                        field_name: "name".to_string(),
                        error: "Invalid file extension".to_string(),
                    }
                    .to_error_response(),
                )
                .unwrap()
        })?),
        None => None,
    };
    let file = File::open(Path::new("./files/static").join(path))
        .await
        .map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                rate_limiter
                    .wrap_response::<_, ()>(NotFoundError.to_error_response())
                    .unwrap()
            } else {
                rate_limiter
                    .wrap_response::<_, ()>(
                        ServerError {
                            error: "Failed to upload file".to_string(),
                        }
                        .to_error_response(),
                    )
                    .unwrap()
            }
        })?;
    log::info!("Fetched static file {}", name);
    rate_limiter.wrap_response(FetchResponse {
        file,
        disposition: Header::new(
            "Content-Disposition",
            format!(
                "inline; filename=\"{}\"",
                path.file_name().unwrap().to_str().unwrap()
            ),
        ),
        content_type: content_type.unwrap_or(ContentType::Any),
    })
}

#[get("/<name>/download", rank = 1)]
pub async fn download_static_file<'a>(
    name: &'a str,
    ip: ClientIP,
    mut cache: Connection<Cache>,
    conf: &State<Conf>,
) -> RateLimitedRouteResponse<Result<FetchResponse<'a>, ErrorResponse>> {
    let mut rate_limiter = RateLimiter::new("fetch_file", "static", ip, conf.inner());
    rate_limiter.process_rate_limit(0, &mut cache).await?;
    let path = Path::new(name).file_name().map(Path::new).ok_or_else(|| {
        rate_limiter
            .wrap_response::<_, ()>(
                ValidationError {
                    field_name: "name".to_string(),
                    error: "Could not find a valid file name".to_string(),
                }
                .to_error_response(),
            )
            .unwrap()
    })?;
    let extension = path.extension();
    let content_type = match extension {
        Some(extension) => ContentType::from_extension(extension.to_str().ok_or_else(|| {
            rate_limiter
                .wrap_response::<_, ()>(
                    ValidationError {
                        field_name: "name".to_string(),
                        error: "Invalid file extension".to_string(),
                    }
                    .to_error_response(),
                )
                .unwrap()
        })?),
        None => None,
    };
    let file = File::open(Path::new("./files/static").join(path))
        .await
        .map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                rate_limiter
                    .wrap_response::<_, ()>(NotFoundError.to_error_response())
                    .unwrap()
            } else {
                rate_limiter
                    .wrap_response::<_, ()>(
                        ServerError {
                            error: "Failed to upload file".to_string(),
                        }
                        .to_error_response(),
                    )
                    .unwrap()
            }
        })?;
    log::info!("Fetched static file {}", name);
    rate_limiter.wrap_response(Ok(FetchResponse {
        file,
        disposition: Header::new(
            "Content-Disposition",
            format!(
                "attachment; filename=\"{}\"",
                path.file_name().unwrap().to_str().unwrap()
            ),
        ),
        content_type: content_type.unwrap_or(ContentType::Any),
    }))
}
