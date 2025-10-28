# Task: Create Docker Compose Configuration

**Status**: pending
**Dependencies**: TK-011
**Estimated Effort**: small

## Objective

Create a docker-compose.yml file that orchestrates the timer platform with PostgreSQL for local development and testing.

## Context

Docker Compose simplifies local development by defining both the timer application and PostgreSQL database as services with proper networking, health checks, and dependency management. The configuration ensures PostgreSQL is fully ready before the timer service starts, automatically runs migrations on container startup, and persists database data in a named volume. This setup allows developers to start the entire stack with a single command.

## Files to Modify/Create

- `docker-compose.yml` - New Docker Compose configuration at project root

## Detailed Steps

1. Create `docker-compose.yml` at project root: `/Users/tar/Documents/alpha/timer/docker-compose.yml`
2. Define version: `version: '3.8'`
3. Define `postgres` service:
   - Image: `postgres:15-alpine`
   - Container name: `timer-postgres`
   - Environment variables:
     - `POSTGRES_USER: timer`
     - `POSTGRES_PASSWORD: timer123`
     - `POSTGRES_DB: timerdb`
   - Ports: `5432:5432`
   - Volumes:
     - `postgres_data:/var/lib/postgresql/data` (named volume for persistence)
     - `./migrations:/docker-entrypoint-initdb.d` (mount migrations directory)
   - Networks: `timer-network`
   - Health check: `pg_isready -U timer -d timerdb` every 10s, timeout 5s, 5 retries
4. Define `timer` service:
   - Build: `.` (uses local Dockerfile)
   - Container name: `timer-platform`
   - Environment variables:
     - `DATABASE_URL: postgresql://timer:timer123@postgres:5432/timerdb`
     - `API_KEY: dev-api-key-change-in-production`
     - `PORT: 3000`
     - `RUST_LOG: info`
   - Ports: `3000:3000`
   - Depends on: `postgres` with condition `service_healthy`
   - Networks: `timer-network`
   - Restart policy: `unless-stopped`
5. Define networks:
   - `timer-network` with bridge driver
6. Define volumes:
   - `postgres_data` (named volume for database persistence)
7. Follow exact structure from CLAUDE.md specification

## Acceptance Criteria

- [ ] docker-compose.yml exists at project root
- [ ] PostgreSQL service configured with correct credentials
- [ ] Timer service depends on PostgreSQL health check
- [ ] Health check ensures PostgreSQL is ready before timer starts
- [ ] Migrations directory mounted into PostgreSQL container
- [ ] Database data persists in named volume
- [ ] Both services on same network for connectivity
- [ ] Timer service restarts automatically unless stopped
- [ ] Environment variables configured for local development
- [ ] Ports correctly mapped (5432 and 3000)

## Reference

See CLAUDE.md - "docker-compose.yml" section for complete configuration with health checks and volume mounts, and "Usage" section for startup commands.
