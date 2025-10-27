# Timer/Callback Scheduling Platform

## Responsibilities

- **Timer Registration**: Accept and validate timer creation requests via RESTful API
- **Timer Storage**: Persist timer configurations, callback URLs, and payloads in PostgreSQL
- **Timer Scheduling**: Monitor registered timers and trigger callbacks at specified execution times
- **Callback Delivery**: Execute HTTP callbacks to external platforms with payload data (single attempt)
- **Timer Management**: Support listing, updating, canceling, and querying timer status

This service does NOT:
- Guarantee exact-millisecond precision (timers checked every 1 second)
- Provide instant cache synchronization (eventual consistency with up to 30-second delay)
- Store callback response data (only tracks success/failure)
- Provide authentication/authorization (MVP uses simple API key)
- Support recurring/periodic timers (MVP is one-shot timers only)
- Retry failed callbacks (one attempt only, fails immediately on error)

## Tech Stack

- **Rust** (edition 2024) - Core language for performance and safety
- **Axum** 0.7 - Modern web framework for RESTful API
- **PostgreSQL** 15+ - Persistent storage for timer configurations
- **SQLx** 0.7 - Type-safe database interactions with compile-time query checking
- **Tokio** 1.x - Async runtime (full feature set) for non-blocking I/O
- **Reqwest** 0.11 - HTTP client for triggering callbacks
- **Chrono** 0.4 - Date/time handling with timezone support
- **UUID** 1.0 - UUIDv4 for globally unique timer IDs
- **Tracing** - Structured logging for observability
- **Docker** - Containerization for local development

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Timer Platform                            │
│                                                                   │
│  ┌──────────────┐         ┌──────────────┐                      │
│  │   REST API   │         │  Scheduler   │                      │
│  │   (Axum)     │         │   (Tokio)    │                      │
│  │              │         │              │                      │
│  │ - Create     │         │ ┌──────────┐ │                      │
│  │ - Get        │         │ │  Memory  │ │                      │
│  │ - List       │         │ │  Loader  │ │                      │
│  │ - Update     │         │ │  (30s)   │◄┼──┐                   │
│  │ - Cancel     │         │ └────┬─────┘ │  │                   │
│  │ - Health     │         │      │       │  │                   │
│  └──────┬───────┘         │ ┌────▼─────┐ │  │                   │
│         │                 │ │Execution │ │  │                   │
│         │                 │ │  Task    │ │  │                   │
│         │                 │ │  (1s)    │ │  │                   │
│         │                 │ └────┬─────┘ │  │                   │
│         │                 └──────┼───────┘  │                   │
│         │                        │          │                   │
│         │                        ▼          │                   │
│         │                 ┌─────────────┐   │                   │
│         │                 │  In-Memory  │   │                   │
│         │                 │    Cache    │◄──┘                   │
│         │                 │             │  read from DB         │
│         │                 │ Near-term:  │  every 30s            │
│         │                 │ <= NOW()+5m │                       │
│         │                 └─────────────┘                       │
│         │                                                        │
│         │   API writes directly to PostgreSQL only              │
│         │   (no cache interaction, eventual consistency)        │
│         │                                                        │
│         ▼                                                        │
│  ┌─────────────────────────────────────┐                        │
│  │         PostgreSQL                   │                        │
│  │      (Source of Truth)               │                        │
│  │                                      │                        │
│  │  - All timers (persistent)           │                        │
│  │  - Indexed for scheduler queries     │                        │
│  │  - CRUD operations go here first     │                        │
│  └─────────────────────────────────────┘                        │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
                          │
                          │ HTTP Callbacks
                          ▼
                ┌──────────────────┐
                │ External Services │
                │  (Webhooks)       │
                └──────────────────┘
```

## Coding Styles

**Philosophy**: Lean and Simple
- Write straightforward Rust with minimal abstractions
- Prefer explicit error handling over complex Result wrappers
- Keep modules flat, avoid deep nesting
- Use clear, self-documenting names
- Trust internal components (no redundant validation)
- Fail fast with descriptive errors

**Project Structure**: Flat organization
```
src/
├── main.rs              // Application entry point, server setup
├── api.rs               // API route handlers
├── models.rs            // Data structures and types
├── db.rs                // Database operations (SQLx queries)
├── scheduler.rs         // Timer polling and execution logic
├── callback.rs          // HTTP callback execution
└── config.rs            // Environment configuration
```

**Conventions**:
- Use `snake_case` for variables, functions, modules
- Use `PascalCase` for types, structs, enums
- Prefix database operations with `db_` (e.g., `db_create_timer`)
- Use `Result<T, AppError>` for fallible operations
- Document complex business logic with inline comments
- Keep functions under 50 lines where possible

## Data Models

### Database Schema (PostgreSQL)

**Table: `timers`**
```sql
CREATE TABLE timers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    execute_at TIMESTAMPTZ NOT NULL,
    callback_url TEXT NOT NULL,
    callback_method VARCHAR(10) NOT NULL DEFAULT 'POST',
    callback_headers JSONB,
    callback_payload JSONB,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    last_error TEXT,
    executed_at TIMESTAMPTZ,
    metadata JSONB
);

CREATE INDEX idx_timers_execute_at_status ON timers(execute_at, status)
    WHERE status = 'pending';

CREATE INDEX idx_timers_status ON timers(status);
CREATE INDEX idx_timers_created_at ON timers(created_at DESC);
```

**Field Descriptions**:
- `id`: UUIDv4 primary key, auto-generated
- `created_at`: Timer creation timestamp (UTC)
- `updated_at`: Last modification timestamp (UTC)
- `execute_at`: When callback should be triggered (UTC)
- `callback_url`: Full HTTP(S) URL for callback destination
- `callback_method`: HTTP method (POST, PUT, PATCH supported)
- `callback_headers`: Optional JSON object of custom headers
- `callback_payload`: JSON payload to send in callback body
- `status`: Current state - `pending`, `executing`, `completed`, `failed`, `canceled`
- `last_error`: Error message if callback failed
- `executed_at`: Timestamp of successful callback execution
- `metadata`: Optional JSON object for client reference data

**Indexes**:
- Composite index on `(execute_at, status)` for efficient scheduler queries
- Status index for filtering timers by state
- Created_at index for listing recent timers

### Rust Models

**Timer Status Enum**:
```rust
pub enum TimerStatus {
    Pending,      // Waiting for execute_at time
    Executing,    // Currently attempting callback
    Completed,    // Successfully executed
    Failed,       // Callback failed (no retry)
    Canceled,     // Manually canceled by user
}
```

**API Request/Response Types**:
```rust
pub struct CreateTimerRequest {
    pub execute_at: DateTime<Utc>,
    pub callback_url: String,
    pub callback_method: Option<String>,
    pub callback_headers: Option<HashMap<String, String>>,
    pub callback_payload: Option<Value>,
    pub metadata: Option<Value>,
}

