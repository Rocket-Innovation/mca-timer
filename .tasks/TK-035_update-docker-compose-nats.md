# Task: Add NATS Service to Docker Compose

**Status**: pending
**Dependencies**: TK-024_initialize-nats-in-main.md
**Estimated Effort**: small

## Objective

Add NATS server service to docker-compose.yml for local development and testing.

## Context

For local development, developers need a NATS server to test NATS callback functionality. We'll add the official NATS Docker image to docker-compose.yml alongside PostgreSQL. The timer service will connect to NATS using the NATS_URL environment variable. NATS should start before the timer service.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/docker-compose.yml` - Add NATS service

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/docker-compose.yml`
2. Add NATS service definition after the postgres service:
   ```yaml
   nats:
     image: nats:2.10-alpine
     container_name: timer-nats
     ports:
       - "4222:4222"
       - "8222:8222"
     command: ["-js", "-m", "8222"]
     networks:
       - timer-network
     healthcheck:
       test: ["CMD", "wget", "--spider", "-q", "http://localhost:8222/healthz"]
       interval: 10s
       timeout: 5s
       retries: 5
   ```
3. Update the timer service environment variables to include NATS_URL:
   ```yaml
   NATS_URL: nats://nats:4222
   ```
4. Update timer service depends_on to include NATS:
   ```yaml
   depends_on:
     postgres:
       condition: service_healthy
     nats:
       condition: service_healthy
   ```
5. Add comments explaining NATS port 4222 (client) and 8222 (monitoring)
6. Ensure the timer-network is shared across all services
7. Test with `docker-compose up -d` to ensure services start correctly

## Acceptance Criteria

- [ ] NATS service is defined in docker-compose.yml
- [ ] NATS uses official nats:2.10-alpine image
- [ ] Port 4222 is exposed for client connections
- [ ] Port 8222 is exposed for HTTP monitoring
- [ ] JetStream is enabled with -js flag
- [ ] Health check verifies NATS is responsive
- [ ] Timer service has NATS_URL environment variable set
- [ ] Timer service depends on NATS health check
- [ ] Services start in correct order: postgres, nats, then timer
- [ ] `docker-compose up -d` succeeds without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Docker Setup" section and "Environment Variables" for NATS_URL configuration
