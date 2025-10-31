# Task: Create HTTP Callback Manual Test

**Status**: pending
**Dependencies**: TK-030_update-create-timer-api.md, TK-035_update-docker-compose-nats.md
**Estimated Effort**: small

## Objective

Create manual test instructions for HTTP callback functionality with the new callback_type and callback_config format.

## Context

With the new callback model, the HTTP callback request format has changed. We need to update the manual testing documentation to show how to create and test HTTP callbacks using the new structure. This ensures developers can verify HTTP functionality still works after the refactoring.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/.tasks/TK-038_http-callback-test.md` - Create test instructions (or update existing test checklist)

## Detailed Steps

1. Create test documentation file with HTTP callback examples
2. Include prerequisites:
   - Docker Compose running with `docker-compose up -d`
   - API_KEY environment variable set
   - Webhook.site URL for testing (or similar webhook receiver)
3. Create HTTP callback test case:
   ```bash
   # Create HTTP callback timer
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d '{
       "execute_at": "2025-10-30T16:00:00Z",
       "callback_type": "http",
       "callback_config": {
         "type": "http",
         "url": "https://webhook.site/your-unique-id",
         "headers": {
           "Authorization": "Bearer test-token",
           "X-Custom-Header": "test-value"
         },
         "payload": {
           "message": "Timer triggered via HTTP!",
           "timestamp": "2025-10-30T16:00:00Z"
         }
       },
       "metadata": {
         "test": "http-callback"
       }
     }'
   ```
4. Add verification steps:
   - Check response has callback_type: "http"
   - Get timer details and verify callback_config structure
   - Wait for execution time and verify webhook received POST request
   - Check timer status changed to "completed"
5. Add failure test case (invalid URL):
   ```bash
   # Create timer with failing callback
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: dev-api-key-change-in-production" \
     -H "Content-Type: application/json" \
     -d '{
       "execute_at": "2025-10-30T16:05:00Z",
       "callback_type": "http",
       "callback_config": {
         "type": "http",
         "url": "https://localhost:9999/nonexistent"
       }
     }'
   ```
6. Add validation test cases:
   - Mismatched callback_type and callback_config (should fail with 400)
   - Missing required fields (should fail with 400)
7. Document expected results for each test case

## Acceptance Criteria

- [ ] Test documentation includes HTTP callback creation example
- [ ] Request uses new callback_type and callback_config format
- [ ] Example shows how to include custom headers and payload
- [ ] Verification steps are clearly documented
- [ ] Failure test case is included
- [ ] Validation test cases are included
- [ ] Expected HTTP status codes and responses are documented

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Testing" → "Manual Testing Checklist" section for HTTP callback examples with new format
