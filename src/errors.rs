//! spust errors

use std::{fmt, sync::Arc};

/// spust error wrapper
#[derive(Debug)]
pub struct SpustError {
    /// error message
    pub message: String,
    /// source error
    pub source: Option<Arc<dyn std::error::Error + Send + Sync>>,
}

impl Clone for SpustError {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            source: self.source.clone(),
        }
    }
}
impl fmt::Display for SpustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            Some(src) => write!(f, "{} - caused by: {}", self.message, src),
            None => write!(f, "{}", self.message),
        }
    }
}

impl SpustError {
    /// Create new AppError
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let ref_str = s.as_ref();
        let message = ref_str.to_string();
        Self {
            message,
            source: None,
        }
    }

    pub fn new_with_source<S: AsRef<str>>(
        s: S,
        src: Arc<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        let ref_str = s.as_ref();
        let message = ref_str.to_string();
        Self {
            message,
            source: Some(src),
        }
    }
}

impl From<sqlx::Error> for SpustError {
    fn from(value: sqlx::Error) -> Self {
        Self {
            message: value.to_string(),
            source: Some(Arc::new(value)),
        }
    }
}

impl From<std::io::Error> for SpustError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
            source: Some(Arc::new(value)),
        }
    }
}
