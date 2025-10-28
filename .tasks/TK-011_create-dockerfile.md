# Task: Create Dockerfile for Production Build

**Status**: pending
**Dependencies**: None
**Estimated Effort**: small

## Objective

Create a multi-stage Dockerfile that builds an optimized production image for the timer platform.

## Context

The Dockerfile uses a multi-stage build to minimize the final image size. The builder stage compiles the Rust application with release optimizations, and the runtime stage creates a minimal Debian-based image with only the compiled binary and necessary runtime dependencies (CA certificates, OpenSSL). This approach significantly reduces the image size compared to including the full Rust toolchain in the runtime image.

## Files to Modify/Create

- `Dockerfile` - New multi-stage build configuration at project root

## Detailed Steps

1. Create `Dockerfile` at project root: `/Users/tar/Documents/alpha/timer/Dockerfile`
2. Define builder stage:
   - Base image: `rust:1.75-bookworm`
   - Set workdir: `/app`
   - Copy `Cargo.toml` and `Cargo.lock`
   - Copy `src/` directory
   - Copy `migrations/` directory (needed for SQLx compile-time checks)
   - Build release binary: `RUN cargo build --release`
3. Define runtime stage:
   - Base image: `debian:bookworm-slim`
   - Install runtime dependencies: `ca-certificates` and `libssl3` via apt-get
   - Clean up apt cache to reduce image size
   - Set workdir: `/app`
   - Copy binary from builder: `COPY --from=builder /app/target/release/timer /app/timer`
   - Expose port 3000
   - Set CMD: `["/app/timer"]`
4. Add comments explaining each stage
5. Follow exact structure from CLAUDE.md specification

## Acceptance Criteria

- [ ] Dockerfile exists at project root
- [ ] Uses multi-stage build with builder and runtime stages
- [ ] Builder stage based on rust:1.75-bookworm
- [ ] Runtime stage based on debian:bookworm-slim
- [ ] Installs ca-certificates and libssl3 in runtime stage
- [ ] Copies only compiled binary to runtime stage
- [ ] Exposes port 3000
- [ ] CMD runs /app/timer binary
- [ ] Apt cache cleaned to minimize image size
- [ ] migrations/ directory included in builder stage

## Reference

See CLAUDE.md - "Docker Setup" section for complete Dockerfile specification and build instructions.
