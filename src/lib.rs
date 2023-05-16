use tracing::warn;

pub mod acc_adapter;
pub mod model;

#[allow(dead_code)]
fn log_todo<T>(v: T, message: &str) -> T {
    warn!("TODO: {message}");
    v
}