pub struct TimerResponse {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub execute_at: DateTime<Utc>,
    pub callback_url: String,
    pub callback_method: String,
    pub status: String,
    pub executed_at: Option<DateTime<Utc>>,
}

pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    pub timer_cache: TimerCache,
}

pub type TimerCache = Arc<RwLock<HashMap<Uuid, Timer>>>;
```

## Response Format

### HTTP Status Codes
- **200** - Success (GET, PUT operations)
- **201** - Created (POST operations)
- **400** - Bad request (invalid input, validation errors)
- **401** - Unauthorized (missing/invalid API key)
- **404** - Timer not found
- **500** - Internal server error

### Response Body Codes
- **0** - Success
- **1** - Unexpected error (database, network, internal)
- **2** - Validation error (invalid input)
- **3** - Not found
- **4** - Unauthorized

**Example Success Response**:
```json
{
    "code": 0,
    "message": "success",
    "data": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "created_at": "2025-10-26T10:30:00Z",
        "execute_at": "2025-10-26T12:00:00Z",
        "callback_url": "https://api.example.com/webhook",
        "callback_method": "POST",
        "status": "pending",
        "executed_at": null
    }
}
```

**Example Error Response**:
```json
{
    "code": 2,
    "message": "execute_at must be in the future",
    "data": null
}
```

## Authentication

**MVP Authentication**: Simple API Key

- All API endpoints require `X-API-Key` header
- API key configured via `API_KEY` environment variable
- Single shared key for all clients (no per-client keys in MVP)
- No rate limiting or request tracking in MVP

**Header Format**:
```
X-API-Key: your-secret-api-key-here
```

**Validation**:
- Check header presence on every request
- Return 401 with code 4 if missing or incorrect
- Use Axum middleware for authentication layer

**Example Middleware**:
```rust
async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = headers.get("X-API-Key")
        .and_then(|v| v.to_str().ok());

    if api_key != Some(&CONFIG.api_key) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
```

## Environment Variables

**Required**:
- `DATABASE_URL` - PostgreSQL connection string (format: `postgresql://user:pass@host:port/dbname`)
- `API_KEY` - Authentication key for API access (minimum 32 characters recommended)

**Optional**:
- `PORT` - HTTP server port (default: `3000`)
- `RUST_LOG` - Logging level (default: `info`)

**Example `.env` file**:
```env
DATABASE_URL=postgresql://timer:timer123@localhost:5432/timerdb
API_KEY=my-super-secret-api-key-at-least-32-chars-long
PORT=3000
RUST_LOG=info
```

**Hard-coded defaults** (not configurable in MVP):
- Scheduler check interval: 1 second
- Memory loader interval: 30 seconds
- Near-term window: 5 minutes
- Callback timeout: 30 seconds

## API Endpoints

### Create Timer
```
POST /timers
```

**Description**: Register a new timer with callback configuration

**Headers**:
```
X-API-Key: your-api-key
Content-Type: application/json
```

**Request Body**:
```json
{
    "execute_at": "2025-10-26T15:30:00Z",
    "callback_url": "https://api.example.com/webhook",
    "callback_method": "POST",
    "callback_headers": {
        "Authorization": "Bearer token123",
        "X-Custom-Header": "value"
    },
    "callback_payload": {
        "event": "timer_triggered",
        "user_id": "user123"
    },
    "metadata": {
        "client_ref": "order-456"
    }
}
```

**Field Validations**:
- `execute_at` (required): Must be ISO 8601 format, must be in the future
- `callback_url` (required): Must be valid HTTP/HTTPS URL
- `callback_method` (optional): Must be POST, PUT, or PATCH (default: POST)
- `callback_headers` (optional): JSON object, keys must be valid HTTP header names
- `callback_payload` (optional): Any valid JSON value
- `metadata` (optional): Any valid JSON value for client reference

**Success Response** (HTTP 201):
```json
{
    "code": 0,
    "message": "timer created successfully",
    "data": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "created_at": "2025-10-26T10:30:00Z",
        "execute_at": "2025-10-26T15:30:00Z",
        "callback_url": "https://api.example.com/webhook",
        "callback_method": "POST",
        "status": "pending",
        "executed_at": null
    }
}
```

**Error Response** (HTTP 400):
```json
{
    "code": 2,
    "message": "execute_at must be in the future",
    "data": null
}
```

**Behavior**:
1. Validate authentication via API key
2. Parse and validate request body
3. Check that `execute_at` is in the future (> now + 5 seconds buffer)
4. Validate callback_url is well-formed HTTP/HTTPS URL
5. Validate callback_method is one of: POST, PUT, PATCH
6. Generate UUIDv4 for timer ID
7. Insert record into `timers` table with status='pending'
8. Return created timer details

**Notes**:
- Timer IDs are auto-generated UUIDv4
- Minimum execution delay: 5 seconds from creation
- Maximum execution delay: No limit (MVP)
- callback_headers stored as JSONB for flexibility
- **Eventual Consistency**: Timer is persisted to PostgreSQL immediately, but appears in scheduler's in-memory cache within 30 seconds (next memory loader sync). For timers scheduled to execute within 30 seconds, there may be a delay of up to 30 seconds before execution starts. This is an acceptable trade-off for simpler design.

---

### Get Timer
```
GET /timers/:id
```

**Description**: Retrieve details of a specific timer by ID

**Headers**:
```
X-API-Key: your-api-key
```

**Path Parameters**:
- `id`: UUID of the timer

**Success Response** (HTTP 200):
```json
{
    "code": 0,
    "message": "success",
    "data": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "created_at": "2025-10-26T10:30:00Z",
        "updated_at": "2025-10-26T10:30:00Z",
        "execute_at": "2025-10-26T15:30:00Z",
        "callback_url": "https://api.example.com/webhook",
        "callback_method": "POST",
        "callback_headers": {
            "Authorization": "Bearer token123"
        },
        "callback_payload": {
            "event": "timer_triggered"
        },
        "status": "pending",
        "last_error": null,
        "executed_at": null,
        "metadata": {
            "client_ref": "order-456"
        }
    }
}
```

