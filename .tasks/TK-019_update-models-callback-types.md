# Task: Update Models with Callback Types

**Status**: pending
**Dependencies**: TK-018_add-nats-dependency.md
**Estimated Effort**: small

## Objective

Add new callback type enums and structs to models.rs to support both HTTP and NATS callbacks.

## Context

The current models.rs contains only HTTP callback fields (callback_url, callback_headers, callback_payload). We need to introduce a flexible callback system using Rust enums that can represent either HTTP or NATS configurations. This uses internally-tagged serde serialization to store callback_type and callback_config in the database.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/models.rs` - Add CallbackType enum, HTTPCallback, NATSCallback, and CallbackConfig enum

## Detailed Steps

1. Open `/Users/tar/Documents/alpha/timer/src/models.rs`
2. Add new enum after TimerStatus:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
   #[sqlx(type_name = "text", rename_all = "lowercase")]
   pub enum CallbackType {
       Http,
       Nats,
   }
   ```
3. Add HTTPCallback struct:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct HTTPCallback {
       pub url: String,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub headers: Option<serde_json::Value>,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub payload: Option<serde_json::Value>,
   }
   ```
4. Add NATSCallback struct:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct NATSCallback {
       pub topic: String,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub key: Option<String>,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub headers: Option<serde_json::Value>,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub payload: Option<serde_json::Value>,
   }
   ```
5. Add CallbackConfig enum with internally-tagged serde:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   #[serde(tag = "type", rename_all = "lowercase")]
   pub enum CallbackConfig {
       Http(HTTPCallback),
       Nats(NATSCallback),
   }
   ```
6. Do NOT modify the Timer struct yet (that happens in TK-021 after migration)

## Acceptance Criteria

- [ ] CallbackType enum is defined with Http and Nats variants
- [ ] HTTPCallback struct contains url, headers, payload fields
- [ ] NATSCallback struct contains topic, key, headers, payload fields
- [ ] CallbackConfig enum uses #[serde(tag = "type")] for internally-tagged serialization
- [ ] All new types implement Debug, Clone, Serialize, Deserialize
- [ ] Code compiles without errors with `cargo check`

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Data Models" section, "Rust Models (in models.rs)"
