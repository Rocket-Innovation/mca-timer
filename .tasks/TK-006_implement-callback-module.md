# Task: Implement HTTP Callback Execution Module

**Status**: pending
**Dependencies**: TK-004, TK-005
**Estimated Effort**: small

## Objective

Create the callback execution module that sends HTTP requests to external webhook URLs and handles response processing.

## Context

The `callback.rs` module handles the execution of HTTP callbacks when timers are due. It constructs HTTP requests using the timer's callback configuration (URL, method, headers, payload), sends the request with a 30-second timeout, and updates the timer status based on the response. Success (2xx) marks the timer as completed, while any failure (4xx/5xx, timeout, network error) immediately marks it as failed with no retries. The module uses reqwest for HTTP client operations and updates the database directly after each execution attempt.

## Files to Modify/Create

- `src/callback.rs` - New callback execution module

## Detailed Steps

1. Create `src/callback.rs` file
2. Import dependencies: `reqwest::Client`, `std::time::Duration`, `uuid::Uuid`, `sqlx::PgPool`, models, db operations
3. Implement `execute_callback()` async function:
   - Takes: pool (PgPool), timer (Timer)
   - Creates HTTP client with 30-second timeout: `Client::builder().timeout(Duration::from_secs(30)).build()?`
   - Determines HTTP method from timer.callback_method (should always be "POST" per spec)
   - Builds request:
     - URL: timer.callback_url
     - Method: POST (hardcoded as per spec)
     - Headers: Always add `Content-Type: application/json`, `User-Agent: timer-platform/0.1.0`
     - Merge custom headers from timer.callback_headers if present
     - Body: serialize timer.callback_payload as JSON (if present)
   - Sends request asynchronously
   - Handles response:
     - If status is 2xx (200-299): Call `db_mark_completed(pool, timer.id)`
     - If status is 4xx/5xx or request fails: Extract error message and call `db_mark_failed(pool, timer.id, error_msg)`
   - Logs execution result (success or failure) with timer ID
   - Returns `Result<(), anyhow::Error>`
4. Add error handling for:
   - Network/DNS errors
   - Connection timeouts
   - TLS/SSL errors
   - HTTP error responses
5. Format error messages descriptively (e.g., "HTTP 500: Internal Server Error", "Connection timeout after 30s")

## Acceptance Criteria

- [ ] `execute_callback()` function constructs HTTP POST request correctly
- [ ] 30-second timeout enforced on all requests
- [ ] Standard headers (`Content-Type`, `User-Agent`) always included
- [ ] Custom headers from timer.callback_headers merged into request
- [ ] JSON payload serialized correctly in request body
- [ ] 2xx responses mark timer as completed via `db_mark_completed()`
- [ ] Non-2xx responses and errors mark timer as failed via `db_mark_failed()`
- [ ] Error messages stored in last_error field are descriptive
- [ ] No retry logic implemented (single attempt only)
- [ ] Execution results logged with timer ID for observability

## Reference

See CLAUDE.md - "Callback Execution" section for HTTP request construction details, success/failure criteria, and "Callback Execution Implementation" for the complete flow diagram and error handling requirements.
