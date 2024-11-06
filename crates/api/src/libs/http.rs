use crate::handler::{avatar, health};

use actix_web::{web, App, HttpServer};
use env_logger::Env;
use listenfd::ListenFd;
use tracing_actix_web::TracingLogger;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
}

#[actix_web::main]
pub async fn start_web_server() -> std::io::Result<()> {
    let env_config: crate::utils::config::AppConfig = crate::env_config();

    let redis_client: redis::Client =
        crate::libs::redis::connection_to_redis(&env_config.redis_url).await;

    let app_state: AppState = AppState {
        redis_client: redis_client.clone(),
    };

    env_logger::init_from_env(Env::default().default_filter_or(env_config.log_level));

    let mut listenfd: ListenFd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(TracingLogger::default())
            .configure(avatar::init_avatar_routes)
            .configure(health::init_health_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host: &str = "0.0.0.0";
            let port: u16 = env_config.web_server_port;

            println!("Web Server started at http://{}:{}", host, port);

            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.workers(env_config.num_workers).run().await
}
