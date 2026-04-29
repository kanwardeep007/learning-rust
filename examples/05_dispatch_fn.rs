//! Lesson 5 — Putting it all together: a toy clone of `kafka_events_consumer`'s
//! `DispatchFn` machinery.
//!
//! Run with:
//!     cargo run --example 05_dispatch_fn
//!
//! ----------------------------------------------------------------------------
//! What this demonstrates
//! ----------------------------------------------------------------------------
//!
//!   - A `MessageHandler` trait the application implements (one per event type).
//!   - A `Decode` trait to turn raw bytes into a typed event (the codec).
//!   - A `DispatchFn` type alias = "Arc + dyn Fn + boxed/pinned future" — i.e.
//!     a single uniform "callable with bytes" type that can hold any handler
//!     for any topic.
//!   - A `Registry` that stores `HashMap<String, DispatchFn>` keyed by topic.
//!   - `register(topic, handler)` builds a closure (an "adapter") that:
//!         bytes -> decode -> handler.handle -> uniform Result type.
//!   - A `dispatch(topic, bytes)` method that simulates one Kafka message
//!     arriving on a topic — happy path, decode error, and handler error.
//!
//! All the moving parts in one ~120-line file, no external Kafka or Protobuf.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Debug)]
#[allow(dead_code)]
enum DispatchError {
    Decode(String),
    Handler(String),
    UnknownTopic(String),
}

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

type DispatchFn =
    Arc<dyn Fn(Vec<u8>) -> BoxFuture<Result<(), DispatchError>> + Send + Sync + 'static>;

trait MessageHandler: Send + Sync + 'static {
    type Event: Send + 'static;
    fn handle(&self, event: Self::Event) -> impl Future<Output = Result<(), String>> + Send;
}

trait Decode: Sized + Send + 'static {
    fn decode(bytes: &[u8]) -> Result<Self, String>;
}

#[derive(Debug)]
struct OrderCreated {
    id: u64,
}

impl Decode for OrderCreated {
    fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != 8 {
            return Err(format!("expected 8 bytes, got {}", bytes.len()));
        }
        let arr: [u8; 8] = bytes.try_into().expect("length checked above");
        Ok(OrderCreated {
            id: u64::from_le_bytes(arr),
        })
    }
}

struct OrderHandler {
    name: &'static str,
}

impl MessageHandler for OrderHandler {
    type Event = OrderCreated;

    async fn handle(&self, event: OrderCreated) -> Result<(), String> {
        if event.id == 0 {
            return Err(format!("[{}] zero id is invalid", self.name));
        }
        println!("[{}] processed order id={}", self.name, event.id);
        Ok(())
    }
}

struct Registry {
    handlers: HashMap<String, DispatchFn>,
}

impl Registry {
    fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    fn register<H>(&mut self, topic: &str, handler: H)
    where
        H: MessageHandler + 'static,
        H::Event: Decode,
    {
        
        
        let handler = Arc::new(handler);

        
        
        
        let dispatch: DispatchFn = Arc::new(move |bytes: Vec<u8>| {
            
            
            let handler = Arc::clone(&handler);

            
            
            Box::pin(async move {
                let event = <H::Event as Decode>::decode(&bytes).map_err(DispatchError::Decode)?;
                handler
                    .handle(event)
                    .await
                    .map_err(DispatchError::Handler)
            })
        });

        self.handlers.insert(topic.to_string(), dispatch);
    }

    async fn dispatch(&self, topic: &str, bytes: Vec<u8>) -> Result<(), DispatchError> {
        let f = self
            .handlers
            .get(topic)
            .ok_or_else(|| DispatchError::UnknownTopic(topic.to_string()))?;
        f(bytes).await
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut registry = Registry::new();
    registry.register(
        "orders.created",
        OrderHandler {
            name: "orders-handler",
        },
    );

    
    println!("--- happy path ---");
    let bytes = 42u64.to_le_bytes().to_vec();
    let result = registry.dispatch("orders.created", bytes).await;
    println!("result = {result:?}\n");

    
    println!("--- handler error path (id == 0) ---");
    let bytes = 0u64.to_le_bytes().to_vec();
    let result = registry.dispatch("orders.created", bytes).await;
    println!("result = {result:?}\n");

    
    println!("--- decode error path (wrong length) ---");
    let bytes = vec![1, 2, 3];
    let result = registry.dispatch("orders.created", bytes).await;
    println!("result = {result:?}\n");

    
    println!("--- unknown topic path ---");
    let result = registry.dispatch("not.registered", vec![]).await;
    println!("result = {result:?}");
}
