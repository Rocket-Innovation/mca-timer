# Task: Update Create Timer API Endpoint

**Status**: pending
**Dependencies**: TK-028_update-database-queries.md, TK-019_update-models-callback-types.md
**Estimated Effort**: small

## Objective

Update api_create_timer.rs to accept callback_type and callback_config instead of individual callback fields.

## Context

The Create Timer API endpoint currently accepts callback_url, callback_headers, and callback_payload. It needs to be updated to accept the new flexible callback model. Clients will send either HTTP or NATS configuration in a single callback_config object with a callback_type discriminator. Validation should ensure callback_config matches callback_type.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/api_create_timer.rs` - Update request struct and handler

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/api_create_timer.rs`
2. Locate CreateTimerRequest struct
3. Replace callback fields:
   ```rust
   // Remove these:
   pub callback_url: String,
   pub callback_headers: Option<HashMap<String, String>>,
   pub callback_payload: Option<Value>,

   // Add these:
   pub callback_type: CallbackType,
   pub callback_config: CallbackConfig,
   ```
4. Update the create_timer handler function:
   - Remove validation of callback_url format (now inside callback_config)
   - Add validation that callback_config matches callback_type:
     ```rust
     // Validate callback_type matches callback_config variant
     match (&req.callback_type, &req.callback_config) {
         (CallbackType::Http, CallbackConfig::Http(_)) => {},
         (CallbackType::Nats, CallbackConfig::Nats(_)) => {},
         _ => return Err(ApiError::validation("callback_type must match callback_config type")),
     }
     ```
   - For HTTP callbacks, validate URL format from callback_config
   - For NATS callbacks, validate topic is not empty
5. Update db_create_timer call:
   ```rust
   db_create_timer(
       &state.pool,
       id,
       req.execute_at,
       req.callback_type,
       req.callback_config,
       req.metadata,
   ).await?
   ```
6. Update response to include callback_type (callback_config not included in response)
7. Run `cargo check` to ensure the endpoint compiles

## Acceptance Criteria

- [ ] CreateTimerRequest uses callback_type and callback_config fields
- [ ] Validation ensures callback_type matches callback_config variant
- [ ] HTTP callbacks validate URL format
- [ ] NATS callbacks validate topic is not empty
- [ ] db_create_timer is called with new parameters
- [ ] Response includes callback_type
- [ ] Error messages are descriptive for validation failures
- [ ] Code compiles without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "API Endpoints" → "Create Timer" section showing new request format with callback_type and callback_config
