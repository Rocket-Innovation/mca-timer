# Task: Implement Scheduler Module with Memory Loader and Execution Tasks

**Status**: pending
**Dependencies**: TK-004, TK-005, TK-006
**Estimated Effort**: medium

## Objective

Create the scheduler module with two concurrent background tasks: Memory Loader (loads near-term timers every 30s) and Execution Task (checks cache every 1s and executes due timers).

## Context

The `scheduler.rs` module implements the hybrid storage architecture with in-memory caching. It spawns two independent Tokio tasks that run in infinite loops. The Memory Loader task queries PostgreSQL every 30 seconds for timers in the near-term window (NOW() - 5 minutes to NOW() + 1 minute) and replaces the entire cache. The Execution Task scans the cache every 1 second for timers where `execute_at <= NOW()`, marks them as executing in the database, removes them from cache, and spawns async tasks to execute callbacks. This architecture reduces database load by 97% while maintaining sub-second execution precision.

## Files to Modify/Create

- `src/scheduler.rs` - New scheduler module

## Detailed Steps

1. Create `src/scheduler.rs` file
2. Import dependencies: `tokio::time::{interval, Duration}`, `std::sync::Arc`, `tokio::sync::RwLock`, `uuid::Uuid`, `chrono::Utc`, `sqlx::PgPool`, models, db, callback
3. Implement `start_scheduler()` function:
   - Takes: pool (PgPool), cache (TimerCache)
   - Clones pool and cache for each task (Arc allows shared ownership)
   - Spawns Memory Loader task with `tokio::spawn()`
   - Spawns Execution Task with `tokio::spawn()`
   - Returns immediately (tasks run in background)
   - Signature: `pub fn start_scheduler(pool: PgPool, cache: TimerCache)`
4. Implement Memory Loader task:
   - Creates interval timer: `interval(Duration::from_secs(30))`
   - Infinite loop with `loop { interval.tick().await; ... }`
   - Calls `db_load_near_term_timers(&pool)` to fetch timers
   - Acquires write lock: `let mut cache_guard = cache.write().await`
   - Clears entire cache: `cache_guard.clear()`
   - Inserts all fetched timers: `for timer in timers { cache_guard.insert(timer.id, timer); }`
   - Releases lock (automatic when guard drops)
   - Logs number of timers loaded
   - On error: log warning and continue (don't crash, retry next interval)
5. Implement Execution Task:
   - Creates interval timer: `interval(Duration::from_secs(1))`
   - Infinite loop with `loop { interval.tick().await; ... }`
   - Gets current time: `let now = Utc::now()`
   - Acquires read lock: `let cache_guard = cache.read().await`
   - Collects due timers: `let due_timers: Vec<Timer> = cache_guard.values().filter(|t| t.execute_at <= now).cloned().collect()`
   - Releases read lock (before spawning tasks)
   - For each due timer:
     - Call `db_mark_executing(&pool, timer.id)` to update status in DB
     - Acquire write lock and remove from cache: `cache.write().await.remove(&timer.id)`
     - Spawn async task: `tokio::spawn(async move { callback::execute_callback(pool.clone(), timer).await })`
   - Log number of timers executed
   - On error: log warning and continue (individual timer failures don't stop scheduler)
6. Add tracing/logging statements:
   - Memory Loader: "Loaded {} timers into cache"
   - Execution Task: "Executing {} due timers"
   - Individual timer execution: "Spawned callback for timer {}"

## Acceptance Criteria

- [ ] `start_scheduler()` function spawns two independent Tokio tasks
- [ ] Memory Loader runs every 30 seconds exactly
- [ ] Memory Loader replaces entire cache (clear + insert all) on each run
- [ ] Memory Loader loads timers in window: NOW() - 5 minutes to NOW() + 1 minute
- [ ] Execution Task runs every 1 second exactly
- [ ] Execution Task only processes timers where `execute_at <= NOW()`
- [ ] Timers are removed from cache immediately after spawning callback task
- [ ] Each callback executes in its own async task (non-blocking)
- [ ] Errors in one timer don't affect other timers or scheduler operation
- [ ] Both tasks log their activity for observability
- [ ] Read locks released before spawning callback tasks (no blocking)
- [ ] Write lock only held briefly when modifying cache

## Reference

See CLAUDE.md - "Timer Scheduler" section for hybrid storage architecture explanation, "Scheduler Flow Diagram" for complete task flow, and "Scheduler Implementation" for detailed implementation notes on concurrency and locking.
