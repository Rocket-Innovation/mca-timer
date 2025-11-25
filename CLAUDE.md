# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project: MCA Timer/Callback Platform

**This is a timer/callback scheduling service** - registers and executes HTTP or NATS callbacks at specified times.

## 📚 Platform Documentation

**All cross-platform documentation is centralized in: `../mca-document/`**

**Essential Guides:**
- **[Documentation Index](../mca-document/README.md)** - Complete platform documentation catalog
- **[Architecture Guide](../mca-document/ARCHITECTURE_GUIDE.md)** - System architecture and integration patterns
- **[Quick Start Guide](../mca-document/QUICK_START.md)** - 5-minute platform setup

## Service Overview

**Language:** Rust (edition 2024)
**Database:** PostgreSQL 15+
**Optional:** NATS (for NATS callbacks)

### Responsibilities
- Accept timer registration via REST API
- Store timers in PostgreSQL
- Execute HTTP or NATS callbacks at specified times
- Track timer status (pending, executing, completed, failed, canceled)

### Key Features
- **Hybrid Storage**: PostgreSQL (persistent) + in-memory cache (hot layer)
- **Dual Callback Types**: HTTP POST or NATS publish
- **One-shot Execution**: Single attempt, no retry
- **Eventual Consistency**: 30-second cache sync delay

### Architecture
```
src/
├── main.rs              # Application entry, server setup
├── config.rs            # Environment configuration
├── models.rs            # Shared data structures
├── db.rs                # Database operations (SQLx)
├── scheduler.rs         # Timer polling and execution
├── callback.rs          # Unified callback dispatcher
├── callback_http.rs     # HTTP callback handler
├── callback_nats.rs     # NATS callback handler
├── api_create_timer.rs  # POST /timers
├── api_get_timer.rs     # GET /timers/:id
├── api_list_timers.rs   # GET /timers
├── api_update_timer.rs  # PUT /timers/:id
├── api_cancel_timer.rs  # DELETE /timers/:id
├── api_health.rs        # GET /healthz
└── auth.rs              # API key middleware
```

## Tech Stack

- **Axum 0.7** - Web framework
- **Tokio 1.x** - Async runtime
- **SQLx 0.7** - Type-safe PostgreSQL client
- **Reqwest 0.11** - HTTP client (for callbacks)
- **async-nats 0.33** - NATS client (optional)
- **Chrono 0.4** - Date/time handling
- **UUID 1.0** - UUIDv7 for timer IDs

## Development Commands

### Quick Start
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f timer

# Stop all services
docker-compose down
```

### Local Development
```bash
# Copy env file
cp .env.example .env

# Run locally (requires PostgreSQL)
cargo run

# Run in release mode
cargo run --release

# Build for release
cargo build --release
```

### Testing & Quality
```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Check code (faster than build)
cargo check
```

### Database Operations
```bash
# Create database
sqlx database create

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Prepare for offline mode
cargo sqlx prepare
```

## Environment Variables

### Required
```bash
# Database (component-based)
PG_HOST=localhost
PG_PORT=5432
PG_USER=timer
PG_PASSWORD=timer123
PG_DB_NAME=timerdb

# Authentication
API_KEY=dev-api-key-change-in-production-min-32-chars   # Min 32 chars
```

### Optional
```bash
# Application
PORT=8080                # HTTP server port
RUST_LOG=info            # Log level (trace, debug, info, warn, error)

# NATS (optional, enables NATS callbacks)
NATS_HOST=localhost      # Required if using NATS callbacks
NATS_PORT=4222
NATS_USER=               # Optional auth
NATS_PASSWORD=           # Optional auth
```

## API Endpoints

### Create Timer (HTTP Callback)
```bash
POST /timers
X-API-Key: your-api-key
Content-Type: application/json

