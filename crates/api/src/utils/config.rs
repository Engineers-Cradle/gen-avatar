use confik::Configuration;

#[derive(Debug, Default, Configuration)]
pub struct AppConfig {
    pub web_server_port: u16,
    pub redis_url: String,
    pub num_workers: usize,
    pub log_level: String,
}