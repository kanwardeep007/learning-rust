//! Lesson 4 — Why does the future need to be PINNED before polling?
//!
//! Run with:
//!     cargo run --example 04_why_pin
//!
//! ----------------------------------------------------------------------------
//! Background
//! ----------------------------------------------------------------------------
//!
//! When you write:
//!
//!     async fn read_some() -> usize {
//!         let buf = [0u8; 8];
//!         let slice = &buf[..];
//!         do_io(slice).await;
//!         slice.len()
//!     }
//!
//! the compiler turns the body into an anonymous state-machine struct that
//! holds, across the .await, BOTH `buf` and `slice`. Because `slice` is
//! `&buf[..]`, the struct contains a field that points INTO ANOTHER FIELD OF
//! THE SAME STRUCT. That's a "self-referential" type.
//!
//! Self-referential types are dangerous: if the struct is moved in memory
//! (Rust normally allows free moves), the inner reference becomes a dangling
//! pointer to the old location. That's undefined behaviour.
//!
//! Rust's solution: `Pin<P>` is a wrapper around a pointer `P` (e.g. `&mut T`,
//! `Box<T>`) that statically promises the pointee will never move again. The
//! Future trait is declared as:
//!
//!     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
//!
//! …meaning you can ONLY poll a future once it has been pinned. Pinning is
//! the type-system handshake that says "OK, the future's address is now
//! frozen, it is safe for the future to hold internal references."
//!
//! `Box::pin(future)` is the easy way to pin: heap-allocate the future
//! (giving it a stable address) and wrap the resulting `Box<F>` in a `Pin`.
//!
//! You ALMOST never call `.poll()` directly. The `.await` keyword does it
//! for you, on a future that the executor has already pinned. We do it
//! manually below for educational purposes.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn main() {
    
    let fut = async {
        21 + 21
    };

    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut pinned: Pin<Box<_>> = Box::pin(fut);

    
    
    
    
    match pinned.as_mut().poll(&mut cx) {
        Poll::Ready(v) => println!("future returned: {v}"),
        Poll::Pending => println!("future not yet ready"),
    }

    
    
    
    
    
    println!();
    println!("Why a noop Waker? Polling normally REQUIRES that, if the future");
    println!("is not ready, it has stored the Waker somewhere so it can wake");
    println!("the executor when work completes. Our toy future returns Ready");
    println!("on the very first poll, so it never needs to wake anyone — a");
    println!("noop is fine. Real executors (tokio, etc.) build proper Wakers.");
}