**Error Response** (HTTP 404):
```json
{
    "code": 3,
    "message": "timer not found",
    "data": null
}
```

**Behavior**:
1. Validate authentication
2. Parse UUID from path parameter
3. Query database for timer by ID
4. Return 404 if not found
5. Return full timer details including headers, payload, metadata

**Notes**:
- Returns complete timer configuration including sensitive data (callback headers)
- No filtering or redaction of fields in MVP

---

### List Timers
```
GET /timers
```

**Description**: List timers with optional filtering and pagination

**Headers**:
```
X-API-Key: your-api-key
```

**Query Parameters**:
- `status` (optional): Filter by status (pending, executing, completed, failed, canceled)
- `limit` (optional): Number of results (default: 50, max: 200)
- `offset` (optional): Pagination offset (default: 0)
- `sort` (optional): Sort field (created_at, execute_at, default: created_at)
- `order` (optional): Sort order (asc, desc, default: desc)

**Example Request**:
```
GET /timers?status=pending&limit=20&offset=0&sort=execute_at&order=asc
```

**Success Response** (HTTP 200):
```json
{
    "code": 0,
    "message": "success",
    "data": {
        "timers": [
            {
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "created_at": "2025-10-26T10:30:00Z",
                "execute_at": "2025-10-26T15:30:00Z",
                "callback_url": "https://api.example.com/webhook",
                "callback_method": "POST",
                "status": "pending",
                "executed_at": null
            }
        ],
        "total": 1,
        "limit": 20,
        "offset": 0
    }
}
```

**Behavior**:
1. Validate authentication
2. Parse and validate query parameters
3. Build SQL query with filters, sorting, pagination
4. Execute query with LIMIT/OFFSET
5. Execute COUNT query for total matching records
6. Return list with pagination metadata

**Notes**:
- Default sort: most recent first (created_at DESC)
- Does not include full timer details (callback_headers, callback_payload, metadata excluded)
- Total count may be cached/estimated for performance (MVP uses exact COUNT)

---

### Update Timer
```
PUT /timers/:id
```

**Description**: Update a pending timer's configuration

**Headers**:
```
X-API-Key: your-api-key
Content-Type: application/json
```

**Path Parameters**:
- `id`: UUID of the timer

**Request Body** (all fields optional):
```json
{
    "execute_at": "2025-10-26T16:00:00Z",
    "callback_url": "https://api.example.com/new-webhook",
    "callback_method": "PUT",
    "callback_headers": {
        "Authorization": "Bearer newtoken"
    },
    "callback_payload": {
        "event": "updated_event"
    },
    "metadata": {
        "updated": true
    }
}
```

**Success Response** (HTTP 200):
```json
{
    "code": 0,
    "message": "timer updated successfully",
    "data": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "created_at": "2025-10-26T10:30:00Z",
        "updated_at": "2025-10-26T11:00:00Z",
        "execute_at": "2025-10-26T16:00:00Z",
        "callback_url": "https://api.example.com/new-webhook",
        "callback_method": "PUT",
        "status": "pending",
        "executed_at": null
    }
}
```

**Error Response** (HTTP 400):
```json
{
    "code": 2,
    "message": "cannot update timer with status 'completed'",
    "data": null
}
```

**Behavior**:
1. Validate authentication
2. Parse UUID and request body
3. Query database to check timer exists and status
4. Reject updates for timers with status: completed, failed, canceled
5. Validate new field values (same rules as create)
6. If execute_at updated and in past, reject
7. Update only provided fields, leave others unchanged
8. Set updated_at to current timestamp
9. Return updated timer details

**Notes**:
- Only pending timers can be updated
- Partial updates supported (only include fields to change)
- Cannot change timer ID or created_at
- **Eventual Consistency**: Updates are persisted to PostgreSQL immediately, but scheduler's in-memory cache refreshes within 30 seconds. If timer is already cached and scheduled for near-term execution, the old version may execute before cache refresh.

---

### Cancel Timer
```
DELETE /timers/:id
```

**Description**: Cancel a pending timer

**Headers**:
```
X-API-Key: your-api-key
```

**Path Parameters**:
- `id`: UUID of the timer

**Success Response** (HTTP 200):
```json
{
    "code": 0,
    "message": "timer canceled successfully",
    "data": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "status": "canceled"
    }
}
```

**Error Response** (HTTP 400):
```json
{
    "code": 2,
    "message": "cannot cancel timer with status 'completed'",
    "data": null
}
```

**Behavior**:
1. Validate authentication
2. Parse UUID from path parameter
3. Query database to check timer exists and status
4. Reject cancellation for timers with status: completed, failed
5. Update status to 'canceled'
6. Set updated_at to current timestamp
7. Return confirmation with new status

**Notes**:
- Canceled timers remain in database (soft delete)
- Cannot cancel already completed or failed timers
- Canceled timers are not processed by scheduler
- Cannot un-cancel a timer (must create new one)
- **Eventual Consistency**: Cancellation is persisted to PostgreSQL immediately, but scheduler's in-memory cache refreshes within 30 seconds. If timer is already cached and due for execution, it might still execute before the next cache refresh detects the cancellation.

---

### Health Check
```
GET /health
```

**Description**: Check service health and database connectivity

**Headers**: None required (public endpoint)

**Success Response** (HTTP 200):
```json
{
    "code": 0,
    "message": "healthy",
    "data": {
        "status": "up",
        "database": "connected",
        "timestamp": "2025-10-26T10:30:00Z"
    }
}
```

**Error Response** (HTTP 500):
```json
{
    "code": 1,
    "message": "database connection failed",
    "data": {
        "status": "degraded",
        "database": "disconnected",
        "timestamp": "2025-10-26T10:30:00Z"
    }
}
```

**Behavior**:
1. Attempt simple database query (SELECT 1)
2. Return 200 if database responsive
3. Return 500 if database query fails
4. Include current timestamp

**Notes**:
- No authentication required (useful for load balancers/monitoring)
- Does not check scheduler thread status (MVP limitation)
- Fast timeout (5 seconds max)

## Business Logic

### Timer Scheduler

