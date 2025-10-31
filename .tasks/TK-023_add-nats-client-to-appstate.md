# Task: Add NATS Client to AppState

**Status**: pending
**Dependencies**: TK-020_add-nats-config.md
**Estimated Effort**: small

## Objective

Add optional NATS client field to AppState struct to enable NATS callback execution across the application.

## Context

The AppState struct holds shared application state (database pool, config, timer cache). We need to add an optional NATS client that will be initialized if NATS_URL is configured. This client will be shared across all handlers and the scheduler for publishing NATS messages.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/models.rs` - Add nats_client field to AppState

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/models.rs`
2. Add import at the top of the file:
   ```rust
   use async_nats::Client as NatsClient;
   ```
3. Locate the AppState struct definition
4. Add new field to AppState:
   ```rust
   pub nats_client: Option<NatsClient>,
   ```
5. Ensure the field is properly documented with a comment:
   ```rust
   /// Optional NATS client for pub/sub callbacks (None if NATS_URL not configured)
   pub nats_client: Option<NatsClient>,
   ```
6. Run `cargo check` to ensure the change compiles
7. Note: The actual initialization of nats_client will happen in main.rs (TK-024)

## Acceptance Criteria

- [ ] AppState struct has nats_client field of type Option<NatsClient>
- [ ] async_nats::Client is imported
- [ ] Field is properly documented
- [ ] Code compiles without errors with `cargo check`
- [ ] No changes to existing AppState fields (pool, config, timer_cache)

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Rust Models" section showing AppState with nats_client field
