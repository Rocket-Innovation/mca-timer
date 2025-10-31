# Task: Update List Timers API Endpoint

**Status**: pending
**Dependencies**: TK-028_update-database-queries.md, TK-019_update-models-callback-types.md
**Estimated Effort**: small

## Objective

Update api_list_timers.rs to include callback_type in TimerResponse and add optional callback_type filter.

## Context

The List Timers endpoint returns a simplified view of timers (without full callback_config for brevity). However, it should include callback_type to allow clients to see what type of callback each timer uses. Additionally, adding an optional callback_type query parameter allows filtering timers by callback type (http or nats).

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/api_list_timers.rs` - Update response struct, query params, and handler

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/api_list_timers.rs`
2. Locate TimerResponse struct (if defined here, or import from models.rs)
3. Add callback_type field to TimerResponse:
   ```rust
   pub struct TimerResponse {
       pub id: Uuid,
       pub created_at: DateTime<Utc>,
       pub execute_at: DateTime<Utc>,
       pub callback_type: CallbackType,  // Add this
       pub status: String,
       pub executed_at: Option<DateTime<Utc>>,
   }
   ```
4. Locate query parameters struct (e.g., ListTimersQuery)
5. Add optional callback_type filter:
   ```rust
   pub callback_type: Option<CallbackType>,
   ```
6. Update the list_timers handler:
   - Pass callback_type filter to db_list_timers if provided
   - Map Timer to TimerResponse including callback_type field
7. Update db_list_timers call in db.rs to support callback_type filtering:
   - Add WHERE clause: `AND callback_type = $X` if filter is Some
   - Build dynamic query with optional filter
8. Run `cargo check` to ensure the endpoint compiles

## Acceptance Criteria

- [ ] TimerResponse includes callback_type field
- [ ] Query parameters accept optional callback_type filter
- [ ] Handler passes callback_type filter to database query
- [ ] db_list_timers supports filtering by callback_type
- [ ] Response includes callback_type for each timer
- [ ] callback_config is NOT included in list response (too verbose)
- [ ] Filtering works correctly (returns only http or nats timers)
- [ ] Code compiles without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "API Endpoints" → "List Timers" section showing response format and query parameters
