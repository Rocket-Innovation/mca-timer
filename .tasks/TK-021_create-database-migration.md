# Task: Create Database Migration for Callback Schema

**Status**: pending
**Dependencies**: TK-019_update-models-callback-types.md
**Estimated Effort**: medium

## Objective

Create a SQL migration file that transforms the current callback schema (callback_url, callback_headers, callback_payload) into the new flexible schema (callback_type, callback_config).

## Context

The database currently stores HTTP callbacks with separate columns. We need to consolidate this into a callback_type discriminator and callback_config JSONB column that can store either HTTP or NATS configurations. This migration must handle existing data by transforming it into the new format.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/migrations/002_add_callback_types.sql` - New migration file

## Detailed Steps

1. Create `/Users/tar/Documents/alpha/timer/migrations/002_add_callback_types.sql`
2. Add header comment explaining the migration purpose
3. Add new columns (without dropping old ones yet):
   ```sql
   ALTER TABLE timers
   ADD COLUMN callback_type VARCHAR(10),
   ADD COLUMN callback_config JSONB;
   ```
4. Migrate existing data from old schema to new schema:
   ```sql
   UPDATE timers SET
     callback_type = 'http',
     callback_config = jsonb_build_object(
       'type', 'http',
       'url', callback_url,
       'headers', COALESCE(callback_headers, 'null'::jsonb),
       'payload', COALESCE(callback_payload, 'null'::jsonb)
     );
   ```
5. Add NOT NULL constraints after data migration:
   ```sql
   ALTER TABLE timers
   ALTER COLUMN callback_type SET DEFAULT 'http',
   ALTER COLUMN callback_type SET NOT NULL,
   ALTER COLUMN callback_config SET NOT NULL;
   ```
6. Add CHECK constraint for valid callback types:
   ```sql
   ALTER TABLE timers
   ADD CONSTRAINT valid_callback_type CHECK (callback_type IN ('http', 'nats'));
   ```
7. Create index on callback_type for filtering:
   ```sql
   CREATE INDEX idx_timers_callback_type ON timers(callback_type);
   ```
8. Drop old columns (after data is migrated):
   ```sql
   ALTER TABLE timers
   DROP COLUMN callback_url,
   DROP COLUMN callback_headers,
   DROP COLUMN callback_payload;
   ```
9. Add comment documenting the new schema
10. Test the migration runs successfully with `sqlx migrate run`

## Acceptance Criteria

- [ ] Migration file 002_add_callback_types.sql is created in migrations/
- [ ] New columns callback_type and callback_config are added
- [ ] Existing timer data is migrated to new format (HTTP type)
- [ ] Old columns are dropped after migration
- [ ] CHECK constraint ensures only 'http' or 'nats' values
- [ ] Index on callback_type is created
- [ ] Migration runs successfully without errors
- [ ] Existing timers retain their callback configuration in new format

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Database Schema" section showing new timers table structure with callback_type and callback_config
