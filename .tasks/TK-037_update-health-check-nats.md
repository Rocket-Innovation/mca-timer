# Task: Update Health Check to Include NATS Status

**Status**: pending
**Dependencies**: TK-023_add-nats-client-to-appstate.md
**Estimated Effort**: small

## Objective

Enhance the health check endpoint to report NATS connection status alongside database status.

## Context

The /health endpoint currently checks database connectivity. With NATS support, it should also report whether NATS is configured and connected. This helps operations teams monitor service health and identify NATS-specific issues. The health check remains public (no auth required).

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/api_health.rs` - Add NATS status to response

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/api_health.rs`
2. Locate the HealthResponse struct
3. Add nats_status field:
   ```rust
   pub struct HealthResponse {
       pub status: String,
       pub database: String,
       pub nats: String,  // Add this field
       pub timestamp: DateTime<Utc>,
   }
   ```
4. Update the health check handler:
   - Check if nats_client is Some in AppState
   - If Some, try to get NATS server info or just report "connected"
   - If None, report "not_configured"
   - Don't fail health check if NATS is not configured (it's optional)
   ```rust
   let nats_status = if state.nats_client.is_some() {
       "connected"
   } else {
       "not_configured"
   };
   ```
5. Update response construction:
   ```rust
   HealthResponse {
       status: "up".to_string(),
       database: "connected".to_string(),
       nats: nats_status.to_string(),
       timestamp: Utc::now(),
   }
   ```
6. Consider: Should health check fail if NATS is configured but disconnected?
   - Decision: No, only database failure should return HTTP 500
   - NATS failures are logged but don't affect overall health
7. Run `cargo check` to ensure the endpoint compiles

## Acceptance Criteria

- [ ] HealthResponse includes nats field
- [ ] Health check reports "connected" if NATS client exists
- [ ] Health check reports "not_configured" if NATS client is None
- [ ] Overall health status remains "up" even if NATS not configured
- [ ] Database failure still returns HTTP 500 (existing behavior)
- [ ] NATS status doesn't affect HTTP status code (NATS is optional)
- [ ] Response JSON includes nats status
- [ ] Code compiles without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "API Endpoints" → "Health Check" section (note: may need updating to reflect NATS status in spec)
