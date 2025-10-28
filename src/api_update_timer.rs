use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{Duration, Utc};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    db,
    models::{ApiResponse, AppState, TimerResponse, TimerStatus},
};

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
) -> Result<
    (StatusCode, Json<ApiResponse<TimerResponse>>),
    (StatusCode, Json<ApiResponse<()>>),
> {
    // Fetch existing timer to check status
    let existing_timer = match db::db_get_timer(&state.pool, id).await {
        Ok(Some(timer)) => timer,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ApiResponse::<()>::error(3, "timer not found")),
            ));
        }
        Err(err) => {
            tracing::error!("Failed to get timer {}: {}", id, err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(1, format!("Database error: {}", err))),
            ));
        }
    };

    // Reject if status is completed, failed, or canceled
    if matches!(
        existing_timer.status,
        TimerStatus::Completed | TimerStatus::Failed | TimerStatus::Canceled
    ) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                2,
                format!("cannot update timer with status '{}'", existing_timer.status),
            )),
        ));
    }

    // Validate new execute_at is in future if provided
    if let Some(execute_at) = req.execute_at {
        let now = Utc::now();
        let min_execute_time = now + Duration::seconds(5);

        if execute_at <= min_execute_time {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::error(
                    2,
                    "execute_at must be at least 5 seconds in the future",
                )),
            ));
        }
    }

    // Validate callback_url if provided
    if let Some(ref url) = req.callback_url {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::error(
                    2,
                    "callback_url must start with http:// or https://",
                )),
            ));
        }
    }

    // Update timer
    match db::db_update_timer(
        &state.pool,
        id,
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
            Ok((StatusCode::OK, Json(ApiResponse::success(response))))
        }
        Err(err) => {
            tracing::error!("Failed to update timer {}: {}", id, err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(1, format!("Database error: {}", err))),
            ))
        }
    }
}
