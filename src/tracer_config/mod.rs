use std::sync::RwLock;

lazy_static::lazy_static! {
    static ref TRACER_CONFIG: RwLock<TracerConfiguration> = RwLock::new(Default::default());
}

#[derive(Clone, Default)]
pub struct TracerConfiguration {
    pub context_enabled: Option<bool>,
    pub timestamp_enabled: Option<bool>,
    pub timestamp_type: Option<TimestampType>,
    pub file_enabled: Option<bool>,
    pub line_enabled: Option<bool>,
    pub format: Option<String>,
}

impl TracerConfiguration {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone, Copy)]
pub enum TimestampType {
    Chrono,
    Unix,
}

pub fn set_tracer_config(config: TracerConfiguration) {
    *TRACER_CONFIG.write().unwrap() = config;
}

pub fn get_tracer_config() -> TracerConfiguration {
    (*TRACER_CONFIG.read().unwrap()).clone()
}