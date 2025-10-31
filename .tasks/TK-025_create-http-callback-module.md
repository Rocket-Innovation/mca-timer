# Task: Create HTTP Callback Module

**Status**: pending
**Dependencies**: TK-022_update-timer-model-struct.md
**Estimated Effort**: small

## Objective

Extract HTTP callback logic from callback.rs into a new dedicated callback_http.rs module.

## Context

Currently, callback.rs contains HTTP-specific callback execution logic. With dual callback support (HTTP and NATS), we need to separate HTTP logic into its own module. This follows the project's flat structure principle and makes the codebase easier to maintain. The HTTP callback executes POST requests with custom headers and JSON payloads.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/callback_http.rs` - New module for HTTP callbacks

## Detailed Steps

1. Create `/Users/tar/Documents/alpha/timer/src/callback_http.rs`
2. Add module header documentation:
   ```rust
   //! HTTP callback execution module
   //! Handles HTTP POST requests to external webhook URLs
   ```
3. Add necessary imports:
   ```rust
   use crate::models::{HTTPCallback, Timer};
   use reqwest::Client;
   use std::time::Duration;
   use tracing::{info, warn};
   ```
4. Create the execute_http_callback function:
   ```rust
   pub async fn execute_http_callback(
       timer: &Timer,
       http_config: &HTTPCallback,
   ) -> Result<(), String> {
       // Build HTTP client with 30s timeout
       let client = Client::builder()
           .timeout(Duration::from_secs(30))
           .build()
           .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

       // Build request
       let mut request = client
           .post(&http_config.url)
           .header("Content-Type", "application/json")
           .header("User-Agent", "timer-platform/0.1.0");

       // Add custom headers if present
       if let Some(headers) = &http_config.headers {
           if let Some(obj) = headers.as_object() {
               for (key, value) in obj {
                   if let Some(val_str) = value.as_str() {
                       request = request.header(key, val_str);
                   }
               }
           }
       }

       // Add payload if present
       if let Some(payload) = &http_config.payload {
           request = request.json(payload);
       }

       // Execute request
       match request.send().await {
           Ok(response) => {
               if response.status().is_success() {
                   info!("HTTP callback succeeded for timer {}: {}", timer.id, response.status());
                   Ok(())
               } else {
                   let error = format!("HTTP {} from {}", response.status(), http_config.url);
                   warn!("HTTP callback failed for timer {}: {}", timer.id, error);
                   Err(error)
               }
           }
           Err(e) => {
               let error = format!("HTTP request failed: {}", e);
               warn!("HTTP callback failed for timer {}: {}", timer.id, error);
               Err(error)
           }
       }
   }
   ```
5. Run `cargo check` to ensure the module compiles

## Acceptance Criteria

- [ ] callback_http.rs file is created in src/
- [ ] execute_http_callback function accepts Timer and HTTPCallback references
- [ ] Function builds HTTP POST request with URL, headers, and payload
- [ ] Custom headers from callback_headers are merged into request
- [ ] 30-second timeout is configured
- [ ] 2xx responses return Ok(()), non-2xx return Err with status code
- [ ] Network errors return Err with error message
- [ ] Appropriate tracing logs for success and failure
- [ ] Code compiles without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Callback Execution" section for HTTP request construction details