**Hybrid Storage Architecture**:

The scheduler uses a two-tier storage system to optimize performance while maintaining data durability:

1. **PostgreSQL (Persistent Layer)**: Stores ALL timers permanently. This is the source of truth and ensures no data is lost even if the application restarts.

2. **In-Memory Cache (Hot Layer)**: Stores only "near-term" timers (timers scheduled to execute within the next 5 minutes). This allows for fast, sub-second execution checks without constantly hitting the database.

**Why Hybrid Storage?**

Without caching, the scheduler would need to query PostgreSQL every second to find timers due for execution. At scale, this creates significant database load:
- Traditional approach: 60 queries/minute per instance
- Hybrid approach: 2 queries/minute per instance
- **Result**: 97% reduction in database queries

Additionally, in-memory lookups are orders of magnitude faster than database queries, enabling sub-second execution precision.

**Scheduler Flow Diagram**:

```
                    ┌─────────────────────────────────────┐
                    │    Application Startup              │
                    └───────────────┬─────────────────────┘
                                    │
                    ┌───────────────▼─────────────────────┐
                    │  Initialize In-Memory Cache          │
                    │  (Empty HashMap)                     │
                    └───────────────┬─────────────────────┘
                                    │
                    ┌───────────────▼─────────────────────┐
                    │  Spawn Two Concurrent Tasks          │
                    └───────────┬───────────┬─────────────┘
                                │           │
                ┌───────────────▼─┐       ┌─▼──────────────────┐
                │  Memory Loader  │       │  Execution Task    │
                │  (30s interval) │       │  (1s interval)     │
                └───────┬─────────┘       └─────┬──────────────┘
                        │                       │
                        │ every 30s             │ every 1s
                        ▼                       ▼
        ┌───────────────────────────┐   ┌──────────────────────────┐
        │ Query PostgreSQL:         │   │ Scan In-Memory Cache:    │
        │                           │   │                          │
        │ SELECT * FROM timers      │   │ for each timer in cache  │
        │ WHERE status = 'pending'  │   │   if execute_at <= NOW() │
        │ AND execute_at <=         │   │     execute callback     │
        │   NOW() + 5 minutes       │   │     remove from cache    │
        │ AND execute_at > NOW()    │   │                          │
        │                           │   │                          │
        └───────────┬───────────────┘   └──────────┬───────────────┘
                    │                              │
                    ▼                              ▼
        ┌───────────────────────┐       ┌─────────────────────────┐
        │ Update In-Memory      │       │ Update PostgreSQL:      │
        │ Cache:                │       │                         │
        │                       │       │ SET status='executing'  │
        │ - Add new timers      │       │ WHERE id = ?            │
        │ - Keep existing       │       │                         │
        │ - Remove expired      │       │ Then spawn async task   │
        └───────────────────────┘       └─────────────────────────┘
```

**Two Scheduler Tasks**:

**1. Memory Loader Task (Runs Every 30 Seconds)**:
- Queries PostgreSQL for timers entering the 5-minute execution window
- Loads these "near-term" timers into the in-memory cache
- Removes timers from cache that are no longer near-term (executed, canceled, or beyond window)
- Ensures the cache stays synchronized with the database
- Runs as a separate Tokio task in an infinite loop

**2. Execution Task (Runs Every 1 Second)**:
- Scans the in-memory cache for timers where `execute_at <= NOW()`
- For each due timer:
  - Updates its status to 'executing' in PostgreSQL
  - Removes it from the in-memory cache
  - Spawns an async task to execute the HTTP callback
- Does NOT query PostgreSQL directly for due timers (uses cache instead)
- Runs as a separate Tokio task in an infinite loop

**Cache Synchronization (Eventual Consistency)**:

API endpoints do NOT directly interact with the in-memory cache. All CRUD operations write only to PostgreSQL, and cache synchronization happens asynchronously via the Memory Loader task:

```
API Create/Update/Cancel Timer
      │
      ▼
┌─────────────────────┐
│ Write to PostgreSQL │
│    (only)           │
└─────────────────────┘
      │
      │ No cache interaction
      │ No notifications
      │
      ▼
  [PostgreSQL updated]
      │
      │ Eventually...
      │ (up to 30 seconds)
      ▼
┌─────────────────────┐
│  Memory Loader      │
│  (runs every 30s)   │
│                     │
│ - Queries DB        │
│ - Updates cache     │
│ - Removes stale     │
└─────────────────────┘
      │
      ▼
┌─────────────────────┐
│  In-Memory Cache    │
│  (synchronized)     │
└─────────────────────┘
```

**Why No Immediate Cache Sync?**

This design prioritizes simplicity over instant consistency:

- **Simpler Implementation**: API layer only knows about PostgreSQL, no cache management logic
- **No Coordination Required**: No pub/sub, no notifications, no distributed cache invalidation
- **Acceptable Delay**: 30-second delay is acceptable for near-term timers to appear in cache
- **Correctness Preserved**: PostgreSQL is source of truth, execution happens correctly (just potentially delayed by up to 30s)
- **Easier to Reason About**: Clear separation of concerns - API writes, scheduler reads

**Performance Benefits**:

| Aspect | Traditional Polling | Hybrid Storage |
|--------|-------------------|----------------|
| DB Queries/min | 60 per instance | 2 per instance |
| Execution Latency | ~100-500ms | ~1-10ms |
| Database Load | High (constant polling) | Low (periodic refresh) |
| Memory Usage | Minimal | ~1KB per near-term timer |
| Scalability | Limited by DB | Scales horizontally |

**Trade-offs**:

- **Memory Usage**: Stores near-term timers in RAM (typically small, <10MB for 10,000 timers in 5min window)
- **Eventual Consistency**: Newly created/updated timers appear in cache within 30 seconds (acceptable delay for simplicity)
- **Execution Delay**: Timers scheduled to execute within 30 seconds might be delayed by up to 30 seconds until next memory loader sync
- **Simpler Design**: No cache invalidation, no pub/sub, no coordination between API and scheduler - easier to implement and maintain

**Timer State Machine**:

