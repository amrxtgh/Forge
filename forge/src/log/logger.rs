use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::ChronoLocal},
};

pub struct Logger {}
impl Logger {
    pub fn init() {
        let subscriber = fmt::fmt()
            .with_target(false)
            .with_file(false)
            .with_line_number(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_env_filter(EnvFilter::from_default_env())
            .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".into()))
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("Failed to initialize logger");
    }
    pub fn info(message: &str) {
        info!("{message}");
    }
    pub fn warn(message: &str) {
        warn!("{message}");
    }
    pub fn debug(message: &str) {
        debug!("{message}");
    }
    pub fn error(message: &str) {
        error!("{message}");
    }
    pub fn trace(message: &str) {
        trace!("{message}");
    }
}
