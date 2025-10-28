# Task: Update Main Application Entry Point

**Status**: pending
**Dependencies**: TK-003, TK-004, TK-007, TK-008, TK-009
**Estimated Effort**: small

## Objective

Update main.rs to orchestrate the complete application startup sequence: load config, connect to database, run migrations, initialize cache, start scheduler, build router, and start HTTP server.

## Context

The `main.rs` file serves as the application entry point and follows the sequential startup process defined in the specification. It must initialize all components in the correct order, fail fast if any required component fails, and share state between the API and scheduler via Arc-wrapped AppState. The main function coordinates loading environment configuration, establishing database connections, running migrations, spawning scheduler tasks, and binding the HTTP server.

## Files to Modify/Create

- `src/main.rs` - Update existing main file with complete initialization

## Detailed Steps

1. Open `src/main.rs` and replace placeholder code
2. Add module declarations at top of file:
   ```rust
   mod config;
   mod models;
   mod db;
   mod api;
   mod scheduler;
   mod callback;
   ```
3. Import dependencies: `std::sync::Arc`, `std::collections::HashMap`, `tokio::sync::RwLock`, `sqlx::postgres::PgPoolOptions`, `axum::{Router, routing::{get, post, put, delete}, middleware}`, `tower_http::trace::TraceLayer`
4. In `main()` function, implement startup sequence:
   - Step 1: Initialize tracing (already exists, keep it)
   - Step 2: Load configuration: `let config = config::Config::from_env().expect("Failed to load configuration");`
   - Step 3: Log loaded configuration (without exposing secrets)
   - Step 4: Connect to database:
     ```rust
     let pool = PgPoolOptions::new()
         .max_connections(5)
         .connect(&config.database_url)
         .await
         .expect("Failed to connect to database");
     ```
   - Step 5: Run migrations: `sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");`
   - Step 6: Initialize in-memory cache: `let timer_cache = Arc::new(RwLock::new(HashMap::new()));`
   - Step 7: Start scheduler: `scheduler::start_scheduler(pool.clone(), timer_cache.clone());`
   - Step 8: Create shared AppState: `let state = Arc::new(AppState { pool, config: config.clone(), timer_cache });`
   - Step 9: Build router with routes:
     - `POST /timers` → `api::create_timer_handler`
     - `GET /timers/:id` → `api::get_timer_handler`
     - `GET /timers` → `api::list_timers_handler`
     - `PUT /timers/:id` → `api::update_timer_handler`
     - `DELETE /timers/:id` → `api::cancel_timer_handler`
     - `GET /health` → `api::health_check_handler` (no auth)
   - Step 10: Apply middleware:
     - Add `auth_middleware` to all routes except `/health`
     - Add `TraceLayer` for request logging to all routes
   - Step 11: Bind server to `0.0.0.0:{config.port}`
   - Step 12: Log "Server listening on {addr}"
   - Step 13: Start serving with `axum::serve(listener, app).await`
5. Use nested routers to apply auth selectively:
   ```rust
   let protected_routes = Router::new()
       .route("/timers", post(api::create_timer_handler))
       .route("/timers", get(api::list_timers_handler))
       .route("/timers/:id", get(api::get_timer_handler))
       .route("/timers/:id", put(api::update_timer_handler))
       .route("/timers/:id", delete(api::cancel_timer_handler))
       .layer(middleware::from_fn_with_state(state.clone(), api::auth_middleware));

   let app = Router::new()
       .merge(protected_routes)
       .route("/health", get(api::health_check_handler))
       .layer(TraceLayer::new_for_http())
       .with_state(state);
   ```
6. Add panic handlers with descriptive messages for each startup step

## Acceptance Criteria

- [ ] All module declarations present at top of file
- [ ] Configuration loaded and validated before database connection
- [ ] Database connection pool created with max 5 connections
- [ ] Migrations run automatically on startup
- [ ] In-memory cache initialized as empty HashMap
- [ ] Scheduler tasks spawned before HTTP server starts
- [ ] AppState created with pool, config, and cache
- [ ] All six API routes registered with correct HTTP methods
- [ ] Auth middleware applied to all routes except /health
- [ ] TraceLayer applied for request logging
- [ ] Server binds to configured port
- [ ] Startup sequence fails fast on any error
- [ ] Application logs key startup milestones

## Reference

See CLAUDE.md - "Application Initialization" section for complete startup sequence, and "Main application entry point (main.rs)" in project structure for coordination responsibilities.