{
  "execute_at": "2025-10-26T15:30:00Z",
  "callback": {
    "type": "http",
    "url": "https://api.example.com/webhook",
    "headers": {"Authorization": "Bearer token"},
    "payload": {"event": "timer_triggered"}
  },
  "metadata": {"client_ref": "order-456"}
}
```

**Response (201):**
```json
{
  "code": 0,
  "message": "timer created successfully",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "pending",
    "callback_type": "http"
  }
}
```

### Create Timer (NATS Callback)
```bash
POST /timers
X-API-Key: your-api-key
Content-Type: application/json

{
  "execute_at": "2025-10-26T15:30:00Z",
  "callback": {
    "type": "nats",
    "topic": "events.timer.triggered",
    "key": "user123",
    "headers": {"X-Event-Type": "timer_triggered"},
    "payload": {"event": "timer_triggered", "user_id": "user123"}
  }
}
```

### Other Endpoints
```bash
GET    /timers/:id                        # Get timer details
GET    /timers?status=pending&limit=20    # List timers
PUT    /timers/:id                        # Update pending timer
DELETE /timers/:id                        # Cancel pending timer
GET    /healthz                           # Health check (no auth)
```

## Database Schema

```sql
CREATE TABLE timers (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    execute_at TIMESTAMPTZ NOT NULL,
    callback_type VARCHAR(10) NOT NULL DEFAULT 'http',
    callback_config JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    last_error TEXT,
    executed_at TIMESTAMPTZ,
    metadata JSONB,

    CONSTRAINT valid_callback_type CHECK (callback_type IN ('http', 'nats'))
);

-- Performance indexes
CREATE INDEX idx_timers_execute_at_status ON timers(execute_at, status)
    WHERE status = 'pending';
```

## Timer States

```
PENDING → EXECUTING → COMPLETED (2xx success)
                   → FAILED    (error/timeout)

PENDING → CANCELED (manual via API)
```

**States:**
- **PENDING**: Waiting for execute_at
- **EXECUTING**: Callback in progress
- **COMPLETED**: Callback succeeded (HTTP 2xx or NATS published)
- **FAILED**: Callback failed (no retry)
- **CANCELED**: Manually canceled before execution

## Scheduler Architecture

**Two concurrent tasks:**

### 1. Memory Loader (30s interval)
- Queries PostgreSQL for near-term + overdue timers
- Loads into in-memory cache
- Window: NOW() - 5min to NOW() + 1min

### 2. Execution Task (1s interval)
- Scans cache for due timers (execute_at <= NOW())
- Spawns callback task
- Removes from cache immediately

**Trade-offs:**
- ✅ 97% fewer DB queries vs polling
- ✅ Sub-second execution precision
- ⚠️ 30s eventual consistency delay
- ⚠️ Timers overdue >5min not executed

## Callback Execution

### HTTP Callbacks
- Method: HTTP POST (hardcoded)
- Timeout: 30 seconds
- Success: HTTP 2xx response
- Failure: 4xx/5xx, network error, timeout
- No retry (single attempt)

### NATS Callbacks
- Publish to topic (fire-and-forget)
- No subscriber ACK wait
- Success: Published without error
- Failure: Connection/publish error
- No retry (single attempt)

## Common Issues

### Timer Not Executing
**Check:**
```bash
# Verify timer status
curl -H "X-API-Key: $API_KEY" http://localhost:8080/timers/{ID}

# Check logs
docker-compose logs -f timer | grep scheduler

# Verify execute_at is in past (UTC)
date -u
```

### Database Connection Errors
**Diagnosis:**
```bash
# Check PostgreSQL running
docker-compose ps postgres

# Test connection
docker-compose exec timer env | grep PG_

# Check migrations
docker-compose exec postgres psql -U timer -d timerdb -c "\dt"
```

**Solution:**
```bash
# Recreate database
sqlx migrate run
```

### NATS Callback Failures
**Check:**
- Verify `NATS_HOST` configured in env
- Check NATS server running: `docker-compose ps nats`
- Test connection: `docker-compose exec timer env | grep NATS_`

**Note:** NATS callbacks require NATS server. HTTP callbacks work standalone.

### High Memory Usage
**Debugging:**
```bash
# Check container stats
docker stats timer-platform

