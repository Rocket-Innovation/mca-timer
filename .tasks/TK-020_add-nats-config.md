# Task: Add NATS Configuration to Config Module

**Status**: pending
**Dependencies**: TK-018_add-nats-dependency.md
**Estimated Effort**: small

## Objective

Add optional NATS_URL configuration parameter to config.rs to enable NATS client initialization.

## Context

The application needs to connect to a NATS server if NATS callbacks are used. NATS connection is optional - if NATS_URL is not provided, the application will only support HTTP callbacks. This follows the MVP principle of keeping things simple while enabling NATS functionality when needed.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/config.rs` - Add nats_url field to Config struct

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/config.rs`
2. Add new field to Config struct:
   ```rust
   pub nats_url: Option<String>,
   ```
3. Update the `from_env()` or similar initialization method to read NATS_URL:
   ```rust
   nats_url: std::env::var("NATS_URL").ok(),
   ```
4. Add validation logic (if Config has a validate method):
   - If nats_url is Some, verify it's a valid URL format
   - If nats_url is None, that's acceptable (NATS is optional)
5. Ensure the field is properly documented with comments explaining it's optional

## Acceptance Criteria

- [ ] Config struct has nats_url field of type Option<String>
- [ ] NATS_URL environment variable is read during config initialization
- [ ] If NATS_URL is not set, nats_url is None (not an error)
- [ ] If NATS_URL is set, it's stored in config.nats_url as Some(url)
- [ ] Code compiles without errors with `cargo check`
- [ ] No breaking changes to existing configuration fields

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Environment Variables" section, optional NATS_URL parameter
