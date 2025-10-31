# Task: Update Get Timer API Endpoint

**Status**: pending
**Dependencies**: TK-028_update-database-queries.md, TK-019_update-models-callback-types.md
**Estimated Effort**: small

## Objective

Update api_get_timer.rs to return callback_type and callback_config instead of individual callback fields.

## Context

The Get Timer endpoint returns complete timer details including callback configuration. With the new schema, it needs to return callback_type and the full callback_config object. The callback_config will be serialized to JSON automatically by serde, showing the internally-tagged structure with "type" field.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/api_get_timer.rs` - Update response struct and handler

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/api_get_timer.rs`
2. Locate TimerDetailResponse struct
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
4. Update the get_timer handler function:
   - Query will already return Timer with new fields (from db.rs updates)
   - Map Timer fields to TimerDetailResponse:
     ```rust
     TimerDetailResponse {
         id: timer.id,
         created_at: timer.created_at,
         updated_at: timer.updated_at,
         execute_at: timer.execute_at,
         callback_type: timer.callback_type,
         callback_config: timer.callback_config,
         status: timer.status.to_string(),
         last_error: timer.last_error,
         executed_at: timer.executed_at,
         metadata: timer.metadata,
     }
     ```
5. Ensure serde will properly serialize CallbackConfig with tag
6. Run `cargo check` to ensure the endpoint compiles

## Acceptance Criteria

- [ ] TimerDetailResponse uses callback_type and callback_config fields
- [ ] Handler maps Timer to TimerDetailResponse correctly
- [ ] Response JSON includes callback_type as string ("http" or "nats")
- [ ] Response JSON includes callback_config with internally-tagged structure
- [ ] No references to old callback_url, callback_headers, callback_payload
- [ ] Code compiles without errors
- [ ] Response format matches CLAUDE.md specification

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "API Endpoints" → "Get Timer" section showing response format with callback_type and callback_config
