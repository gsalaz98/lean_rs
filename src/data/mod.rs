use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use crate::engine::data_feeds::subscriptions::SubscriptionDataSource;
use map_files::MapFileProvider;
use bars::*;
use chrono::prelude::*;

pub mod bars;
pub mod collections;
pub mod map_files;
pub mod universe;

/// Base form of data representable
pub trait BaseData {
    /// Instructs `SubscriptionDataReader` where to look for the data
    fn get_source() -> SubscriptionDataSource;
    /// Reads and transforms the data to type T
    fn reader<T>(config: SubscriptionDataConfig, line: T, date: u128) -> Self
    where
        T: DataEvent;
}

/// Event of data. Can contain multiple pieces of data
pub trait DataEvent: Iterator {

}

pub trait Time {
    fn to_chrono(&self) -> DateTime<Utc>;
}

/// Subscription configuration: instructs the [`crate::subscriptions::SubscriptionDataReader`] 
pub struct SubscriptionDataConfig<'a>
{
    instance: DataTypes,
    security_type: SecurityType,
    symbol: Symbol<'a>,
    tick_type: TickType,
    resolution: Resolution,
    increment: Duration,
    fill_data_forward: bool,
    extended_hours: bool,
    mapped_symbol: String,
    normalization_mode: u8,
    market: Market,
    data_tz: i8,
    exchange_tz: i8,
}

#[derive(Eq, PartialEq, Hash)]
pub struct EpochTime {
    pub time: u64
}

pub struct Symbol<'a> {
    id: Box<SecurityIdentifier<'a>>,
    value: &'a str,
    underlying: Option<Box<Symbol<'a>>>,
    security_type: SecurityType,
}

pub struct Security {
    
}

pub struct SecurityIdentifierCache<'a> {
    cache: HashMap<&'a str, Box<SecurityIdentifier<'a>>>
}

pub struct SecurityIdentifier<'a> {
    map_file_provider_name: &'a str,
    map_file_provider: dyn MapFileProvider,
}

pub struct Slice {
    splits: Option<Splits>,
    dividends: Option<Dividends>,
    delistings: Option<Delistings>,
    symbol_changed_events: Option<SymbolChangedEvents>,

    time: u128,
    has_data: bool,
    bars: Option<Vec<TradeBar>>,
    quote_bars: Option<Vec<QuoteBar>>,
    ticks: Option<Vec<Tick>>,
    option_chains: Option<Vec<OptionChain>>,
    futures_chains: Option<Vec<FuturesChain>>,
}

pub struct Splits {}
pub struct Dividends {}
pub struct Delistings {}
pub struct SymbolChangedEvents {}

/// Asset class
pub enum SecurityType {
    BaseData,
    Equity,
    Forex,
    Cfd,
    Crypto,
    Future,
}

pub enum Market {
    GDAX
}

/// Tick type
pub enum TickType {
    Trade,
    Quote,
    OpenInterest,
    OrderbookUpdate,
    OrderbookRemove,
}

/// Resoulution/timeframe of the data covered per event
pub enum Resolution {
    Daily,
    Hour,
    Minute,
    Second,
    Tick,
}

/// Variants of BaseData that have been implemented as structs.
/// We define this enum so that we can identify the "type"
/// as is done on QC's C# backtesting engine
pub enum DataTypes {
    TradeBar,
    QuoteBar,
    OrderbookUpdate,
    OrderbookRemove,
}

pub enum SecurityPrice {
    Cfd,
    Crypto,
    Equity,
    Forex,
    Future,
    Options,
    Security
}

pub enum Exchange {

}

pub enum SaleCondition {

}