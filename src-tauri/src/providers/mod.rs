//! Provider system entry point.
//!
//! Future implementations:
//! - HTTP health checks
//! - JSON API usage
//! - VPS Agent

pub trait Provider {
    fn name(&self) -> &str;
}
