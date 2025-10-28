# Task: Implement Database Operations Module

**Status**: pending
**Dependencies**: TK-004
**Estimated Effort**: medium

## Objective

Create the database operations module with all SQLx queries for CRUD operations on timers.

## Context

The `db.rs` module encapsulates all database interactions using SQLx for type-safe, compile-time checked queries. Each function is prefixed with `db_` by convention and takes `&PgPool` as the first parameter. Functions return `Result<T, sqlx::Error>` for error handling. All queries must handle JSONB serialization/deserialization for headers, payload, and metadata fields. The module focuses on the six core operations: create, get by ID, list with filters, update, cancel (soft delete), and load near-term timers for scheduler.

## Files to Modify/Create

- `src/db.rs` - New database operations module

## Detailed Steps

1. Create `src/db.rs` file
2. Import dependencies: `sqlx::{PgPool, FromRow}`, `uuid::Uuid`, `chrono::{DateTime, Utc}`, models types
3. Implement `db_create_timer()`:
   - Takes: pool, CreateTimerRequest
   - Generates new UUIDv4 using `Uuid::new_v4()`
   - Uses INSERT query with RETURNING clause
   - Serializes callback_headers, callback_payload, metadata to JSONB
   - Returns `Result<Timer, sqlx::Error>`
4. Implement `db_get_timer()`:
   - Takes: pool, timer_id (Uuid)
   - Uses SELECT WHERE id = $1
   - Deserializes JSONB fields back to HashMap/Value
   - Returns `Result<Option<Timer>, sqlx::Error>`
5. Implement `db_list_timers()`:
   - Takes: pool, status_filter (Option<String>), limit, offset, sort_field, sort_order
   - Builds dynamic WHERE clause if status filter provided
   - Adds ORDER BY with sort_field and sort_order
   - Uses LIMIT and OFFSET for pagination
   - Also runs COUNT query for total
   - Returns `Result<(Vec<Timer>, i64), sqlx::Error>` (timers and total count)
6. Implement `db_update_timer()`:
   - Takes: pool, timer_id, UpdateTimerRequest
   - Builds dynamic UPDATE query only for provided fields (non-None Options)
   - Includes `updated_at = NOW()` automatically
   - Returns `Result<Timer, sqlx::Error>`
7. Implement `db_cancel_timer()`:
   - Takes: pool, timer_id
   - Updates status to 'canceled' and updated_at to NOW()
   - Returns `Result<Timer, sqlx::Error>`
8. Implement `db_load_near_term_timers()`:
   - Takes: pool
   - Queries timers with: `status = 'pending' AND execute_at > NOW() - INTERVAL '5 minutes' AND execute_at <= NOW() + INTERVAL '1 minute'`
   - This loads overdue timers (up to 5 min past) and future timers (up to 1 min ahead)
   - Returns `Result<Vec<Timer>, sqlx::Error>`
9. Implement `db_mark_executing()`:
   - Takes: pool, timer_id
   - Updates status to 'executing' and updated_at to NOW()
   - Returns `Result<(), sqlx::Error>`
10. Implement `db_mark_completed()`:
    - Takes: pool, timer_id
    - Updates status to 'completed', executed_at to NOW()
    - Returns `Result<(), sqlx::Error>`
11. Implement `db_mark_failed()`:
    - Takes: pool, timer_id, error_message
    - Updates status to 'failed', last_error to error_message, executed_at to NOW()
    - Returns `Result<(), sqlx::Error>`

## Acceptance Criteria

- [ ] All 11 database functions implemented with correct signatures
- [ ] JSONB serialization/deserialization handles HashMap and serde_json::Value
- [ ] `db_load_near_term_timers()` query window is exactly: NOW() - 5min to NOW() + 1min
- [ ] `db_list_timers()` supports dynamic filtering, sorting, and pagination
- [ ] `db_update_timer()` only updates provided fields (partial updates)
- [ ] All functions return proper Result types for error handling
- [ ] Functions are prefixed with `db_` following naming convention
- [ ] No SQL injection vulnerabilities (use parameterized queries)

## Reference

See CLAUDE.md - "Database operations (SQLx queries)" references in db.rs description, and "System Architecture" for scheduler query requirements including the near-term window definition.
