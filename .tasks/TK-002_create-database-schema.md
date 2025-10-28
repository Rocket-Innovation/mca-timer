# Task: Create Database Schema Migration

**Status**: pending
**Dependencies**: TK-001
**Estimated Effort**: small

## Objective

Create the SQL migration file that defines the timers table schema with all necessary indexes, constraints, and triggers.

## Context

This migration creates the core `timers` table that serves as the persistent storage for all timer configurations. The table includes fields for callback configuration (URL, method, headers, payload), execution tracking (status, executed_at, last_error), and metadata. Performance indexes are critical for the scheduler queries. The schema uses PostgreSQL-specific features like JSONB, UUIDv4 generation, and CHECK constraints.

## Files to Modify/Create

- `migrations/20250101000000_create_timers_table.sql` - Main migration file with complete schema

## Detailed Steps

1. Create file `migrations/20250101000000_create_timers_table.sql`
2. Add CREATE TABLE statement for `timers` with these columns:
   - `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`
   - `created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
   - `updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
   - `execute_at TIMESTAMPTZ NOT NULL`
   - `callback_url TEXT NOT NULL`
   - `callback_method VARCHAR(10) NOT NULL DEFAULT 'POST'`
   - `callback_headers JSONB`
   - `callback_payload JSONB`
   - `status VARCHAR(20) NOT NULL DEFAULT 'pending'`
   - `last_error TEXT`
   - `executed_at TIMESTAMPTZ`
   - `metadata JSONB`
3. Add CHECK constraints:
   - `valid_status CHECK (status IN ('pending', 'executing', 'completed', 'failed', 'canceled'))`
   - `valid_method CHECK (callback_method IN ('POST', 'PUT', 'PATCH'))`
   - `future_execute_at CHECK (execute_at > created_at)`
4. Create three indexes:
   - `idx_timers_execute_at_status ON timers(execute_at, status) WHERE status = 'pending'` (partial index for scheduler)
   - `idx_timers_status ON timers(status)` (for filtering)
   - `idx_timers_created_at ON timers(created_at DESC)` (for listing)
5. Create trigger function `update_updated_at_column()` that sets `NEW.updated_at = NOW()`
6. Create trigger `update_timers_updated_at` that calls the function BEFORE UPDATE

## Acceptance Criteria

- [ ] Migration file exists with correct timestamp naming
- [ ] All 12 columns defined with correct types and constraints
- [ ] Three CHECK constraints prevent invalid data
- [ ] Three indexes created for query performance
- [ ] Trigger automatically updates `updated_at` on row changes
- [ ] Schema matches specification in CLAUDE.md exactly

## Reference

See CLAUDE.md - "Database Schema (PostgreSQL)" section for complete schema definition with all field descriptions and index purposes.
