use crate::data::*;

pub trait Bar {}

pub struct TradeBar {}
pub struct QuoteBar {}
pub struct Tick {
    tick_type: TickType,
    quantity: u64,
    exchange: String,
    sale_condition: SaleCondition,
    suspicious: bool,

    bid_price: u64,
    bid_size: u64,
    ask_price: u64,
    ask_size: u64,
    last_price: u64,
}

pub struct OptionChain {}
pub struct FuturesChain {}

