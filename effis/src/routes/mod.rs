mod buckets;
mod index;
mod static_routes;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        index::upload,
        index::fetch,
        index::fetch_download,
        index::fetch_data,
        buckets::upload,
        buckets::fetch,
        buckets::fetch_download,
        buckets::fetch_data,
    ]
}

pub fn static_routes() -> Vec<Route> {
    routes![
        static_routes::fetch_static_file,
        static_routes::download_static_file,
    ]
}
