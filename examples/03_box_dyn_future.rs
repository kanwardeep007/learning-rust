//! Lesson 3 — Why must the future be on the HEAP, behind a pointer of known
//! size? In other words: why `Box<dyn Future<...>>`?
//!
//! Run with:
//!     cargo run --example 03_box_dyn_future
//!
//! ----------------------------------------------------------------------------
//! Background
//! ----------------------------------------------------------------------------
//!
//! Every value in Rust must have a size known at compile time when it is
//! stored on the stack or as a struct field. Fact: `dyn Trait` types do NOT
//! have a known size — the trait could be implemented by an `i32` (4 bytes)
//! or by a 1-megabyte struct.
//!
//! That's why you cannot say:
//!
//!     fn foo() -> dyn Future<Output = i32> { ... }    // ERROR
//!     let v: Vec<dyn Future<Output = i32>> = ...;     // ERROR
//!
//! The fix is to put the unsized `dyn Trait` value somewhere on the heap and
//! refer to it by a thin pointer:
//!
//!     fn foo() -> Box<dyn Future<Output = i32>> { ... }   // OK, returns 1 ptr
//!     let v: Vec<Box<dyn Future<Output = i32>>> = ...;    // OK, vec of ptrs
//!
//! `Box<T>` is always pointer-sized regardless of what T is. So putting a
//! `dyn Trait` inside a `Box` solves the "unknown size" problem at the cost
//! of one heap allocation per future.
//!
//! Why heap and not just `&dyn Future`? Because the future has to OUTLIVE
//! the call site that produced it (for example: built inside a closure,
//! returned, then awaited later). References would tie its lifetime to a
//! particular stack frame.
//!
//! This example demonstrates BOTH points:
//!   (a) two futures with different sizes, stored uniformly via Box.
//!   (b) a future that outlives the function that built it.

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

fn small_future() -> BoxFuture<&'static str> {
    
    
    Box::pin(async { "small" })
}

fn big_future() -> BoxFuture<&'static str> {
    
    
    
    
    Box::pin(async {
        let _buffer = [0u8; 4096];
        tokio::time::sleep(Duration::from_millis(1)).await;
        "big"
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    
    
    
    

    let factory: Vec<fn() -> BoxFuture<&'static str>> = vec![small_future, big_future];

    for make in factory {
        
        
        
        let fut: BoxFuture<&'static str> = make();
        let val = fut.await;
        println!("got value: {val}");
    }

    
    
    
    

    let f = build_future_far_away();
    let result = f.await;
    println!("from far-away factory: {result}");
}

fn build_future_far_away() -> BoxFuture<i32> {
    let captured = 100;
    Box::pin(async move {
        tokio::time::sleep(Duration::from_millis(1)).await;
        captured + 1
    })
}
