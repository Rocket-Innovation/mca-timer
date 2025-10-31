# Task: Create NATS Callback Manual Test

**Status**: pending
**Dependencies**: TK-030_update-create-timer-api.md, TK-035_update-docker-compose-nats.md
**Estimated Effort**: small

## Objective

Create manual test instructions for NATS callback functionality, including subscriber setup and message verification.

## Context

NATS callbacks are new functionality that needs comprehensive testing documentation. Unlike HTTP callbacks that can be tested with webhook.site, NATS requires setting up a subscriber to receive messages. We need to provide clear instructions for creating NATS timers and verifying message delivery.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/.tasks/TK-039_nats-callback-test.md` - Create test instructions

## Detailed Steps

1. Create test documentation file with NATS callback examples
2. Include prerequisites:
   - Docker Compose running with NATS service
   - NATS CLI installed for testing (or use nats Docker image)
   - API_KEY environment variable set
3. Create NATS subscriber setup instructions:
   ```bash
   # Subscribe to test topic using NATS CLI
   nats sub "events.timer.>" --server=nats://localhost:4222

   # Or using Docker:
   docker run --rm --network timer-network -it natsio/nats-box:latest nats sub "events.timer.>" --server=nats://nats:4222
   ```
4. Create NATS callback test case:
   ```bash
   # Create NATS callback timer
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d '{
       "execute_at": "2025-10-30T16:10:00Z",
       "callback_type": "nats",
       "callback_config": {
         "type": "nats",
         "topic": "events.timer",
         "key": "test-key",
         "headers": {
           "X-Timer-ID": "test-123",
           "X-Source": "timer-platform"
         },
         "payload": {
           "message": "Timer triggered via NATS!",
           "timestamp": "2025-10-30T16:10:00Z"
         }
       },
       "metadata": {
         "test": "nats-callback"
       }
     }'
   ```
5. Add verification steps:
   - Check response has callback_type: "nats"
   - Get timer details and verify callback_config structure
   - Wait for execution time
   - Verify NATS subscriber receives message on topic "events.timer.test-key"
   - Verify message headers and payload are correct
   - Check timer status changed to "completed"
6. Add test without NATS URL configured:
   - Stop timer service
   - Remove NATS_URL from environment
   - Restart timer service
   - Try creating NATS timer (should succeed)
   - Wait for execution (should fail with "NATS client not available")
7. Add validation test cases:
   - Empty topic (should fail with 400)
   - Mismatched callback_type and callback_config (should fail with 400)
8. Document expected results for each test case

## Acceptance Criteria

- [ ] Test documentation includes NATS subscriber setup
- [ ] NATS callback creation example uses new format
- [ ] Example shows topic, key, headers, and payload
- [ ] Verification steps explain how to monitor NATS messages
- [ ] Test case for NATS unavailable scenario
- [ ] Validation test cases are included
- [ ] Expected behaviors and error messages are documented
- [ ] Instructions work with Docker Compose setup

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Testing" section for NATS callback testing examples
