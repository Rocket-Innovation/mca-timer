use anyhow::Result;
use reqwest::Client;
use sqlx::PgPool;
use std::time::Duration;

use crate::db::{db_mark_completed, db_mark_failed};
use crate::models::Timer;

/// Execute HTTP callback for a timer
pub async fn execute_callback(pool: &PgPool, timer: Timer) -> Result<()> {
    tracing::info!("Executing callback for timer {}", timer.id);

    // Create HTTP client with 30-second timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    // Build the request - always POST as per spec
    let mut request = client
        .post(&timer.callback_url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "timer-platform/0.1.0");

    // Add custom headers if present
    if let Some(headers) = &timer.callback_headers {
        if let Some(headers_map) = headers.as_object() {
            for (key, value) in headers_map {
                if let Some(value_str) = value.as_str() {
                    request = request.header(key, value_str);
                }
            }
        }
    }

    // Add JSON payload if present
    if let Some(payload) = &timer.callback_payload {
        request = request.json(payload);
    }

    // Send the request and handle response
    match request.send().await {
        Ok(response) => {
            let status = response.status();

            if status.is_success() {
                // 2xx response - mark as completed
                tracing::info!(
                    "Callback succeeded for timer {} with status {}",
                    timer.id,
                    status
                );
                db_mark_completed(pool, timer.id).await?;
            } else {
                // 4xx/5xx response - mark as failed
                let error_message = format!("HTTP {}: {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown"));
                tracing::warn!(
                    "Callback failed for timer {} with status {}: {}",
                    timer.id,
                    status,
                    error_message
                );
                db_mark_failed(pool, timer.id, error_message).await?;
            }
        }
        Err(err) => {
            // Network/timeout/TLS error - mark as failed
            let error_message = if err.is_timeout() {
                "Connection timeout after 30s".to_string()
            } else if err.is_connect() {
                format!("Connection error: {}", err)
            } else if err.is_request() {
                format!("Request error: {}", err)
            } else {
                format!("Network error: {}", err)
            };

            tracing::warn!(
                "Callback failed for timer {} with error: {}",
                timer.id,
                error_message
            );
            db_mark_failed(pool, timer.id, error_message).await?;
        }
    }

    Ok(())
}
