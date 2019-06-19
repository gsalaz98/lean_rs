/// ## LEAN.Rs- LEAN Algorithmic Trading Platform in Rust.
/// 
/// An event driven backtester powered by Rust.
/// This crate is independent/unofficial and not related in any way to QuantConnect

/// Brokerage implementations
mod brokerages;
/// Core parts of the LEAN engine
mod engine;
/// Market event types, such as quotes, trades, etc.
pub mod data;

#[deny(missing_docs)]
#[warn(clippy::all)]

fn main() {

}
