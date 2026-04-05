use url_shortener_infra::config::AppConfig;

fn main() {
    let _config = AppConfig::load();
    println!("CLI started");
}