```
                         ┌─────────┐
                         │ PENDING │  (Initial state)
                         └────┬────┘
                              │
                              │ execute_at reached
                              │
                              ▼
                         ┌──────────┐
                         │EXECUTING │  (Callback in progress)
                         └────┬─────┘
                              │
                ┌─────────────┼─────────────┐
                │             │             │
     HTTP 2xx   │             │             │  HTTP 4xx/5xx
     success    │             │             │  timeout or
                │             │             │  network error
                ▼             │             ▼
          ┌──────────┐        │        ┌────────┐
          │COMPLETED │        │        │ FAILED │
          └──────────┘        │        └────────┘
                              │
   ┌──────────┐               │
   │ CANCELED │◄──────────────┘
   └──────────┘        DELETE /timers/:id
      ▲                (from PENDING only)
      │
      │ Manual cancellation
      │ via API
      │
   ┌──┴──────┐
   │ PENDING │
   └─────────┘
```

**State Descriptions**:

- **PENDING**: Timer created and waiting for `execute_at` time to arrive. Timer is in the in-memory cache if within 5-minute window. Can be canceled via API.

- **EXECUTING**: Scheduler has picked up the timer and is currently attempting to execute the HTTP callback. Timer is removed from cache during this state. Cannot be canceled during execution.

- **COMPLETED**: Callback successfully executed (received HTTP 2xx response). This is a final state. Timer remains in PostgreSQL for historical tracking.

- **FAILED**: Callback failed (HTTP 4xx/5xx, timeout, or network error). No retry attempts. This is a final state. Timer remains in PostgreSQL with error message in `last_error` field for debugging.

- **CANCELED**: User manually canceled the timer via API before execution. This is a final state. Timer can only be canceled from PENDING state.

### Callback Execution

**Callback Execution Flow**:

When a timer's `execute_at` time is reached, the scheduler spawns an asynchronous task to execute the HTTP callback to the external service.

```
┌──────────────────────────────────────────────────────────────┐
│              Callback Execution Process                       │
└──────────────────────────────────────────────────────────────┘

  ┌─────────────────┐
  │ Timer Due       │
  │ execute_at      │
  │ <= NOW()        │
  └────────┬────────┘
           │
           ▼
  ┌─────────────────────────────────────┐
  │ Build HTTP Request:                 │
  │                                     │
  │ Method: callback_method (POST/PUT) │
  │ URL: callback_url                   │
  │ Headers:                            │
  │   - Content-Type: application/json  │
  │   - User-Agent: timer-platform/0.1.0│
  │   - Custom headers from DB          │
  │ Body: callback_payload (JSON)       │
  │ Timeout: 30 seconds                 │
  └────────┬────────────────────────────┘
           │
           ▼
  ┌─────────────────┐
  │ Send HTTP       │
  │ Request         │
  │ (async)         │
  └────────┬────────┘
           │
           ▼
  ┌────────────────────────────┐
  │ Wait for Response          │
  │ (max 30s timeout)          │
  └────────┬───────────────────┘
           │
           │
    ┌──────┴──────┐
    │             │
    ▼             ▼
┌─────────┐  ┌──────────────┐
│HTTP 2xx │  │HTTP 4xx/5xx  │
│Success  │  │Network Error │
│         │  │Timeout       │
└────┬────┘  └──────┬───────┘
     │              │
     │              ▼
     │       ┌────────────────┐
     │       │ Mark as FAILED │
     │       │                │
     │       │ Store error    │
     │       │ message in     │
     │       │ last_error     │
     │       └────────────────┘
     │
     ▼
┌────────────────┐
│ Mark as        │
│ COMPLETED      │
│                │
│ Set            │
│ executed_at    │
│ = NOW()        │
└────────────────┘
```

**HTTP Request Details**:

The platform constructs HTTP requests with the following configuration:

- **Method**: Uses the `callback_method` stored in the timer (POST, PUT, or PATCH). POST is the default if not specified.

- **URL**: The full `callback_url` from the timer. Must be a valid HTTP or HTTPS URL. The platform does not modify or validate the URL structure beyond basic format checks.

- **Headers**:
  - `Content-Type: application/json` - Always sent, as all payloads are JSON
  - `User-Agent: timer-platform/0.1.0` - Identifies the caller
  - Custom headers from `callback_headers` field - Allows authentication tokens, API keys, or any custom headers the external service requires

- **Body**: The `callback_payload` JSON object serialized as the request body. Can be any valid JSON structure (object, array, string, number, etc.).

- **Timeout**: Fixed 30-second timeout. If the external service doesn't respond within 30 seconds, the request is canceled and treated as a failure.

**Success Criteria**:
- HTTP status code 2xx (200-299)
- Response body ignored (not stored)
- Timer marked as 'completed'
- Set executed_at timestamp

**Failure Handling**:

When a callback fails (HTTP 4xx/5xx, network error, or timeout), the timer is immediately marked as FAILED:

1. **Store Error Details**: The error message (e.g., "HTTP 500", "Connection timeout") is stored in the `last_error` field for debugging purposes.

2. **Mark as FAILED**: The timer status is immediately set to `failed` with no retry attempts.

3. **Finalize**: The `executed_at` timestamp is set to record when the failure occurred.

**Why No Retry Logic?**

This design prioritizes simplicity and predictability. External services should implement their own retry logic by creating new timers if needed. This approach:

- **Simplifies Implementation**: No retry state machine or scheduling logic required
- **Predictable Behavior**: Each timer executes exactly once
- **Clear Ownership**: External services control retry policies through their own logic
- **Easier Debugging**: Single execution per timer makes troubleshooting straightforward

If automatic retries are needed in the future, they can be added as an optional feature with configurable policies.

### Error Handling

**Database Errors**:
- Connection failures: Log error, retry next scheduler interval
- Constraint violations: Return 400 to client with descriptive message
- Query timeouts: Log error, continue processing other timers

**Callback Errors**:
- Network/DNS/timeout errors: Mark as 'failed' immediately, store error in last_error
- TLS/SSL errors: Mark as 'failed' immediately, store error in last_error
- HTTP 4xx/5xx errors: Mark as 'failed' immediately, store error in last_error
- No retry attempts - single execution per timer

**Configuration Errors**:
- Invalid DATABASE_URL: Panic on startup (fail fast)
- Missing API_KEY: Panic on startup (fail fast)

## Docker Setup

### Dockerfile

```dockerfile
# Multi-stage build for optimized image size

# Stage 1: Build
FROM rust:1.75-bookworm as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/timer /app/timer

# Expose port
EXPOSE 3000

# Run the binary
CMD ["/app/timer"]
```

**Build Instructions**:
```bash
docker build -t timer-platform:latest .
```

