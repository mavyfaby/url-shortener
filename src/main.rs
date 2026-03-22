mod config;

use config::AppConfig;

fn main() {
    let config = AppConfig::load();

    println!("TZ: {}", config.tz);
    println!("PORT: {}", config.port);
    println!("POSTGRES_HOST: {}", config.postgres_host);
    println!("POSTGRES_PORT: {}", config.postgres_port);
    println!("POSTGRES_USER: {}", config.postgres_user);
    println!("POSTGRES_PASSWORD: {}", config.postgres_password);
    println!("POSTGRES_DB: {}", config.postgres_db);
}
