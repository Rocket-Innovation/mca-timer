# Task: Manual Testing and Verification

**Status**: pending
**Dependencies**: TK-001, TK-002, TK-003, TK-004, TK-005, TK-006, TK-007, TK-008, TK-009, TK-010, TK-011, TK-012
**Estimated Effort**: medium

## Objective

Perform comprehensive manual testing of all API endpoints and scheduler functionality to verify the system works end-to-end.

## Context

After implementing all components, this task validates that the complete system functions correctly through manual testing. Following the test scenarios in CLAUDE.md, verify each API endpoint, test the scheduler's timer execution, and confirm error handling works as expected. Use curl commands to interact with the API and check database state directly when needed. Document any issues found for fixing before considering the implementation complete.

## Files to Modify/Create

No files to create - this is a testing task that validates existing implementation.

## Detailed Steps

1. Start the system: `docker-compose up -d`
2. Verify both containers are running: `docker-compose ps`
3. Check application logs for successful startup: `docker-compose logs timer`
4. Set API key for testing: `export API_KEY="dev-api-key-change-in-production"`

**Test 1: Health Check (Public Endpoint)**
5. Test health endpoint without auth:
   ```bash
   curl -X GET http://localhost:3000/health
   ```
   - Expected: HTTP 200, status="up", database="connected"

**Test 2: Authentication**
6. Test missing API key:
   ```bash
   curl -X GET http://localhost:3000/timers
   ```
   - Expected: HTTP 401, code=4, message="unauthorized"

7. Test invalid API key:
   ```bash
   curl -X GET http://localhost:3000/timers -H "X-API-Key: wrong-key"
   ```
   - Expected: HTTP 401, code=4

**Test 3: Create Timer**
8. Create a timer 60 seconds in the future:
   ```bash
   curl -X POST http://localhost:3000/timers \
     -H "X-API-Key: $API_KEY" \
     -H "Content-Type: application/json" \
     -d '{
       "execute_at": "'$(date -u -v+60S +"%Y-%m-%dT%H:%M:%SZ")'",
       "callback_url": "https://webhook.site/YOUR-UNIQUE-ID",
       "callback_payload": {"test": "data"}
     }'
   ```
   - Expected: HTTP 201, code=0, timer object with id and status="pending"
   - Save timer_id from response for subsequent tests

**Test 4: Get Timer**
9. Retrieve timer by ID:
   ```bash
   curl -X GET http://localhost:3000/timers/{TIMER_ID} \
     -H "X-API-Key: $API_KEY"
   ```
   - Expected: HTTP 200, complete timer details including callback_payload

**Test 5: List Timers**
10. List all pending timers:
    ```bash
    curl -X GET "http://localhost:3000/timers?status=pending" \
      -H "X-API-Key: $API_KEY"
    ```
    - Expected: HTTP 200, array containing created timer, pagination metadata

**Test 6: Update Timer**
11. Update timer's execute_at:
    ```bash
    curl -X PUT http://localhost:3000/timers/{TIMER_ID} \
      -H "X-API-Key: $API_KEY" \
      -H "Content-Type: application/json" \
      -d '{"execute_at": "'$(date -u -v+120S +"%Y-%m-%dT%H:%M:%SZ")'"}'
    ```
    - Expected: HTTP 200, updated timer with new execute_at

**Test 7: Cancel Timer**
12. Cancel the timer:
    ```bash
    curl -X DELETE http://localhost:3000/timers/{TIMER_ID} \
      -H "X-API-Key: $API_KEY"
    ```
    - Expected: HTTP 200, status="canceled"

13. Verify canceled timer cannot be updated:
    ```bash
    curl -X PUT http://localhost:3000/timers/{TIMER_ID} \
      -H "X-API-Key: $API_KEY" \
      -H "Content-Type: application/json" \
      -d '{"callback_url": "https://example.com"}'
    ```
    - Expected: HTTP 400, code=2, error about cannot update canceled timer

**Test 8: Timer Execution**
14. Create timer that executes in 10 seconds:
    ```bash
    curl -X POST http://localhost:3000/timers \
      -H "X-API-Key: $API_KEY" \
      -H "Content-Type: application/json" \
      -d '{
        "execute_at": "'$(date -u -v+10S +"%Y-%m-%dT%H:%M:%SZ")'",
        "callback_url": "https://webhook.site/YOUR-UNIQUE-ID",
        "callback_payload": {"message": "test execution"}
      }'
    ```
    - Save new timer_id

15. Wait 15 seconds for execution

16. Check timer status:
    ```bash
    curl -X GET http://localhost:3000/timers/{TIMER_ID} \
      -H "X-API-Key: $API_KEY"
    ```
    - Expected: status="completed", executed_at populated

17. Verify webhook receiver got the callback (check webhook.site)

**Test 9: Scheduler Cache Loading**
18. Check logs to verify Memory Loader is running:
    ```bash
    docker-compose logs timer | grep -i "loaded.*timers"
    ```
    - Expected: Log entries every 30 seconds showing timer cache updates

**Test 10: Validation Errors**
19. Create timer with execute_at in past:
    ```bash
    curl -X POST http://localhost:3000/timers \
      -H "X-API-Key: $API_KEY" \
      -H "Content-Type: application/json" \
      -d '{
        "execute_at": "2020-01-01T00:00:00Z",
        "callback_url": "https://example.com"
      }'
    ```
    - Expected: HTTP 400, code=2, validation error message

20. Create timer with invalid URL:
    ```bash
    curl -X POST http://localhost:3000/timers \
      -H "X-API-Key: $API_KEY" \
      -H "Content-Type: application/json" \
      -d '{
        "execute_at": "'$(date -u -v+60S +"%Y-%m-%dT%H:%M:%SZ")'",
        "callback_url": "not-a-url"
      }'
    ```
    - Expected: HTTP 400, code=2, validation error

**Test 11: Database Verification**
21. Connect to database and verify schema:
    ```bash
    docker-compose exec postgres psql -U timer -d timerdb -c "\d timers"
    ```
    - Expected: Table structure matches schema with all indexes

22. Query timers directly:
    ```bash
    docker-compose exec postgres psql -U timer -d timerdb -c "SELECT id, status, execute_at FROM timers;"
    ```
    - Expected: See created timers with correct statuses

**Test 12: Cleanup**
23. Stop services: `docker-compose down`
24. Optional: Remove volumes to reset: `docker-compose down -v`

## Acceptance Criteria

- [ ] Health check endpoint works without authentication
- [ ] All protected endpoints require valid X-API-Key header
- [ ] Can create timer with valid future execute_at
- [ ] Can retrieve timer by ID with full details
- [ ] Can list timers with filtering and pagination
- [ ] Can update pending timer
- [ ] Cannot update canceled/completed timers
- [ ] Can cancel pending timer
- [ ] Timer executes automatically at specified time
- [ ] Executed timer marked as "completed" with executed_at timestamp
- [ ] Webhook receiver gets callback with correct payload
- [ ] Memory Loader logs show cache updates every 30s
- [ ] Validation errors return HTTP 400 with code 2
- [ ] Past execute_at rejected
- [ ] Invalid URLs rejected
- [ ] Database schema matches specification
- [ ] All API responses follow ApiResponse<T> format

## Reference

See CLAUDE.md - "Testing" section with "Manual Testing Checklist" for complete test scenarios and expected results. Also review "API Endpoints" section for request/response formats.
