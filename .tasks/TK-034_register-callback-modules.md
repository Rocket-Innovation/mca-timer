# Task: Register New Callback Modules in Main

**Status**: pending
**Dependencies**: TK-025_create-http-callback-module.md, TK-026_create-nats-callback-module.md
**Estimated Effort**: small

## Objective

Add module declarations for callback_http and callback_nats in main.rs so they are compiled and available.

## Context

Rust requires explicit module declarations in main.rs (or lib.rs) for the compiler to include source files. We've created callback_http.rs and callback_nats.rs, but they won't be compiled unless declared as modules. The callback.rs module already exists and imports these new modules.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/main.rs` - Add module declarations

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/main.rs`
2. Locate the section where modules are declared (typically at the top after use statements)
3. Add the following module declarations after `mod callback;`:
   ```rust
   mod callback_http;
   mod callback_nats;
   ```
4. Ensure these declarations are in the same section as other mod declarations (config, models, db, etc.)
5. The order doesn't matter, but grouping callback-related modules together is cleaner
6. Run `cargo check` to ensure modules compile and are properly linked

## Acceptance Criteria

- [ ] callback_http module is declared in main.rs
- [ ] callback_nats module is declared in main.rs
- [ ] Module declarations are in the appropriate location (with other mod declarations)
- [ ] Code compiles without errors
- [ ] No "unused module" warnings

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Project Structure" section showing all module files including callback_http.rs and callback_nats.rs
