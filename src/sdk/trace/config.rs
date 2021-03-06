//! SDK Configuration
//!
//! Configuration represents the global tracing configuration, overrides
//! can be set for the default OpenTelemetry limits and Sampler.
use crate::api;

/// Tracer configuration
#[derive(Debug)]
pub struct Config {
    /// The sampler that the sdk should use
    pub default_sampler: api::Sampler,
    /// The max events that can be added to a `Span`.
    pub max_events_per_span: u32,
    /// The max attributes that can be added to a `Span`.
    pub max_attributes_per_span: u32,
    /// The max links that can be added to a `Span`.
    pub max_links_per_span: u32,
}

impl Default for Config {
    /// Create default global sdk configuration.
    fn default() -> Self {
        Config {
            default_sampler: api::Sampler::Always,
            max_events_per_span: 128,
            max_attributes_per_span: 32,
            max_links_per_span: 32,
        }
    }
}