# Check cache size in logs
docker-compose logs timer | grep "cache size"
```

**Normal:** ~2-5MB for timers in 6-minute window

### SSL/TLS Errors
**Solution:** For local testing, use HTTP or webhook.site

## Testing

### Quick Manual Test
```bash
# Create timer for 10 seconds from now
FUTURE_TIME=$(date -u -v+10S +"%Y-%m-%dT%H:%M:%SZ")

curl -X POST http://localhost:8080/timers \
  -H "X-API-Key: dev-api-key-change-in-production-min-32-chars" \
  -H "Content-Type: application/json" \
  -d "{
    \"execute_at\": \"$FUTURE_TIME\",
    \"callback\": {
      \"type\": \"http\",
      \"url\": \"https://webhook.site/your-unique-id\",
      \"payload\": {\"test\": \"execution\"}
    }
  }"

# Wait 15 seconds, check status
sleep 15
curl -H "X-API-Key: dev-api-key-change-in-production-min-32-chars" \
  http://localhost:8080/timers/{TIMER_ID}
```

**Expected:** status='completed', executed_at populated

## Docker Setup

### docker-compose.yml
```yaml
services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: timer
      POSTGRES_PASSWORD: timer123
      POSTGRES_DB: timerdb
    ports:
      - "5432:5432"

  nats:
    image: nats:2.10-alpine
    ports:
      - "4222:4222"
    command: "-js -m 8222"

  timer:
    build: .
    environment:
      PG_HOST: postgres
      PG_PORT: 5432
      PG_USER: timer
      PG_PASSWORD: timer123
      PG_DB_NAME: timerdb
      API_KEY: dev-api-key-change-in-production-min-32-chars
      NATS_HOST: nats
      NATS_PORT: 4222
    ports:
      - "8080:8080"
    depends_on:
      - postgres
      - nats
```

### Multi-stage Dockerfile
```dockerfile
FROM rust:1.75-bookworm as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3
COPY --from=builder /app/target/release/timer /app/timer
CMD ["/app/timer"]
```

## Response Format

### Status Codes
- **200** - Success (GET, PUT)
- **201** - Created (POST)
- **400** - Bad request (validation error)
- **401** - Unauthorized (invalid API key)
- **404** - Timer not found
- **500** - Internal error

### Response Body Codes
- **0** - Success
- **1** - Unexpected error
- **2** - Validation error
- **3** - Not found
- **4** - Unauthorized

## Key Constraints

**What this service does NOT do:**
- ❌ Retry failed callbacks (single attempt only)
- ❌ Guarantee millisecond precision (checks every 1s)
- ❌ Support recurring/periodic timers (one-shot only)
- ❌ Store callback responses
- ❌ Provide instant cache sync (30s delay)
- ❌ Execute timers overdue >5 minutes

## Related Services

This service is standalone and does not directly integrate with other MCA services. It's used by:
- **[mca-automation-workflow](../mca-automation-workflow/CLAUDE.md)** - May use timers for delayed workflow execution
- **External systems** - Any service needing scheduled callbacks

**Other MCA Services:**
- **[mca-front-end](../mca-front-end/CLAUDE.md)** - React workflow builder UI
- **[mca-bigQuery](../mca-bigQuery/CLAUDE.md)** - PostgreSQL query service
- **[mca-notification](../mca-notification/CLAUDE.md)** - LINE notification worker
- **[mca-engine-sdk](../mca-engine-sdk/CLAUDE.md)** - Spider-Go workflow SDK
- **[roc-argocd](../roc-argocd/CLAUDE.md)** - Kubernetes deployment

---

**Last Updated:** November 25, 2025
**Service:** Timer/Callback Scheduling Platform
**Repository:** `mca-timer`
