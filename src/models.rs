use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::config::Config;

// Timer status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TimerStatus {
    Pending,
    Executing,
    Completed,
    Failed,
    Canceled,
}

impl std::fmt::Display for TimerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerStatus::Pending => write!(f, "pending"),
            TimerStatus::Executing => write!(f, "executing"),
            TimerStatus::Completed => write!(f, "completed"),
            TimerStatus::Failed => write!(f, "failed"),
            TimerStatus::Canceled => write!(f, "canceled"),
        }
    }
}

impl std::str::FromStr for TimerStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(TimerStatus::Pending),
            "executing" => Ok(TimerStatus::Executing),
            "completed" => Ok(TimerStatus::Completed),
            "failed" => Ok(TimerStatus::Failed),
            "canceled" => Ok(TimerStatus::Canceled),
            _ => Err(format!("Invalid timer status: {}", s)),
        }
    }
}

// Internal Timer struct (matches database schema)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Timer {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub execute_at: DateTime<Utc>,
    pub callback_url: String,
    pub callback_headers: Option<serde_json::Value>,
    pub callback_payload: Option<serde_json::Value>,
    pub status: TimerStatus,
    pub last_error: Option<String>,
    pub executed_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

// Shared response type (used by multiple endpoints)
#[derive(Debug, Serialize, Deserialize)]
pub struct TimerResponse {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub execute_at: DateTime<Utc>,
    pub callback_url: String,
    pub status: String,
    pub executed_at: Option<DateTime<Utc>>,
}

// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
        }
    }
}

// Application state (shared across handlers)
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    #[allow(dead_code)] // Used by scheduler in background tasks
    pub timer_cache: TimerCache,
}

// Type alias for timer cache
pub type TimerCache = Arc<RwLock<HashMap<Uuid, Timer>>>;

// Helper functions for type conversions
impl Timer {
    /// Convert Timer to TimerResponse (summary view)
    pub fn to_response(&self) -> TimerResponse {
        TimerResponse {
            id: self.id,
            created_at: self.created_at,
            execute_at: self.execute_at,
            callback_url: self.callback_url.clone(),
            status: self.status.to_string(),
            executed_at: self.executed_at,
        }
    }
}
