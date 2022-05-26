pub struct Logger;

impl Logger {
    pub fn setup(settings: &crate::api::settings::Settings) {
        if std::env::var_os("RUST_LOG").is_none() {
            let level = settings.logger.level.as_str();
            let env = format!("dapi={},tower_http=debug", level);

            std::env::set_var("RUST_LOG", env);
        }

        tracing_subscriber::fmt::init();
    }
}