//! Lesson 2 — Why every `async {}` block has its own ANONYMOUS type, and why
//! that means you cannot store two of them in the same `Vec` without boxing.
//!
//! Run with:
//!     cargo run --example 02_async_block_types
//!
//! ----------------------------------------------------------------------------
//! Background
//! ----------------------------------------------------------------------------
//!
//! When you write `async { 1 }`, the compiler does NOT produce a value of a
//! type you can name. It produces a value of an anonymous, compiler-generated
//! state-machine struct (something internally like
//! `__Future_Generated_At_File_Foo_Line_42`). That struct implements
//! `Future<Output = i32>`, but its concrete name is unnameable in source code.
//!
//! Two different `async {}` blocks — even with identical bodies — produce
//! DIFFERENT anonymous types. That's why this won't compile:
//!
//!     let f1 = async { 1 };
//!     let f2 = async { 2 };
//!     let v = vec![f1, f2];   // type mismatch
//!
//! `f1` and `f2` are not the same type. `Vec<T>` needs a single `T`.
//!
//! The kafka_events_consumer crate has the same problem at scale:
//! every call to `.handler(topic, h)` produces a closure with a unique type
//! (because the captured `handler` and `codec` are different each time).
//! It needs to store all of them in `HashMap<String, ???>`. The fix is to
//! erase the concrete types behind a uniform pointer: `dyn Fn(...) -> ...`
//! and (for the futures the closures return) `Pin<Box<dyn Future<...>>>`.
//!
//! This file shows the problem first, then the fix.

use std::future::Future;
use std::pin::Pin;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    
    
    

    let f1 = async { 1_i32 };
    let f2 = async { 2_i32 };

    
    
    
    
    
    
    
    
    
    
    

    println!("(1) f1.await = {}", f1.await);
    println!("(1) f2.await = {}", f2.await);

    
    
    

    type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

    fn make_one() -> BoxFuture<i32> {
        Box::pin(async { 1 })
    }
    fn make_two() -> BoxFuture<i32> {
        Box::pin(async {
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            2
        })
    }

    let futures: Vec<BoxFuture<i32>> = vec![make_one(), make_two()];

    for (i, f) in futures.into_iter().enumerate() {
        let v = f.await;
        println!("(2) futures[{i}].await = {v}");
    }

    
    
    
    
    
    

    println!();
    println!("Note: make_one and make_two return DIFFERENT concrete future");
    println!("types (the second contains a `Sleep`, the first does not), but");
    println!("by boxing them as `Pin<Box<dyn Future<Output = i32>>>` they");
    println!("become a uniform type the Vec accepts. Same trick the kafka");
    println!("library uses to store every topic's dispatcher in one HashMap.");
}
