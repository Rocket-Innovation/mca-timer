# Task: Verify and Update .gitignore

**Status**: pending
**Dependencies**: None
**Estimated Effort**: small

## Objective

Ensure .gitignore is properly configured to exclude build artifacts, dependencies, environment files, and IDE-specific files.

## Context

The .gitignore file already exists in the project. This task verifies it includes all necessary patterns to prevent committing sensitive data (like .env files with credentials), build artifacts (target/ directory), and IDE configurations. Following Rust best practices, the file should ignore Cargo's build outputs and local configuration while keeping tracked files like .env.example.

## Files to Modify/Create

- `.gitignore` - Verify and update if needed

## Detailed Steps

1. Open existing `.gitignore` at `/Users/tar/Documents/alpha/timer/.gitignore`
2. Verify it includes Rust-specific patterns:
   - `/target/` - Cargo build directory
   - `Cargo.lock` should be committed (is a binary project)
   - `**/*.rs.bk` - Backup files from rustfmt
3. Verify it includes environment and secrets:
   - `.env` - Local environment variables (contains secrets)
   - `.env.local` - Alternative local env file
   - Do NOT ignore `.env.example` (should be tracked)
4. Verify it includes IDE and editor files:
   - `.vscode/` or `.vscode/*` if not using shared settings
   - `.idea/` for IntelliJ
   - `*.swp`, `*.swo` for Vim
   - `.DS_Store` for macOS
5. Add SQLx offline data (optional):
   - `sqlx-data.json` if using offline mode for CI
6. Verify `.tasks/` directory is NOT ignored (tasks should be tracked)

## Acceptance Criteria

- [ ] .gitignore exists and is properly formatted
- [ ] `/target/` directory ignored (Cargo build artifacts)
- [ ] `.env` file ignored (contains secrets)
- [ ] `.env.example` NOT ignored (template should be tracked)
- [ ] Common IDE files ignored (.vscode/, .idea/, .DS_Store)
- [ ] Backup files ignored (*.rs.bk, *.swp)
- [ ] No overly broad patterns that might ignore important files
- [ ] `.tasks/` directory NOT ignored

## Reference

Standard Rust .gitignore practices, and CLAUDE.md implies .env contains secrets (API_KEY) which should never be committed.
