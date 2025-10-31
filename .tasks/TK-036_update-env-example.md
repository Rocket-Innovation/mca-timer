# Task: Update .env.example with NATS_URL

**Status**: pending
**Dependencies**: TK-020_add-nats-config.md
**Estimated Effort**: small

## Objective

Add NATS_URL configuration to .env.example file with documentation.

## Context

The .env.example file serves as a template for developers setting up their local environment. With NATS support added, we need to document the optional NATS_URL parameter. This helps developers understand they can enable NATS callbacks by configuring this variable.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/.env.example` - Add NATS_URL with comment

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/.env.example` (create if it doesn't exist)
2. Add NATS_URL configuration after DATABASE_URL section:
   ```env
   # PostgreSQL connection string (required)
   DATABASE_URL=postgresql://timer:timer123@localhost:5432/timerdb

   # NATS connection URL (optional - enables NATS callbacks)
   # If not set, only HTTP callbacks are supported
   NATS_URL=nats://localhost:4222

   # API authentication key (required, minimum 32 characters recommended)
   API_KEY=dev-api-key-change-in-production-at-least-32-chars-long

   # HTTP server port (optional, default: 3000)
   PORT=3000

   # Logging level (optional, default: info)
   # Options: trace, debug, info, warn, error
   RUST_LOG=info
   ```
3. Add explanatory comment that NATS_URL is optional
4. Show example connection string format: `nats://host:port`
5. Mention that docker-compose uses `nats://nats:4222` (container name)
6. Ensure all existing variables remain documented

## Acceptance Criteria

- [ ] .env.example includes NATS_URL variable
- [ ] NATS_URL is commented as optional
- [ ] Example value shows correct format (nats://localhost:4222)
- [ ] Comment explains NATS callbacks are disabled if not set
- [ ] All other environment variables remain documented
- [ ] File follows consistent formatting
- [ ] Docker Compose connection string is mentioned in comments

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Environment Variables" section listing NATS_URL as optional parameter
