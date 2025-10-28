# Task: Implement Shared Data Models Module

**Status**: pending
**Dependencies**: TK-003
**Estimated Effort**: small

## Objective

Create the models module defining SHARED data structures used across multiple modules. Endpoint-specific request/response types are defined in their respective API files.

## Context

The `models.rs` module contains only the core types shared across multiple modules: Timer (internal representation), TimerStatus enum, ApiResponse wrapper, AppState, and TimerCache. Endpoint-specific types like CreateTimerRequest, UpdateTimerRequest, etc. are defined in their respective API endpoint files (api_create_timer.rs, api_update_timer.rs, etc.) following the "one endpoint per file" architecture. This keeps models.rs focused on truly shared types.

## Files to Create

- `src/models.rs` - Shared models module

## Detailed Steps

### 1. Create `src/models.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::config::Config;

// Timer status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

// Application state (shared across handlers)
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
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
```

### 2. Add module declaration to `src/main.rs`

```rust
mod models;
```

## Notes on Endpoint-Specific Types

The following types are NOT in models.rs - they live in their respective endpoint files:

- **api_create_timer.rs**: `CreateTimerRequest`
- **api_get_timer.rs**: `TimerDetailResponse`
- **api_list_timers.rs**: `ListTimersQuery`, `ListTimersResponse`
- **api_update_timer.rs**: `UpdateTimerRequest`
- **api_cancel_timer.rs**: `CancelTimerResponse`
- **api_health.rs**: `HealthData`

This separation keeps models.rs focused on truly shared types and makes endpoint files self-contained.

## Acceptance Criteria

- [ ] `TimerStatus` enum with 5 variants
- [ ] Display trait converts TimerStatus to lowercase strings
- [ ] FromStr trait parses strings to TimerStatus (case-insensitive)
- [ ] `Timer` struct with all 11 fields matching database schema
- [ ] `TimerResponse` struct for summary views (6 fields)
- [ ] `ApiResponse<T>` generic wrapper with code, message, data
- [ ] `AppState` struct with pool, config, timer_cache
- [ ] `TimerCache` type alias defined
- [ ] Helper method `Timer::to_response()` for conversion
- [ ] All types derive appropriate traits (Serialize, Deserialize, Debug, Clone)
- [ ] sqlx::Type implemented for TimerStatus

## Reference

See CLAUDE.md - "Rust Models" section for complete type definitions and "Project Structure" for file organization pattern showing shared vs endpoint-specific types.
