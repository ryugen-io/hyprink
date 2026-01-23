use std::sync::OnceLock;

// Re-export types from hyprlog
pub use hyprlog::{CleanupOptions, CleanupResult, Level, LogStats};

static LOGGER: OnceLock<hyprlog::Logger> = OnceLock::new();

/// Get the global logger instance, initialized with default settings for hyprink.
fn get_logger() -> &'static hyprlog::Logger {
    LOGGER.get_or_init(|| {
        hyprlog::Logger::builder()
            .level(Level::Debug)
            .terminal()
            .colors(true)
            .done()
            .file()
            .app_name("hyprink")
            .done()
            .json()
            .app_name("hyprink")
            .done()
            .build()
    })
}

/// Log a message with the given level and scope.
pub fn log(level: Level, scope: &str, msg: &str) {
    get_logger().log(level, scope, msg);
}

/// Log an info message.
pub fn info(scope: &str, msg: &str) {
    get_logger().info(scope, msg);
}

/// Log a debug message.
pub fn debug(scope: &str, msg: &str) {
    get_logger().debug(scope, msg);
}

/// Log a warning message.
pub fn warn(scope: &str, msg: &str) {
    get_logger().warn(scope, msg);
}

/// Log an error message.
pub fn error(scope: &str, msg: &str) {
    get_logger().error(scope, msg);
}

/// Log a trace message.
pub fn trace(scope: &str, msg: &str) {
    get_logger().trace(scope, msg);
}
