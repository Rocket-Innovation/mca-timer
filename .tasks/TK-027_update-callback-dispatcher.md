# Task: Update Callback Dispatcher Module

**Status**: pending
**Dependencies**: TK-025_create-http-callback-module.md, TK-026_create-nats-callback-module.md
**Estimated Effort**: small

## Objective

Refactor callback.rs to become a dispatcher that routes callback execution to either callback_http or callback_nats based on callback_type.

## Context

The current callback.rs contains HTTP-specific logic. We need to transform it into a dispatcher that examines the Timer's callback_type and callback_config, then delegates to the appropriate module. This maintains a clean separation of concerns and makes it easy to add more callback types in the future.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/callback.rs` - Refactor into dispatcher

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/callback.rs`
2. Replace all existing HTTP-specific code with dispatcher logic
3. Add imports:
   ```rust
   use crate::callback_http::execute_http_callback;
   use crate::callback_nats::execute_nats_callback;
   use crate::models::{CallbackConfig, Timer};
   use crate::db::db_update_timer_status;
   use async_nats::Client as NatsClient;
   use chrono::Utc;
   use sqlx::PgPool;
   use tracing::{info, warn};
   ```
4. Create new execute_callback function:
   ```rust
   pub async fn execute_callback(
       timer: Timer,
       pool: &PgPool,
       nats_client: Option<&NatsClient>,
   ) {
       info!("Executing callback for timer {}", timer.id);

       // Dispatch to appropriate callback handler
       let result = match &timer.callback_config {
           CallbackConfig::Http(http_config) => {
               execute_http_callback(&timer, http_config).await
           }
           CallbackConfig::Nats(nats_config) => {
               if let Some(client) = nats_client {
                   execute_nats_callback(&timer, nats_config, client).await
               } else {
                   Err("NATS client not available (NATS_URL not configured)".to_string())
               }
           }
       };

       // Update timer status based on result
       let executed_at = Utc::now();
       match result {
           Ok(_) => {
               info!("Callback completed successfully for timer {}", timer.id);
               if let Err(e) = db_update_timer_status(
                   pool,
                   timer.id,
                   "completed",
                   None,
                   Some(executed_at),
               ).await {
                   warn!("Failed to update timer status: {}", e);
               }
           }
           Err(error_msg) => {
               warn!("Callback failed for timer {}: {}", timer.id, error_msg);
               if let Err(e) = db_update_timer_status(
                   pool,
                   timer.id,
                   "failed",
                   Some(&error_msg),
                   Some(executed_at),
               ).await {
                   warn!("Failed to update timer status: {}", e);
               }
           }
       }
   }
   ```
5. Remove all old HTTP-specific code (request building, reqwest usage, etc.)
6. Ensure the function signature matches what scheduler.rs expects
7. Run `cargo check` to ensure the module compiles

## Acceptance Criteria

- [ ] callback.rs no longer contains HTTP-specific request building
- [ ] execute_callback function dispatches based on CallbackConfig enum
- [ ] HTTP callbacks are routed to callback_http::execute_http_callback
- [ ] NATS callbacks are routed to callback_nats::execute_nats_callback
- [ ] NATS callbacks return error if nats_client is None
- [ ] Timer status is updated to 'completed' on success
- [ ] Timer status is updated to 'failed' with error message on failure
- [ ] executed_at timestamp is set for both success and failure
- [ ] Appropriate tracing logs throughout
- [ ] Code compiles without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Callback Execution Implementation" section describing dispatcher pattern
