use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    db,
    models::{ApiResponse, AppState, TimerResponse},
};

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
) -> Result<
    (StatusCode, Json<ApiResponse<TimerResponse>>),
    (StatusCode, Json<ApiResponse<()>>),
> {
    // Validate execute_at is in future (> NOW + 5 seconds)
    let now = Utc::now();
    let min_execute_time = now + Duration::seconds(5);

    if req.execute_at <= min_execute_time {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                2,
                "execute_at must be at least 5 seconds in the future",
            )),
        ));
    }

    // Validate callback_url starts with http:// or https://
    if !req.callback_url.starts_with("http://") && !req.callback_url.starts_with("https://") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                2,
                "callback_url must start with http:// or https://",
            )),
        ));
    }

    // Create timer in database
    match db::db_create_timer(
        &state.pool,
        req.execute_at,
        req.callback_url,
        req.callback_headers,
        req.callback_payload,
        req.metadata,
    )
    .await
    {
        Ok(timer) => {
            let response = timer.to_response();
            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::success(response)),
            ))
        }
        Err(err) => {
            tracing::error!("Failed to create timer: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(1, format!("Database error: {}", err))),
            ))
        }
    }
}