### docker-compose.yml

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    container_name: timer-postgres
    environment:
      POSTGRES_USER: timer
      POSTGRES_PASSWORD: timer123
      POSTGRES_DB: timerdb
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    networks:
      - timer-network
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U timer -d timerdb"]
      interval: 10s
      timeout: 5s
      retries: 5

  timer:
    build: .
    container_name: timer-platform
    environment:
      DATABASE_URL: postgresql://timer:timer123@postgres:5432/timerdb
      API_KEY: dev-api-key-change-in-production
      PORT: 3000
      RUST_LOG: info
    ports:
      - "3000:3000"
    depends_on:
      postgres:
        condition: service_healthy
    networks:
      - timer-network
    restart: unless-stopped

networks:
  timer-network:
    driver: bridge

volumes:
  postgres_data:
```

**Usage**:
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f timer

# Stop all services
docker-compose down

# Rebuild after code changes
docker-compose up -d --build
```

## Database Migrations

### Migration File: `migrations/001_create_timers_table.sql`

```sql
-- Create timers table
CREATE TABLE timers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    execute_at TIMESTAMPTZ NOT NULL,
    callback_url TEXT NOT NULL,
    callback_method VARCHAR(10) NOT NULL DEFAULT 'POST',
    callback_headers JSONB,
    callback_payload JSONB,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    last_error TEXT,
    executed_at TIMESTAMPTZ,
    metadata JSONB,

    CONSTRAINT valid_status CHECK (status IN ('pending', 'executing', 'completed', 'failed', 'canceled')),
    CONSTRAINT valid_method CHECK (callback_method IN ('POST', 'PUT', 'PATCH')),
    CONSTRAINT future_execute_at CHECK (execute_at > created_at)
);

-- Create indexes for performance
CREATE INDEX idx_timers_execute_at_status ON timers(execute_at, status)
    WHERE status = 'pending';

CREATE INDEX idx_timers_status ON timers(status);

CREATE INDEX idx_timers_created_at ON timers(created_at DESC);

-- Create updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_timers_updated_at BEFORE UPDATE ON timers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

**Running Migrations**:

SQLx supports compile-time checked migrations. Run migrations using:

```bash
# Create migration
sqlx migrate add create_timers_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## Dependencies (Cargo.toml)

The project already has dependencies configured. Key dependencies:

```toml
[dependencies]
# Web framework
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["trace", "cors"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "chrono", "uuid"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Time handling
chrono = { version = "0.4", features = ["serde"] }

# UUID generation
uuid = { version = "1.0", features = ["v4", "serde"] }

# HTTP client for callbacks
reqwest = { version = "0.11", features = ["json"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Environment variables
dotenvy = "0.15"
```

## Development Commands

**Build**:
```bash
cargo build
```

**Run locally** (requires PostgreSQL running):
```bash
# Copy example env file
cp .env.example .env

# Edit .env with your configuration
# Then run
cargo run
```

**Run in release mode**:
```bash
cargo run --release
```

**Build for release**:
```bash
cargo build --release
```

**Check code** (faster than build):
```bash
cargo check
```

**Run tests**:
```bash
cargo test
```

**Run a specific test**:
```bash
cargo test test_create_timer
```

**Format code**:
```bash
cargo fmt
```

**Lint**:
```bash
cargo clippy
```

**Database operations**:
```bash
# Create database
sqlx database create

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check SQLx queries at compile time (requires running database)
cargo sqlx prepare
```

## Testing

### Manual Testing Checklist

**Prerequisites**:
```bash
# Start services
docker-compose up -d

# Set API key
export API_KEY="dev-api-key-change-in-production"
```

**Test 1: Create Timer**
```bash
curl -X POST http://localhost:3000/timers \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "execute_at": "2025-10-26T16:00:00Z",
    "callback_url": "https://webhook.site/your-unique-id",
    "callback_method": "POST",
    "callback_payload": {
      "message": "Timer triggered!",
      "timestamp": "2025-10-26T16:00:00Z"
    }
  }'
```

**Expected**: HTTP 201, response with timer ID and status='pending'

**Test 2: Get Timer**
```bash
curl -X GET http://localhost:3000/timers/{TIMER_ID} \
  -H "X-API-Key: $API_KEY"
```

**Expected**: HTTP 200, complete timer details

**Test 3: List Timers**
```bash
curl -X GET "http://localhost:3000/timers?status=pending&limit=10" \
  -H "X-API-Key: $API_KEY"
```

**Expected**: HTTP 200, array of timers with pagination metadata

**Test 4: Update Timer**
```bash
curl -X PUT http://localhost:3000/timers/{TIMER_ID} \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "execute_at": "2025-10-26T17:00:00Z"
  }'
```

**Expected**: HTTP 200, updated timer with new execute_at

**Test 5: Cancel Timer**
```bash
curl -X DELETE http://localhost:3000/timers/{TIMER_ID} \
  -H "X-API-Key: $API_KEY"
```

**Expected**: HTTP 200, timer status changed to 'canceled'

**Test 6: Health Check**
```bash
curl -X GET http://localhost:3000/health
```

**Expected**: HTTP 200, status='up', database='connected'

**Test 7: Authentication Failure**
```bash
curl -X GET http://localhost:3000/timers \
  -H "X-API-Key: wrong-key"
```

**Expected**: HTTP 401, code=4, message about unauthorized

**Test 8: Timer Execution** (requires webhook receiver)
```bash
# Create timer with execute_at in 10 seconds
FUTURE_TIME=$(date -u -v+10S +"%Y-%m-%dT%H:%M:%SZ")

curl -X POST http://localhost:3000/timers \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d "{
    \"execute_at\": \"$FUTURE_TIME\",
    \"callback_url\": \"https://webhook.site/your-unique-id\",
    \"callback_payload\": {\"test\": \"execution\"}
  }"

# Wait 15 seconds, then check timer status
sleep 15
curl -X GET http://localhost:3000/timers/{TIMER_ID} \
  -H "X-API-Key: $API_KEY"
```

**Expected**: Timer status='completed', executed_at populated, webhook received callback

**Test 9: Failure Handling** (requires failing webhook)
```bash
# Create timer with invalid URL to test failure handling
curl -X POST http://localhost:3000/timers \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "execute_at": "2025-10-26T16:00:00Z",
    "callback_url": "https://localhost:9999/nonexistent"
  }'

# Monitor logs to see failure
docker-compose logs -f timer
```

