# Task: Update Database Queries for New Schema

**Status**: pending
**Dependencies**: TK-022_update-timer-model-struct.md, TK-021_create-database-migration.md
**Estimated Effort**: medium

## Objective

Update all database queries in db.rs to use callback_type and callback_config instead of callback_url, callback_headers, callback_payload.

## Context

After the database schema migration, all SQLx queries must be updated to select and insert the new callback fields. The callback_config is stored as JSONB, so we need to ensure proper serialization/deserialization. SQLx will handle the conversion between Rust CallbackConfig enum and PostgreSQL JSONB automatically with proper derives.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/db.rs` - Update all queries with new field names

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/db.rs`
2. Find all SELECT queries that include callback fields
3. Replace:
   - `callback_url` → `callback_type, callback_config`
   - Remove `callback_headers, callback_payload`
4. Update db_create_timer function:
   - Change parameter from individual fields to `callback_type: CallbackType, callback_config: CallbackConfig`
   - Update INSERT query to use new fields:
     ```sql
     INSERT INTO timers (id, created_at, updated_at, execute_at, callback_type, callback_config, status, metadata)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
     ```
   - Use sqlx::types::Json wrapper for callback_config: `sqlx::types::Json(&callback_config)`
5. Update db_get_timer_by_id function:
   - Modify SELECT query to fetch callback_type and callback_config
   - Ensure Timer struct mapping works with new fields
6. Update db_list_timers function:
   - Modify SELECT query to include callback_type and callback_config
   - Maintain pagination and filtering logic
7. Update db_update_timer function:
   - Accept optional callback_type and callback_config parameters
   - Build dynamic UPDATE query to include new fields when provided
   - Use sqlx::types::Json for callback_config serialization
8. Update db_load_near_term_timers function (for scheduler):
   - Ensure SELECT includes callback_type and callback_config
   - No filtering changes needed, just field selection
9. Test that all queries compile with `cargo check`
10. Note: sqlx::query macros will need database connection for compile-time checking

## Acceptance Criteria

- [ ] All SELECT queries include callback_type and callback_config
- [ ] db_create_timer accepts CallbackType and CallbackConfig parameters
- [ ] db_update_timer supports optional callback_type and callback_config updates
- [ ] callback_config is properly serialized/deserialized as JSONB
- [ ] No references to old callback_url, callback_headers, callback_payload remain
- [ ] All database functions compile without errors
- [ ] sqlx compile-time checks pass (requires running database)

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Database Schema" section showing new field structure and JSONB storage format
