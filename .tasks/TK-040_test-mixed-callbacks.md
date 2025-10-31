# Task: Create Mixed Callback Integration Test

**Status**: pending
**Dependencies**: TK-038_create-http-callback-test.md, TK-039_create-nats-callback-test.md
**Estimated Effort**: small

## Objective

Create test scenario that verifies both HTTP and NATS callbacks work simultaneously in the same system.

## Context

The Timer Platform supports both HTTP and NATS callbacks running concurrently. We need to test that the scheduler correctly dispatches to both callback types, handles failures independently, and that filtering by callback_type works. This integration test ensures the dual-callback system works as a cohesive whole.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/.tasks/TK-040_mixed-callback-test.md` - Create test instructions

## Detailed Steps

1. Create comprehensive test scenario documentation
2. Include prerequisites:
   - Docker Compose fully running (postgres, nats, timer)
   - NATS subscriber active
   - Webhook.site or similar HTTP receiver ready
3. Create test scenario: 5 timers with mixed types
   ```bash
   # Timer 1: HTTP callback (near-term execution)
   EXEC_TIME_1=$(date -u -v+15S +"%Y-%m-%dT%H:%M:%SZ")
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d "{
       \"execute_at\": \"$EXEC_TIME_1\",
       \"callback_type\": \"http\",
       \"callback_config\": {
         \"type\": \"http\",
         \"url\": \"https://webhook.site/your-id\"
       }
     }"

   # Timer 2: NATS callback (near-term execution)
   EXEC_TIME_2=$(date -u -v+20S +"%Y-%m-%dT%H:%M:%SZ")
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d "{
       \"execute_at\": \"$EXEC_TIME_2\",
       \"callback_type\": \"nats\",
       \"callback_config\": {
         \"type\": \"nats\",
         \"topic\": \"events.test\",
         \"key\": \"timer2\"
       }
     }"

   # Timer 3: HTTP callback (failing URL)
   EXEC_TIME_3=$(date -u -v+25S +"%Y-%m-%dT%H:%M:%SZ")
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d "{
       \"execute_at\": \"$EXEC_TIME_3\",
       \"callback_type\": \"http\",
       \"callback_config\": {
         \"type\": \"http\",
         \"url\": \"https://localhost:9999/fail\"
       }
     }"

   # Timer 4: NATS callback (different topic)
   EXEC_TIME_4=$(date -u -v+30S +"%Y-%m-%dT%H:%M:%SZ")
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d "{
       \"execute_at\": \"$EXEC_TIME_4\",
       \"callback_type\": \"nats\",
       \"callback_config\": {
         \"type\": \"nats\",
         \"topic\": \"events.another\"
       }
     }"

   # Timer 5: HTTP callback (pending, far future)
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d '{
       "execute_at": "2025-12-31T23:59:59Z",
       "callback_type": "http",
       "callback_config": {
         "type": "http",
         "url": "https://webhook.site/your-id"
       }
     }'
   ```
4. Add filter testing:
   ```bash
   # List all timers
   curl -H "X-API-Key: dev-api-key-change-in-production" \
     "http://localhost:3000/timers"

   # List only HTTP timers
   curl -H "X-API-Key: dev-api-key-change-in-production" \
     "http://localhost:3000/timers?callback_type=http"

   # List only NATS timers
   curl -H "X-API-Key: dev-api-key-change-in-production" \
     "http://localhost:3000/timers?callback_type=nats"
   ```
5. Add verification checklist:
   - [ ] Timer 1 executes successfully, webhook receives HTTP POST
   - [ ] Timer 2 executes successfully, NATS subscriber receives message
   - [ ] Timer 3 fails with HTTP error in last_error field
   - [ ] Timer 4 executes successfully on different NATS topic
   - [ ] Timer 5 remains pending (far future)
   - [ ] Filtering by callback_type returns correct timers
   - [ ] All timers appear in /timers list with correct callback_type
   - [ ] Scheduler logs show both HTTP and NATS execution attempts
6. Add update test:
   ```bash
   # Update Timer 5 from HTTP to NATS
   curl -X PUT http://localhost:3000/timers/{TIMER_5_ID} \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d '{
       "callback_type": "nats",
       "callback_config": {
         "type": "nats",
         "topic": "events.updated"
       }
     }'
   ```
7. Document expected outcomes and common troubleshooting

## Acceptance Criteria

- [ ] Test scenario creates multiple timers of both types
- [ ] Instructions include filter testing
- [ ] Update test changes callback type
- [ ] Verification checklist covers all success/failure scenarios
- [ ] Test demonstrates independent failure handling
- [ ] Test verifies callback_type filtering works
- [ ] Instructions are reproducible and clear
- [ ] Expected logs and outputs are documented

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Testing" section for integration test requirements
