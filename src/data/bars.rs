use crate::data::*;

/// Piece of data that can contain market information
pub trait Bar {}

/// Market data for equities that contain no bid/ask information
pub struct TradeBar {}
/// Market data that contains bid/ask information, but potentially no OHLCV
pub struct QuoteBar {}
/// Event data. This is a single event that occurs when a trade executes, making
/// it the most granular trade data you can attain 
pub struct Tick {
    tick_type: TickType,
    quantity: u64,
    exchange: Exchange,
    sale_condition: SaleCondition,
    suspicious: bool,

    bid_price: u64,
    bid_size: u64,
    ask_price: u64,
    ask_size: u64,
    last_price: u64,
}

/// Options chain
pub struct OptionChain {}
/// Futures chain
pub struct FuturesChain {}
