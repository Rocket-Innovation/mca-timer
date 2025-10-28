# Task: Update Environment Variables Example File

**Status**: pending
**Dependencies**: None
**Estimated Effort**: small

## Objective

Update the .env.example file to include all required and optional environment variables with descriptive comments and example values.

## Context

The .env.example file serves as documentation for developers setting up the application locally. It shows all environment variables the application expects, provides example values, and explains the purpose of each variable. Developers copy this file to .env and customize the values for their local environment. The file already exists but may need updates to match the complete specification.

## Files to Modify/Create

- `.env.example` - Update existing file with complete variable documentation

## Detailed Steps

1. Open existing `.env.example` file at `/Users/tar/Documents/alpha/timer/.env.example`
2. Verify it includes all required variables:
   - `DATABASE_URL` with example PostgreSQL connection string
   - `API_KEY` with example 32+ character key
3. Verify it includes all optional variables with defaults:
   - `PORT` (default: 3000)
   - `RUST_LOG` (default: info)
4. Add comments explaining each variable:
   - Database connection string format
   - API key minimum length requirement (32 chars)
   - Port configuration for HTTP server
   - Logging level options (trace, debug, info, warn, error)
5. Ensure example values are safe (no production credentials)
6. Format for readability with section comments
7. Match the exact format from CLAUDE.md specification

## Acceptance Criteria

- [ ] All four environment variables documented
- [ ] DATABASE_URL includes complete connection string format example
- [ ] API_KEY example is at least 32 characters
- [ ] PORT and RUST_LOG shown with their defaults
- [ ] Comments explain purpose and format of each variable
- [ ] No sensitive/production values included
- [ ] File format matches CLAUDE.md specification
- [ ] Sections clearly separated with comments

## Reference

See CLAUDE.md - "Environment Variables" section for complete list of required and optional variables with descriptions, and "Example .env file" for exact format.
