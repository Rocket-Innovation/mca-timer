# Task: Implement Configuration Module

**Status**: pending
**Dependencies**: None
**Estimated Effort**: small

## Objective

Create the configuration module that loads and validates environment variables required for the application.

## Context

The `config.rs` module centralizes all environment variable loading and validation. It reads from `.env` files (via dotenvy) and provides a typed `Config` struct that is shared across the application. The module must fail fast on startup if required variables are missing or invalid, following the fail-fast philosophy. Configuration includes database connection, API authentication, server port, and logging level.

## Files to Modify/Create

- `src/config.rs` - New configuration module

## Detailed Steps

1. Create `src/config.rs` file
2. Import dependencies: `dotenvy`, `std::env`, `anyhow`
3. Define `Config` struct with fields:
   - `database_url: String` (required)
   - `api_key: String` (required, minimum 32 chars)
   - `port: u16` (optional, default 3000)
   - `rust_log: String` (optional, default "info")
4. Implement `Config::from_env()` method that:
   - Calls `dotenvy::dotenv().ok()` to load .env file (ignore if missing)
   - Reads `DATABASE_URL` from env, returns error if missing
   - Reads `API_KEY` from env, validates length >= 32 chars, returns error if invalid
   - Reads `PORT` from env, parses to u16, defaults to 3000
   - Reads `RUST_LOG` from env, defaults to "info"
   - Returns `Result<Config, anyhow::Error>`
5. Add validation helper method `validate_database_url()` that checks URL starts with "postgresql://"
6. Add helpful error messages for each validation failure

## Acceptance Criteria

- [ ] `Config` struct defined with all four fields
- [ ] `from_env()` method loads and validates all environment variables
- [ ] Missing `DATABASE_URL` returns error with clear message
- [ ] API key shorter than 32 characters returns error
- [ ] Invalid database URL format returns error
- [ ] Optional fields have correct defaults (port=3000, rust_log="info")
- [ ] Module can be imported and used in main.rs

## Reference

See CLAUDE.md - "Environment Variables" section for required and optional variables, and "Application Initialization" section for startup sequence including configuration loading.
