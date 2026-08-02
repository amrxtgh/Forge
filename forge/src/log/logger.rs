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
        info!("Initialized log");
    }

    // core log infos
    pub fn core_info(message: &str) {
        info!("[CORE] {message}");
    }
    pub fn core_warn(message: &str) {
        warn!("[CORE] {message}");
    }
    pub fn core_debug(message: &str) {
        debug!("[CORE] {message}");
    }
    pub fn core_error(message: &str) {
        error!("[CORE] {message}");
    }
    pub fn core_trace(message: &str) {
        trace!("[CORE] {message}");
    }

    // client log infos
    pub fn client_info(message: &str) {
        info!("[CLIENT] {message}");
    }
    pub fn client_warn(message: &str) {
        warn!("[CLIENT] {message}");
    }
    pub fn client_debug(message: &str) {
        debug!("[CLIENT] {message}");
    }
    pub fn client_error(message: &str) {
        error!("[CLIENT] {message}");
    }
    pub fn client_trace(message: &str) {
        trace!("[CLIENT] {message}");
    }
}
