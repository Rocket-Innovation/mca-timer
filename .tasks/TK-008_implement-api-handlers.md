# Task: Implement API Route Handlers

**Status**: pending
**Dependencies**: TK-004, TK-005
**Estimated Effort**: medium

## Objective

Create individual API endpoint files, each containing its request/response structs and handler function. This follows the "one endpoint per file" architecture for better code organization.

## Context

Each API endpoint is implemented in its own file (e.g., `api_create_timer.rs`, `api_get_timer.rs`) containing the endpoint's request/response types and handler function. This provides clear separation of concerns and makes endpoints easy to locate. Handlers extract shared `AppState`, validate input, call database operations, and return standardized JSON responses using `ApiResponse<T>`. The router in `main.rs` will import and mount all handlers.

## Files to Create

- `src/api_create_timer.rs` - POST /timers endpoint
- `src/api_get_timer.rs` - GET /timers/:id endpoint
- `src/api_list_timers.rs` - GET /timers endpoint
- `src/api_update_timer.rs` - PUT /timers/:id endpoint
- `src/api_cancel_timer.rs` - DELETE /timers/:id endpoint
- `src/api_health.rs` - GET /health endpoint

## Detailed Steps

### 1. Create `src/api_create_timer.rs`

```rust
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{db, models::{ApiResponse, AppState, TimerResponse}};

#[derive(Debug, Deserialize)]
pub struct CreateTimerRequest {
    pub execute_at: chrono::DateTime<Utc>,
    pub callback_url: String,
    pub callback_headers: Option<serde_json::Value>,
    pub callback_payload: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn create_timer(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTimerRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TimerResponse>>), (StatusCode, Json<ApiResponse<()>>)> {
    // Validate execute_at is in future (> NOW + 5 seconds)
    // Validate callback_url starts with http:// or https://
    // Generate UUIDv7 for timer ID
    // Call db::db_create_timer()
    // Convert Timer to TimerResponse
    // Return HTTP 201 with code 0
}
```

### 2. Create `src/api_get_timer.rs`

```rust
use axum::{extract::{State, Path}, http::StatusCode, Json};
use std::sync::Arc;
use uuid::Uuid;

use crate::{db, models::{ApiResponse, AppState}};

#[derive(Debug, Serialize)]
pub struct TimerDetailResponse {
    pub id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub execute_at: chrono::DateTime<Utc>,
    pub callback_url: String,
    pub callback_headers: Option<serde_json::Value>,
    pub callback_payload: Option<serde_json::Value>,
    pub status: String,
    pub last_error: Option<String>,
    pub executed_at: Option<chrono::DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn get_timer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<TimerDetailResponse>>), (StatusCode, Json<ApiResponse<()>>)> {
    // Call db::db_get_timer()
    // If None: return HTTP 404 with code 3
    // Convert Timer to TimerDetailResponse
    // Return HTTP 200 with code 0
}
```

### 3. Create `src/api_list_timers.rs`

```rust
use axum::{extract::{State, Query}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{db, models::{ApiResponse, AppState, TimerResponse}};

#[derive(Debug, Deserialize)]
pub struct ListTimersQuery {
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListTimersResponse {
    pub timers: Vec<TimerResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

pub async fn list_timers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListTimersQuery>,
) -> Result<(StatusCode, Json<ApiResponse<ListTimersResponse>>), (StatusCode, Json<ApiResponse<()>>)> {
    // Set defaults: limit=50 (max 200), offset=0, sort=created_at, order=desc
    // Call db::db_list_timers()
    // Convert Vec<Timer> to Vec<TimerResponse>
    // Return HTTP 200 with code 0
}
```

### 4. Create `src/api_update_timer.rs`

```rust
use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{db, models::{ApiResponse, AppState, TimerResponse}};

#[derive(Debug, Deserialize)]
pub struct UpdateTimerRequest {
    pub execute_at: Option<chrono::DateTime<Utc>>,
    pub callback_url: Option<String>,
    pub callback_headers: Option<serde_json::Value>,
    pub callback_payload: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn update_timer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTimerRequest>,
) -> Result<(StatusCode, Json<ApiResponse<TimerResponse>>), (StatusCode, Json<ApiResponse<()>>)> {
    // Fetch existing timer to check status
    // Reject if status is completed, failed, or canceled (HTTP 400, code 2)
    // Validate new execute_at is in future if provided
    // Call db::db_update_timer()
    // Return HTTP 200 with code 0
}
```

### 5. Create `src/api_cancel_timer.rs`

```rust
use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{db, models::{ApiResponse, AppState}};

#[derive(Debug, Serialize)]
pub struct CancelTimerResponse {
    pub id: Uuid,
    pub status: String,
}

pub async fn cancel_timer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<CancelTimerResponse>>), (StatusCode, Json<ApiResponse<()>>)> {
    // Fetch existing timer to check status
    // Reject if status is completed or failed (HTTP 400, code 2)
    // Call db::db_cancel_timer()
    // Return HTTP 200 with code 0
}
```

### 6. Create `src/api_health.rs`

```rust
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;

use crate::models::{ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct HealthData {
    pub status: String,
    pub database: String,
    pub timestamp: chrono::DateTime<Utc>,
}

pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<HealthData>>) {
    // Test database: sqlx::query("SELECT 1").fetch_one(&state.pool).await
    // On success: HTTP 200, code 0, status="up", database="connected"
    // On error: HTTP 500, code 1, status="degraded", database="disconnected"
}
```

### 7. Update `src/main.rs` router

Import all handlers and mount routes:

```rust
mod api_create_timer;
mod api_get_timer;
mod api_list_timers;
mod api_update_timer;
mod api_cancel_timer;
mod api_health;

// In router setup:
let app = Router::new()
    .route("/timers", post(api_create_timer::create_timer))
    .route("/timers", get(api_list_timers::list_timers))
    .route("/timers/:id", get(api_get_timer::get_timer))
    .route("/timers/:id", put(api_update_timer::update_timer))
    .route("/timers/:id", delete(api_cancel_timer::cancel_timer))
    .route("/health", get(api_health::health_check))
    .with_state(Arc::new(state));
```

## Acceptance Criteria

- [ ] Six separate API files created, each with endpoint-specific types and handler
- [ ] Create endpoint validates input and returns 201 on success
- [ ] Get endpoint returns 404 when timer not found
- [ ] List endpoint supports filtering, sorting, pagination with defaults
- [ ] Update endpoint prevents updating completed/failed/canceled timers
- [ ] Cancel endpoint prevents canceling completed/failed timers
- [ ] Health check tests database connectivity
- [ ] All responses follow ApiResponse<T> format with correct codes
- [ ] Validation errors return HTTP 400 with code 2
- [ ] Not found errors return HTTP 404 with code 3
- [ ] Database errors return HTTP 500 with code 1
- [ ] Router in main.rs imports and mounts all handlers

## Reference

See CLAUDE.md - "API Endpoints" section for complete endpoint specifications. Also see "Project Structure" for file organization pattern.
