# Task: Update Scheduler Module with NATS Client

**Status**: pending
**Dependencies**: TK-027_update-callback-dispatcher.md, TK-028_update-database-queries.md
**Estimated Effort**: small

## Objective

Update scheduler.rs to pass the NATS client to the callback execution function.

## Context

The scheduler module spawns callback execution tasks. With NATS support, the execute_callback function now requires an optional NATS client reference. The scheduler needs to extract the nats_client from AppState and pass it along when spawning callback tasks. No other logic changes are needed.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/scheduler.rs` - Update callback invocation to include nats_client

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/scheduler.rs`
2. Locate the execution task that spawns callback tasks
3. Find where `execute_callback` is called (likely inside a tokio::spawn)
4. Update the function call to include nats_client from AppState:
   ```rust
   let nats_client = app_state.nats_client.as_ref();
   tokio::spawn(async move {
       execute_callback(timer, &pool, nats_client).await;
   });
   ```
5. Ensure app_state is accessible in the execution task scope
6. If needed, clone the Arc reference to nats_client to move into the spawned task
7. Verify no other changes are needed (Timer struct already has callback_type and callback_config)
8. Run `cargo check` to ensure the module compiles

## Acceptance Criteria

- [ ] Scheduler passes nats_client to execute_callback function
- [ ] nats_client is extracted from AppState as Option<&NatsClient>
- [ ] Proper ownership and lifetime management for spawned tasks
- [ ] No changes to memory loader or execution interval logic
- [ ] Code compiles without errors
- [ ] Scheduler continues to work for both HTTP and NATS callbacks

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Scheduler Implementation" section describing callback task spawning
