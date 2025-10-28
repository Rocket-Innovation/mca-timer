# Task: Implement API Key Authentication Middleware

**Status**: pending
**Dependencies**: TK-004, TK-008
**Estimated Effort**: small

## Objective

Create authentication middleware in a dedicated `auth.rs` file that validates the X-API-Key header on all API requests except the health check endpoint.

## Context

The authentication middleware intercepts all incoming requests and validates that the `X-API-Key` header matches the API key configured in the application. This provides simple shared-key authentication for the MVP. The middleware should be applied to all routes except `/health` (which is public for load balancers and monitoring). Invalid or missing keys return a 401 Unauthorized response with code 4.

## Files to Create

- `src/auth.rs` - Authentication middleware module

## Detailed Steps

### 1. Create `src/auth.rs`

```rust
use axum::{
    extract::State,
    http::{Request, StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
    Json,
};
use std::sync::Arc;

use crate::models::{ApiResponse, AppState};

pub async fn auth_middleware<B>(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
    // Extract X-API-Key header
    let api_key = headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok());

    // Validate against configured API key
    if api_key != Some(&state.config.api_key) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                code: 4,
                message: "unauthorized".to_string(),
                data: None,
            }),
        ));
    }

    // Key is valid, proceed to handler
    Ok(next.run(req).await)
}
```

### 2. Update `src/main.rs`

Add the auth module and apply middleware to protected routes:

```rust
mod auth;

// In router setup, apply middleware to protected routes only:
let protected = Router::new()
    .route("/timers", post(api_create_timer::create_timer))
    .route("/timers", get(api_list_timers::list_timers))
    .route("/timers/:id", get(api_get_timer::get_timer))
    .route("/timers/:id", put(api_update_timer::update_timer))
    .route("/timers/:id", delete(api_cancel_timer::cancel_timer))
    .layer(axum::middleware::from_fn_with_state(
        Arc::clone(&app_state),
        auth::auth_middleware,
    ));

// Public routes (no auth)
let public = Router::new()
    .route("/health", get(api_health::health_check));

// Combine routers
let app = Router::new()
    .merge(protected)
    .merge(public)
    .with_state(app_state);
```

### 3. Add required imports to main.rs

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::trace::TraceLayer;
```

## Acceptance Criteria

- [ ] `src/auth.rs` created with auth_middleware function
- [ ] Middleware extracts X-API-Key from headers
- [ ] Missing header returns HTTP 401 with code 4
- [ ] Invalid key returns HTTP 401 with code 4
- [ ] Valid key allows request to proceed to handler
- [ ] Error response follows ApiResponse<()> format
- [ ] Middleware is async and generic over request body type `<B>`
- [ ] Applied only to protected routes (timers endpoints)
- [ ] Health check endpoint remains public (no auth required)
- [ ] main.rs imports and applies middleware correctly

## Reference

See CLAUDE.md - "Authentication" section for API key validation requirements and "Project Structure" for auth.rs file organization.
