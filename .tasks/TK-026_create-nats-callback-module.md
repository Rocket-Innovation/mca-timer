# Task: Create NATS Callback Module

**Status**: pending
**Dependencies**: TK-023_add-nats-client-to-appstate.md, TK-022_update-timer-model-struct.md
**Estimated Effort**: small

## Objective

Create new callback_nats.rs module that handles NATS message publishing for timer callbacks.

## Context

NATS callbacks are simpler than HTTP - they publish a JSON message to a specified topic in a fire-and-forget manner. The NATS client is obtained from AppState (which was initialized in main.rs). If no NATS client is available but a NATS callback is requested, it's an error condition. Optional headers and key can be included in the NATS message.

## Files to Modify/Create

- `/Users/tar/Documents/alpha/timer/src/callback_nats.rs` - New module for NATS callbacks

## Detailed Steps

1. Create `/Users/tar/Documents/alpha/timer/src/callback_nats.rs`
2. Add module header documentation:
   ```rust
   //! NATS callback execution module
   //! Handles fire-and-forget message publishing to NATS topics
   ```
3. Add necessary imports:
   ```rust
   use crate::models::{NATSCallback, Timer};
   use async_nats::Client as NatsClient;
   use tracing::{info, warn};
   ```
4. Create the execute_nats_callback function:
   ```rust
   pub async fn execute_nats_callback(
       timer: &Timer,
       nats_config: &NATSCallback,
       nats_client: &NatsClient,
   ) -> Result<(), String> {
       // Build message payload
       let payload = if let Some(payload_value) = &nats_config.payload {
           serde_json::to_vec(payload_value)
               .map_err(|e| format!("Failed to serialize payload: {}", e))?
       } else {
           // Empty payload if none provided
           Vec::new()
       };

       // Build NATS subject (topic + optional key)
       let subject = if let Some(key) = &nats_config.key {
           format!("{}.{}", nats_config.topic, key)
       } else {
           nats_config.topic.clone()
       };

       // Create message with headers if present
       let mut message = async_nats::Message::new(subject.clone(), payload.into());

       if let Some(headers_value) = &nats_config.headers {
           if let Some(headers_obj) = headers_value.as_object() {
               let mut nats_headers = async_nats::HeaderMap::new();
               for (key, value) in headers_obj {
                   if let Some(val_str) = value.as_str() {
                       nats_headers.insert(key.as_str(), val_str);
                   }
               }
               message.headers = Some(nats_headers);
           }
       }

       // Publish message (fire-and-forget)
       match nats_client.publish_with_headers(
           message.subject,
           message.headers.unwrap_or_default(),
           message.payload,
       ).await {
           Ok(_) => {
               info!("NATS callback succeeded for timer {}: published to {}", timer.id, subject);
               Ok(())
           }
           Err(e) => {
               let error = format!("NATS publish failed: {}", e);
               warn!("NATS callback failed for timer {}: {}", timer.id, error);
               Err(error)
           }
       }
   }
   ```
5. Run `cargo check` to ensure the module compiles

## Acceptance Criteria

- [ ] callback_nats.rs file is created in src/
- [ ] execute_nats_callback function accepts Timer, NATSCallback, and NatsClient references
- [ ] Function builds NATS subject from topic + optional key
- [ ] Payload is serialized to JSON bytes
- [ ] Custom headers are added to NATS message if present
- [ ] Message is published with publish_with_headers
- [ ] Success returns Ok(()), errors return Err with error message
- [ ] Appropriate tracing logs for success and failure
- [ ] Code compiles without errors

## Reference

See `/Users/tar/Documents/alpha/timer/CLAUDE.md` - "Callback Execution" section for NATS message construction details
