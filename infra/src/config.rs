use std::env;

pub struct AppConfig {
    pub tz: String,
    
    pub port: u16,

    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let env_file = env::var("ENV_FILE")
            .expect("ENV_FILE env is required.");

        let env_file_path = env::current_dir()
            .expect("Error occurred while getting current directory.")
            .join(env_file);

        dotenvy::from_path(env_file_path)
            .expect("Cannot load environment file.");

        Self {
            tz: env::var("TZ")
                .expect("TZ env is required."),
            port: env::var("PORT")
                .expect("PORT env is required.")
                .parse::<u16>()
                .expect("PORT must be a valid port."),
            postgres_host: env::var("POSTGRES_HOST")
                .expect("POSTGRES_HOST env is required."),
            postgres_port: env::var("POSTGRES_PORT")
                .expect("POSTGRES_PORT env is required.")
                .parse::<u16>()
                .expect("POSTGRES_PORT must be a valid port."),
            postgres_user: env::var("POSTGRES_USER")
                .expect("POSTGRES_USER env is required."),
            postgres_password: env::var("POSTGRES_PASSWORD")
                .expect("POSTGRES_PASSWORD env is required."),
            postgres_db: env::var("POSTGRES_DB")
                .expect("POSTGRES_DB env is required."),
        }
    }
}
