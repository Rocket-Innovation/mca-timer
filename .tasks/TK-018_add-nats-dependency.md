# Task: Add async-nats Dependency

**Status**: pending
**Dependencies**: None
**Estimated Effort**: small

## Objective

Add the async-nats crate to Cargo.toml to enable NATS pub/sub functionality for callback delivery.

## Context

The Timer Platform is expanding callback support from HTTP-only to dual mode (HTTP or NATS). NATS is a lightweight pub/sub messaging system that enables fire-and-forget message delivery. We need to add the async-nats dependency as the foundation for NATS callback implementation.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/Cargo.toml` - Add async-nats dependency

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/Cargo.toml`
2. Locate the `[dependencies]` section
3. Add the following line after the reqwest dependency:
   ```toml
   async-nats = "0.33"
   ```
4. Run `cargo check` to ensure the dependency resolves correctly
5. Run `cargo update` to update Cargo.lock with the new dependency

## Acceptance Criteria

- [ ] async-nats 0.33 is added to Cargo.toml dependencies
- [ ] `cargo check` completes successfully without errors
- [ ] Cargo.lock is updated with async-nats and its dependencies
- [ ] No existing dependencies are modified or removed

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Tech Stack" section specifying `async-nats 0.33`
