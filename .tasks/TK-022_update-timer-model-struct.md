# Task: Update Timer Struct with Callback Fields

**Status**: pending
**Dependencies**: TK-021_create-database-migration.md
**Estimated Effort**: small

## Objective

Replace callback_url, callback_headers, callback_payload fields in Timer struct with callback_type and callback_config.

## Context

After the database migration is complete, the Timer struct in models.rs must be updated to reflect the new schema. This change will affect how timers are queried, created, and serialized. The Timer struct is used throughout the codebase, but we'll update it carefully and fix compilation errors in subsequent tasks.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/models.rs` - Modify Timer struct

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/models.rs`
2. Locate the Timer struct definition
3. Remove these fields:
   ```rust
   pub callback_url: String,
   pub callback_headers: Option<HashMap<String, String>>,
   pub callback_payload: Option<Value>,
   ```
4. Add these new fields in their place:
   ```rust
   pub callback_type: CallbackType,
   pub callback_config: CallbackConfig,
   ```
5. Ensure the CallbackType and CallbackConfig types are already defined (from TK-019)
6. Update any sqlx::FromRow derives if necessary
7. Run `cargo check` to identify compilation errors (expect errors in db.rs and callback.rs - these will be fixed in later tasks)
8. Document the changes with inline comments explaining the new callback model

## Acceptance Criteria

- [ ] Timer struct no longer has callback_url, callback_headers, callback_payload fields
- [ ] Timer struct has callback_type and callback_config fields
- [ ] CallbackType is of enum type with Http and Nats variants
- [ ] CallbackConfig is of enum type with internally-tagged serialization
- [ ] Code compiles with warnings/errors (expected - will fix in subsequent tasks)
- [ ] Fields are properly documented

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Rust Models" section showing updated Timer struct definition
