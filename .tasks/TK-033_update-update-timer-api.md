# Task: Update Update Timer API Endpoint

**Status**: pending
**Dependencies**: TK-028_update-database-queries.md, TK-019_update-models-callback-types.md
**Estimated Effort**: small

## Objective

Update api_update_timer.rs to support updating callback_type and callback_config fields.

## Context

The Update Timer endpoint allows modifying pending timers. With the new callback model, clients can update both the callback_type and callback_config. All fields remain optional (partial updates). Validation must ensure callback_type matches callback_config variant if both are provided. Only pending timers can be updated.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/api_update_timer.rs` - Update request struct and handler

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/api_update_timer.rs`
2. Locate UpdateTimerRequest struct
3. Replace callback fields:
   ```rust
   // Remove these:
   pub callback_url: Option<String>,
   pub callback_headers: Option<HashMap<String, String>>,
   pub callback_payload: Option<Value>,

   // Add these:
   pub callback_type: Option<CallbackType>,
   pub callback_config: Option<CallbackConfig>,
   ```
4. Update the update_timer handler function:
   - Add validation: if both callback_type and callback_config are provided, ensure they match
     ```rust
     if let (Some(cb_type), Some(cb_config)) = (&req.callback_type, &req.callback_config) {
         match (cb_type, cb_config) {
             (CallbackType::Http, CallbackConfig::Http(_)) => {},
             (CallbackType::Nats, CallbackConfig::Nats(_)) => {},
             _ => return Err(ApiError::validation("callback_type must match callback_config type")),
         }
     }
     ```
   - Add validation: if only callback_type is provided without callback_config (or vice versa), return error
     ```rust
     if req.callback_type.is_some() != req.callback_config.is_some() {
         return Err(ApiError::validation("callback_type and callback_config must be updated together"));
     }
     ```
   - For HTTP callbacks, validate URL format if callback_config contains HTTP
   - For NATS callbacks, validate topic is not empty if callback_config contains NATS
5. Update db_update_timer call:
   ```rust
   db_update_timer(
       &state.pool,
       timer_id,
       req.execute_at,
       req.callback_type,
       req.callback_config,
       req.metadata,
   ).await?
   ```
6. Ensure only pending timers can be updated (existing logic should remain)
7. Run `cargo check` to ensure the endpoint compiles

## Acceptance Criteria

- [ ] UpdateTimerRequest uses optional callback_type and callback_config fields
- [ ] Validation ensures callback_type and callback_config are updated together
- [ ] Validation ensures callback_type matches callback_config variant
- [ ] HTTP callbacks validate URL format
- [ ] NATS callbacks validate topic is not empty
- [ ] db_update_timer is called with new parameters
- [ ] Only pending timers can be updated (existing constraint preserved)
- [ ] Error messages are descriptive for validation failures
- [ ] Code compiles without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "API Endpoints" → "Update Timer" section showing request format with optional callback_type and callback_config
