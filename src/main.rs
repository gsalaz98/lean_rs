/// ## lean_rs - LEAN Algorithmic Trading Platform in Rust.
/// 
/// An event driven backtester powered by Rust.
/// This crate is independent/unofficial and not related in any way to QuantConnect

/// Main definition of algorithms
#[deny(missing_docs)]
mod algorithm;
/// Algorithm Implementations by users
#[deny(missing_docs)]
mod algorithm_rust;
/// Brokerage implementations
#[deny(missing_docs)]
mod brokerages;
/// Core parts of the LEAN engine
#[deny(missing_docs)]
mod engine;
/// Market event types, such as quotes, trades, etc.
#[deny(missing_docs)]
pub mod data;
/// Misc. stuff in common
pub mod common;

fn main() {

}
