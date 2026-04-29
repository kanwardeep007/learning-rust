//! Lesson 1 — Why "plain `fn` pointers don't allow either".
//!
//! "Either" meaning:
//!   (a) capturing environment variables, and
//!   (b) being async (returning a future tied to captured state).
//!
//! Run with:
//!     cargo run --example 01_fn_pointer_vs_closure
//!
//! ----------------------------------------------------------------------------
//! Background
//! ----------------------------------------------------------------------------
//!
//! In Rust there are THREE different "callable" things, with three different
//! type spellings:
//!
//!   1. `fn(Args) -> Ret`           -- a function POINTER. Just an address.
//!                                     Cannot capture local state.
//!
//!   2. `impl Fn(Args) -> Ret`      -- generic over a CLOSURE TYPE. Each closure
//!      (or `F: Fn(...)`)              has its own anonymous type that includes
//!                                     the captured variables as fields.
//!
//!   3. `dyn Fn(Args) -> Ret`       -- a TYPE-ERASED closure behind a pointer.
//!      (usually `Box<dyn Fn...>`)     Useful when you need to store closures
//!                                     of *different* concrete types in the
//!                                     same collection (e.g. a HashMap).
//!
//! Plain `fn` pointers can be coerced from non-capturing closures, but they
//! cannot represent capturing closures at all. That's the limitation that
//! forces the kafka_events_consumer crate to use `dyn Fn(...)`.

fn run_with_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn run_with_generic_closure<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn run_with_dyn_closure(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    fn double(x: i32) -> i32 {
        x * 2
    }
    println!("(1) fn pointer: {}", run_with_fn_ptr(double, 5));

    let non_capturing = |x: i32| x * 2;
    println!(
        "(2) non-capturing closure as fn ptr: {}",
        run_with_fn_ptr(non_capturing, 5)
    );

    let factor: i32 = 7;
    let capturing = |x: i32| x * factor;

    println!(
        "(3) capturing closure (generic Fn): {}",
        run_with_generic_closure(capturing, 5)
    );


    let capturing2 = |x: i32| x * factor;
    println!(
        "(4) capturing closure (dyn Fn behind &): {}",
        run_with_dyn_closure(&capturing2, 5)
    );

    let boxed: Box<dyn Fn(i32) -> i32> = Box::new(move |x| x * factor);
    println!("(5) boxed dyn Fn: {}", boxed(5));

    println!();
    println!("Now uncomment the block below in the source file to see the");
    println!("compiler refuse a capturing closure as a `fn` pointer:");
    println!();
    println!("    // run_with_fn_ptr(capturing, 5);");
    println!();
    println!("error[E0308]: mismatched types");
    println!("expected fn pointer `fn(i32) -> i32`");
    println!("   found closure `{{closure@...}}`");
    println!("note: closures can only be coerced to `fn` types if they do not");
    println!("      capture any variables");

    
    
    
    

    
}
