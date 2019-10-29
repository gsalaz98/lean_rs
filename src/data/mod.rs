#![deny(missing_docs)]

use std::collections::HashMap;
use std::hash::Hash;
use std::io::prelude::*;
use std::rc::Rc;
use std::time::Duration;
use crate::engine::data_feeds::subscriptions::SubscriptionDataSource;
use map_files::MapFileProvider;
use bars::*;
use chrono::prelude::*;

/// Kinds of Bars, such as TradeBars, QuoteBars, etc.
pub mod bars;
/// BaseData collection data types and methods
pub mod collections;
/// Equity mapping and map file methods
pub mod map_files;
/// Universe selection
pub mod universe;
/// Subscription definitions
pub mod subscriptions;

/// Base form of data representable in LEAN
pub trait BaseData {
    /// Instructs `SubscriptionDataReader` where to look for the data
    fn get_source() -> SubscriptionDataSource;
    /// Reads and transforms the data to type T
    fn reader<T>(config: SubscriptionDataConfig, factory: T, date: u128) -> Self
    where
        T: Read;
}

/// Represents time in LEAN
pub trait Time {
    /// Converts the underlying time data to a [`DateTime`]
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

/// Time represented by seconds since the beginning of the universe (aka Janurary 1st, 1970)
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct EpochTime {
    /// Moment in time represented by epoch seconds
    pub time: u64
}

/// Represents a tradable security. Symbol structs are distinguished by a [`SecurityIdentifier`]
/// that contains more information about the given symbol, such as their underlying ID, first trade date,
/// first mapped symbol, etc.
/// 
/// Symbols are heavily used in algorithms to represent tradable securities.
/// They can be used to place orders on a specific security, maintain a consistent 
/// symbol across time through split and delisting events.
/// 
/// # F.A.Q.
/// Q: How do I access the current ticker?
/// A: In order to access the current ticker of the Symbol, you can reference [`Symbol.value`]
/// 
/// Q: If I'm trading options, how do I access the underlying symbol?
/// A: Use [`Symbol.underlying`]
pub struct Symbol<'a> {
    pub id: SecurityIdentifier<'a>,
    value: String,
    underlying: Option<Symbol<'a>>,
    security_type: SecurityType,
}

/// Tradeable Security
pub struct Security {
    
}

/// Cache of security identifiers
pub struct SecurityIdentifierCache<'a> {
    cache: HashMap<String, SecurityIdentifier<'a>>
}

/// Identifies a security via its unique properties contained within.
/// We use a Base-32 ID to encode information into the SecurityIdentifier
/// that can be used to transmit the information across networks
pub struct SecurityIdentifier<'a> {
    pub symbol: Box<Symbol<'a>>,

    map_file_provider_name: String,
    map_file_provider: &'a dyn MapFileProvider,
}

/// Represents a "slice" of data at a given point in time.
/// [`QCAlgorithm.on_data`] consumes slices produced.
pub struct Slice<B> 
where 
    B: BaseData 
{
    splits: Option<Splits>,
    dividends: Option<Dividends>,
    delistings: Option<Delistings>,
    symbol_changed_events: Option<SymbolChangedEvents>,

    time: u128,
    has_data: bool,

    option_chains: Option<Vec<OptionChain>>,
    futures_chains: Option<Vec<FuturesChain>>,

    data: B
}

/// Split events
pub struct Splits {}

/// Dividend events
pub struct Dividends {}

/// Delisting events
pub struct Delistings {}

/// Symbol rename events
pub struct SymbolChangedEvents {
    pub old_symbol: String,
    pub new_symbol: String
}

/// Asset class
pub enum SecurityType {
    /// The base form of data. This should never be used, but we've 
    /// included it here for legacy purposes
    Base,
    /// Bond markets
    Bond,
    /// Equities/stocks
    Equity,
    /// Foreign Exchange (currency trading)
    Forex,
    /// Cfd
    Cfd,
    /// Cryptocurrency
    Crypto,
    /// Cryptocurrency futures
    CryptoFutures,
    /// Cryptocurrency options
    CryptoOptions,
    /// Futures Contracts
    Future,
    /// Options markets. We call it Option"S" with an s
    /// to avoid conflict with the [`Option<T>`] type
    Options,
}

/// Venue a transaction took place in
pub enum Exchange {
    /// New York Stock Exchange (NYSE)
    NYSE,
    /// NASDAQ
    NASDAQ
}

/// Supported markets
pub enum Market {
    /// Coinbase Pro
    GDAX
}

/// Tick type
pub enum TickType {
    /// Trade event 
    Trade,
    /// Quote event 
    Quote,
    /// Open interest event 
    OpenInterest,
    /// Orderbook update event 
    OrderbookUpdate,
    /// Orderbook remove event
    OrderbookRemove,
}

/// Resoulution/timeframe of the data covered per event
pub enum Resolution {
    /// Daily resolution loads data day to day at 00:00:00 UTC
    Daily,
    /// Hourly resolution loads data by hour
    Hour,
    /// Minute resolution loads data by minute
    Minute,
    /// Second resolution loads data by second
    Second,
    /// Tick resolution loads data event-by-event. This is the most granular timeframe possible.
    Tick,
}

/// Variants of BaseData that have been implemented as structs.
/// We define this enum so that we can identify the "type"
/// as is done on QC's C# backtesting engine
pub enum DataTypes {
    /// Represents data with OHLCV, but no Bid/Ask information
    TradeBar,
    /// Represents data with potentially missing OHCLV, but contains Bid/Ask information
    QuoteBar,
    /// Orderbook update event
    OrderbookUpdate,
    /// Orderbook order removal event
    OrderbookRemove,
}

/// Condition of sale - for equity data
pub enum SaleCondition {

}

impl<'a> SecurityIdentifier<'a> {
    pub fn generate_base<T: BaseData>(ticker: impl Into<String>, market: Market, map_symbol: bool, date: Option<chrono::DateTime>) -> Self {
        let symbol: Symbol;
        let first_date: chrono::DateTime;

        if map_symbol {
            let first_ticker_date = Self::get_first_ticker_and_date(symbol, market);
            symbol = first_ticker_date.0;
            first_date = first_ticker_date.1;
        }

        Self::generate(
            first_date, 
            Self::generate_base_symbol::<T>(symbol),
            SecurityType::Base,
            market,
            false)
    }
}