# Task: Initialize NATS Client in Main

**Status**: pending
**Dependencies**: TK-023_add-nats-client-to-appstate.md
**Estimated Effort**: small

## Objective

Add NATS client initialization logic to main.rs, connecting to NATS server if NATS_URL is configured.

## Context

During application startup, after database connection is established, we need to conditionally connect to NATS if the NATS_URL environment variable is set. Connection failure should be treated as a fatal error (fail-fast principle). The NATS client is then stored in AppState for use by schedulers and handlers.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/main.rs` - Add NATS initialization before router setup

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/main.rs`
2. Add import at the top:
   ```rust
   use async_nats;
   ```
3. Locate the section after database pool initialization
4. Add NATS client initialization logic:
   ```rust
   // Initialize NATS client (optional)
   let nats_client = if let Some(nats_url) = &config.nats_url {
       tracing::info!("Connecting to NATS at {}", nats_url);
       match async_nats::connect(nats_url).await {
           Ok(client) => {
               tracing::info!("NATS connection established");
               Some(client)
           }
           Err(e) => {
               tracing::error!("Failed to connect to NATS: {}", e);
               panic!("NATS connection failed: {}", e);
           }
       }
   } else {
       tracing::info!("NATS_URL not configured, NATS callbacks disabled");
       None
   };
   ```
5. Update AppState initialization to include nats_client:
   ```rust
   let app_state = AppState {
       pool: pool.clone(),
       config: config.clone(),
       timer_cache: Arc::new(RwLock::new(HashMap::new())),
       nats_client,
   };
   ```
6. Ensure the nats_client is properly logged (connected or disabled)

## Acceptance Criteria

- [ ] NATS connection logic is added after database initialization
- [ ] If NATS_URL is set, application connects to NATS server
- [ ] If NATS connection fails, application panics with descriptive error
- [ ] If NATS_URL is not set, nats_client is None (no error)
- [ ] AppState is initialized with nats_client field
- [ ] Startup logs indicate NATS status (connected or disabled)
- [ ] Code compiles and runs without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Application Initialization" section describing NATS client setup