**Expected**: Single execution attempt, immediate status='failed' with error message in last_error field

### Integration Tests

Integration tests should cover the following scenarios:

**Database Layer Tests**:
- Create timer and verify it's stored correctly
- Retrieve timer by ID
- List timers with filtering (by status)
- Update timer fields
- Cancel timer and verify status change
- Query near-term timers for memory loader

**Scheduler Tests**:
- Memory loader loads near-term timers into cache
- Execution task picks up due timers from cache
- Timer state transitions correctly (pending -> executing -> completed/failed)
- Failed timers are marked correctly with error details
- Canceled timers are not executed

**Callback Tests**:
- Successful callback (HTTP 200) marks timer as completed
- Failed callback (HTTP 500) marks timer as failed immediately
- Network errors mark timer as failed with error message
- Timeout errors mark timer as failed with error message
- Error details stored in last_error field for debugging

## Common Issues & Debugging

**Issue**: `sqlx::Error: error returned from database: relation "timers" does not exist`
- **Solution**: Run migrations with `sqlx migrate run` or recreate database
- **Check**: Verify `DATABASE_URL` is correct and database exists
- **Command**: `docker-compose exec postgres psql -U timer -d timerdb -c "\dt"`

**Issue**: Timer not executing at expected time
- **Debugging**:
  1. Check timer status: `curl -H "X-API-Key: $API_KEY" http://localhost:3000/timers/{ID}`
  2. Verify execute_at is in past: Compare execute_at to current UTC time
  3. Check scheduler logs: `docker-compose logs -f timer | grep scheduler`
  4. Verify scheduler interval: Check `SCHEDULER_INTERVAL_SECS` environment variable
- **Solution**: Ensure execute_at is in UTC, check database clock sync

**Issue**: Callbacks failing with SSL/TLS errors
- **Debugging**: Check callback URL uses valid HTTPS certificate
- **Solution**: For local testing, use HTTP endpoints or services like webhook.site
- **Note**: Production should always use HTTPS

**Issue**: High database load from scheduler polling
- **Debugging**: Check index usage: `EXPLAIN ANALYZE SELECT * FROM timers WHERE status = 'pending' AND execute_at <= NOW()`
- **Solution**: Verify indexes exist, check memory loader is working properly
- **Optimization**: Use PostgreSQL `LISTEN/NOTIFY` for event-driven scheduling (future enhancement)

**Issue**: Memory usage growing over time
- **Debugging**: Check number of spawned tasks: `docker stats timer-platform`
- **Solution**: Ensure callback tasks are completing (check for timeout issues)
- **Prevention**: Implement task limit or use bounded executor (future enhancement)

**Issue**: Cannot connect to database from timer service
- **Debugging**:
  1. Check PostgreSQL is running: `docker-compose ps postgres`
  2. Check network connectivity: `docker-compose exec timer ping postgres`
  3. Verify DATABASE_URL: `docker-compose exec timer env | grep DATABASE_URL`
- **Solution**: Ensure `depends_on` and `healthcheck` configured in docker-compose.yml

## MVP Limitations

**NOT Implemented in MVP**:

1. **Recurring/Periodic Timers**: Only one-shot timers supported
   - **Workaround**: Client must create new timer after each execution
   - **Future**: Add `recurrence_pattern` field for cron-like scheduling

2. **Multi-tenant Authentication**: Single shared API key for all clients
   - **Workaround**: Deploy separate instances per tenant
   - **Future**: Add client_id/client_secret authentication

3. **Rate Limiting**: No request throttling or quota management
   - **Workaround**: Use reverse proxy (nginx) for rate limiting
   - **Future**: Add per-client rate limits with Redis

4. **Callback Response Storage**: Response body from callbacks not stored
   - **Workaround**: Callback receiver must persist responses
   - **Future**: Add optional response storage with size limits

5. **Webhook Signatures**: No HMAC signatures on callbacks
   - **Workaround**: Use HTTPS with unique callback URLs per client
   - **Future**: Add `X-Timer-Signature` header with HMAC-SHA256

6. **Distributed Scheduler**: Single scheduler instance (multi-instance deployments may duplicate work)
   - **Workaround**: Use database locking (`FOR UPDATE SKIP LOCKED`)
   - **Future**: Add distributed lock manager (Redis) or leader election

7. **Timer Timezone Support**: All times must be UTC
   - **Workaround**: Client converts local time to UTC before submission
   - **Future**: Accept timezone parameter, convert internally

8. **Bulk Operations**: No batch create/update/delete endpoints
   - **Workaround**: Make multiple sequential requests
   - **Future**: Add `POST /timers/batch` endpoint

9. **Metrics/Observability**: Basic logging only, no Prometheus metrics
   - **Workaround**: Parse logs for monitoring
   - **Future**: Add `/metrics` endpoint with Prometheus exporter

10. **Timer Deduplication**: No protection against duplicate timer creation
    - **Workaround**: Client generates idempotency keys and checks for duplicates
    - **Future**: Add `idempotency_key` field with unique constraint

11. **Callback Request Validation**: No schema validation for callback responses
    - **Workaround**: Client validates responses on their end
    - **Future**: Add optional JSON schema validation

## Implementation Notes

### Application Initialization

The application follows a sequential startup process to ensure all components are properly initialized before accepting requests:

```
┌─────────────────────────────────────────────────────────┐
│            Application Startup Sequence                 │
└─────────────────────────────────────────────────────────┘

1. Load Environment Variables
   │
   ├─ Read .env file (if exists)
   ├─ Parse DATABASE_URL (required)
   ├─ Parse API_KEY (required)
   ├─ Parse PORT (optional, default: 3000)
   └─ Parse RUST_LOG (optional, default: info)
   │
   ▼
2. Initialize Logging
   │
   ├─ Configure tracing subscriber
   ├─ Set log level from RUST_LOG
   └─ Enable structured logging for observability
   │
   ▼
3. Load Configuration
   │
   ├─ Validate DATABASE_URL format
   ├─ Validate API_KEY length (minimum 32 chars)
   └─ Store configuration in global state
   │
   ▼
4. Connect to Database
   │
   ├─ Create PostgreSQL connection pool
   ├─ Set max connections to 5
   ├─ Test connection with ping
   └─ Fail fast if database unreachable
   │
   ▼
5. Run Database Migrations
   │
   ├─ Execute all pending migrations
   ├─ Create timers table (if not exists)
   ├─ Create indexes
   └─ Create triggers (updated_at auto-update)
   │
   ▼
6. Start Scheduler Tasks
   │
   ├─ Initialize in-memory cache (empty HashMap)
   ├─ Spawn memory loader task (30s interval)
   ├─ Spawn execution task (1s interval)
   └─ Share cache reference in AppState (used only by scheduler)
   │
   ▼
7. Build API Router
   │
   ├─ Mount API endpoints (/timers, /health)
   ├─ Add authentication middleware (X-API-Key)
   ├─ Add tracing middleware (request logging)
   └─ Attach shared state (pool, config, cache)
   │
   ▼
8. Start HTTP Server
   │
   ├─ Bind to 0.0.0.0:{PORT}
   ├─ Log startup message
   └─ Begin accepting connections
   │
   ▼
[RUNNING]
```

