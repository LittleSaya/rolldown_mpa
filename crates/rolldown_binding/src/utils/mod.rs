pub mod napi_error_ext;
use napi::Env;
use std::env;
use std::sync::atomic::AtomicBool;
mod into_js_unknown_vec;
pub use into_js_unknown_vec::*;
mod js_callback;
pub use js_callback::*;
use rolldown_tracing::enable_tracing_on_demand;

static IS_ENABLE_TRACING: AtomicBool = AtomicBool::new(false);

pub fn init_custom_trace_subscriber(mut napi_env: Env) {
  let value = env::var("TRACE").unwrap_or_default();
  if value == "1" && !IS_ENABLE_TRACING.swap(true, std::sync::atomic::Ordering::SeqCst) {
    let guard = enable_tracing_on_demand();
    if let Some(guard) = guard {
      napi_env
        .add_env_cleanup_hook(guard, |flush_guard| {
          flush_guard.flush();
          drop(flush_guard);
        })
        .expect("Should able to initialize cleanup for custom trace subscriber");
    }
  }
}