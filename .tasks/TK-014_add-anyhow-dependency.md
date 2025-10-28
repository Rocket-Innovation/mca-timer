# Task: Add Missing Dependencies to Cargo.toml

**Status**: pending
**Dependencies**: None
**Estimated Effort**: small

## Objective

Add any missing dependencies to Cargo.toml that are required by the implementation but not currently listed.

## Context

While reviewing the task implementations, several modules reference dependencies that aren't explicitly listed in the current Cargo.toml. Specifically, `anyhow` is used for error handling in config.rs and callback.rs, and we should verify all necessary features are enabled for existing dependencies. This task ensures the project can compile by adding any missing crate dependencies.

## Files to Modify/Create

- `Cargo.toml` - Add missing dependencies

## Detailed Steps

1. Open `Cargo.toml` at `/Users/tar/Documents/alpha/timer/Cargo.toml`
2. Review existing dependencies to confirm they have correct features:
   - `axum` should have default features
   - `tokio` already has `features = ["full"]` ✓
   - `sqlx` already has required features ✓
   - `uuid` already has v4 and serde ✓
   - `reqwest` already has json feature ✓
3. Add `anyhow` crate for error handling:
   - Add to `[dependencies]` section
   - Version: `"1.0"`
   - Used in config.rs and callback.rs for Result types
4. Verify `tower` and `tower-http` have necessary features:
   - `tower-http` needs `["trace", "cors"]` (already present ✓)
5. Consider adding `thiserror` if custom error types are needed (optional for MVP)

## Acceptance Criteria

- [ ] `anyhow = "1.0"` added to dependencies
- [ ] All existing dependencies have correct feature flags
- [ ] Cargo.toml can be parsed without errors
- [ ] Running `cargo check` shows no missing dependency errors
- [ ] Dependencies are organized logically with comments

## Reference

See CLAUDE.md - "Dependencies (Cargo.toml)" section for the complete dependency list, though note that anyhow is implied by the error handling patterns described throughout the specification.