**Key Implementation Details**:

- **Fail-Fast Startup**: If any step fails (missing env vars, database connection, migrations), the application panics and exits immediately. This ensures the system is not running in a degraded state.

- **Connection Pooling**: PostgreSQL connection pool is limited to 5 connections to prevent overwhelming the database. This is suitable for MVP workloads.

- **Scheduler Independence**: The scheduler tasks run independently of the API server. Even if the API is overloaded, timers continue to execute.

- **Shared State**: The in-memory cache is wrapped in `Arc<RwLock<>>` for thread-safe access by scheduler tasks. API handlers do NOT interact with the cache - they only read/write to PostgreSQL. Only the Memory Loader task writes to cache, and the Execution task reads from it.

### Scheduler Implementation

The scheduler consists of two independent Tokio tasks that run concurrently in infinite loops:

**Memory Loader Task Implementation**:
- Runs every 30 seconds in an infinite loop
- Queries PostgreSQL for timers within 5-minute window
- Acquires write lock on cache
- Inserts/updates timers in HashMap
- Releases write lock
- Sleeps for 30 seconds
- Repeats

**Execution Task Implementation**:
- Runs every 1 second in an infinite loop
- Acquires read lock on cache
- Filters timers where `execute_at <= NOW()`
- Collects due timers into a vector
- Releases read lock
- For each due timer:
  - Updates status to 'executing' in PostgreSQL
  - Removes from cache (write lock)
  - Spawns async callback task
- Sleeps for 1 second
- Repeats

**Concurrency Considerations**:

- **Read-Write Lock**: Uses `tokio::sync::RwLock` for concurrent access. Multiple readers can read simultaneously, but writers block everyone.

- **Lock Duration**: Locks are held for minimal time. Read locks released before spawning callback tasks to avoid blocking.

- **Task Independence**: Memory loader and execution task never block each other (different lock acquisitions).

- **Async Spawning**: Each callback executes in its own Tokio task, so slow callbacks don't block timer execution.

### Callback Execution Implementation

The callback execution logic is encapsulated in a dedicated module that handles the HTTP request construction, execution, and result processing.

**Callback Handler Responsibilities**:

1. **HTTP Client Setup**: Creates a configured HTTP client with 30-second timeout

2. **Request Construction**:
   - Parse callback method from timer (POST/PUT/PATCH, default to POST)
   - Set URL from timer's `callback_url`
   - Add standard headers (`Content-Type`, `User-Agent`)
   - Merge custom headers from timer's `callback_headers` JSONB field
   - Serialize `callback_payload` as JSON body (if present)

3. **Request Execution**: Send HTTP request asynchronously using reqwest

4. **Response Handling**:
   - **Success (2xx status)**: Update timer status to `COMPLETED`, set `executed_at` timestamp, log success
   - **Failure (non-2xx or error)**: Mark as `FAILED`, store error message, set `executed_at` timestamp, log warning

5. **Failure Handler**:
   - Store error details in `last_error` field
   - Mark timer status as `FAILED`
   - Set `executed_at` to record when the failure occurred
   - Log warning with timer ID and error details
   - All database updates use fire-and-forget pattern (errors logged but not propagated)

**Database Updates**:

All timer state changes are persisted to PostgreSQL immediately:
- `status` field updated to reflect current state (pending, executing, completed, failed, canceled)
- `last_error` stores error message when callback fails
- `executed_at` set on both successful completion and failure
- `updated_at` automatically updated by database trigger

### API Error Response Handling

The API layer implements standardized error responses to ensure consistent error handling across all endpoints:

**Error Type Structure**:
- `code`: Integer response code (0-4, as defined in Response Format section)
- `message`: Human-readable error description
- `status`: HTTP status code (400, 401, 404, 500)

**Error Categories**:

1. **Validation Errors** (code: 2, HTTP 400)
   - Invalid input format
   - Missing required fields
   - Out-of-range values
   - Example: "execute_at must be in the future"

2. **Not Found Errors** (code: 3, HTTP 404)
   - Timer ID doesn't exist
   - Example: "timer not found"

3. **Unauthorized Errors** (code: 4, HTTP 401)
   - Missing X-API-Key header
   - Invalid API key
   - Example: "unauthorized"

4. **Internal Errors** (code: 1, HTTP 500)
   - Database connection failures
   - Unexpected runtime errors
   - Example: "database error: connection lost"

All error responses follow the standard `ApiResponse<T>` format with `data: null` and appropriate `code` and `message` fields.

## Integration

**Outbound (Callbacks)**:
- HTTP POST/PUT/PATCH to `callback_url` with JSON payload
- Custom headers via `callback_headers`
- 30s timeout (fixed)
- Single execution attempt per timer
- 2xx = success (completed), 4xx/5xx/network error = failure (failed)

**Inbound (API)**:
- RESTful HTTP API with `X-API-Key` authentication
- JSON request/response bodies
- 6 endpoints: create, get, list, update, cancel, health
- See API Endpoints section for full details

**Service Dependencies**:
- PostgreSQL 15+ (required) - Must be running and accessible
- No other microservices required (standalone service)
- No Redis, message queues, or external caches needed

## Production Considerations

**Security**: Strong API keys (32+ chars), HTTPS callbacks, per-client auth, rate limiting

**Scalability**: Multi-instance safe, monitor DB pool, consider Redis for distributed locks

**Monitoring**: Log state transitions, track callback rates, alert on failures
