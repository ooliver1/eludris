#[cfg(test)]
mod tests;

#[macro_use]
extern crate rocket;

mod cors;
mod ratelimit;
mod routes;

use std::{alloc::System, env, ffi::OsStr, io, rc::Rc};

use anyhow::Context;
use libloading::Library;
use rocket::{Build, Config, Rocket};
use rocket_db_pools::Database;
use routes::*;
use todel::Conf;

use plugins::{Plugin, RequestListener};

#[global_allocator]
static ALLOCATOR: System = System;

struct PluginRegistrar {
    callbacks: Vec<RequestListenerProxy>,
    lib: Rc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Rc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            callbacks: Vec::default(),
        }
    }
}

impl plugins::PluginRegistrar for PluginRegistrar {
    fn register_request_listener(&mut self, callback: Box<dyn RequestListener>) {
        let proxy = RequestListenerProxy {
            callback,
            _lib: Rc::clone(&self.lib),
        };
        self.callbacks.push(proxy);
    }
}

#[derive(Default)]
pub struct RequestListeners {
    callbacks: Vec<RequestListenerProxy>,
    libraries: Vec<Rc<Library>>,
}

impl RequestListeners {
    pub fn new() -> RequestListeners {
        RequestListeners::default()
    }

    /// # Safety
    ///
    /// A plugin **must** be implemented using the
    /// [`plugins::export_plugin!()`] macro. Trying to manually implement
    /// a plugin without going through that macro will result in undefined
    /// behaviour.
    pub unsafe fn load<P: AsRef<OsStr>>(
        &mut self,
        library_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let library = Rc::new(Library::new(library_path)?);

        let plugin = library.get::<*mut Plugin>(b"plugin\0")?.read();

        // version checks to prevent accidental ABI incompatibilities
        if plugin.rustc_version != plugins::RUSTC_VERSION
            || plugin.core_version != plugins::CORE_VERSION
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Version mismatch",
            )));
        }

        let mut registrar = PluginRegistrar::new(Rc::clone(&library));

        (plugin.register)(&mut registrar);

        self.callbacks.extend(registrar.callbacks);
        self.libraries.push(library);

        Ok(())
    }
}

pub struct RequestListenerProxy {
    callback: Box<dyn RequestListener>,
    _lib: Rc<Library>,
}

impl RequestListener for RequestListenerProxy {
    fn call(&self, request: rocket::Request) {
        self.callback.call(request)
    }
}

#[derive(Database)]
#[database("cache")]
pub struct Cache(deadpool_redis::Pool);

fn rocket() -> Result<Rocket<Build>, anyhow::Error> {
    #[cfg(test)]
    {
        env::set_var("ELUDRIS_CONF", "../tests/Eludris.toml");
    }
    dotenv::dotenv().ok();
    env_logger::try_init().ok();

    let config = Config::figment()
        .merge((
            "port",
            env::var("OPRISH_PORT")
                .unwrap_or_else(|_| "7159".to_string())
                .parse::<u32>()
                .context("Invalid \"OPRISH_PORT\" environment variable")?,
        ))
        .merge((
            "databases.db",
            rocket_db_pools::Config {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "mysql://root:root@localhost:3306/eludris".to_string()),
                min_connections: None,
                max_connections: 1024,
                connect_timeout: 3,
                idle_timeout: None,
            },
        ))
        .merge((
            "databases.cache",
            rocket_db_pools::Config {
                url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
                min_connections: None,
                max_connections: 1024,
                connect_timeout: 3,
                idle_timeout: None,
            },
        ));

    Ok(rocket::custom(config)
        .mount("/", get_routes())
        .mount("/messages", messages::get_routes())
        .manage(Conf::new_from_env()?)
        .attach(Cache::init())
        .attach(cors::Cors))
}

#[rocket::main]
async fn main() -> Result<(), anyhow::Error> {
    let _ = rocket()?
        .launch()
        .await
        .context("Encountered an error while running Rest API")?;

    Ok(())
}
