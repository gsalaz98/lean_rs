use std::collections::HashMap;
use chrono;
use chrono_tz::*;

use crate::algorithm::algorithm::Algorithm;
use crate::common::market_hours_database::MarketHoursDatabaseProvider;
use crate::common::portfolio::PortfolioProvider;
use crate::data::{BaseData, Market, Resolution, Security, SecurityType, SecurityIdentifier, Slice, Symbol};
use crate::data::subscriptions::SubscriptionManagerProvider;
use crate::engine::data_feeds::DataFeed;
pub trait MappedDataAlgorithm : Algorithm {
    /// We redesign add_data<T> here to be more usable with custom data
    fn add_data<T: BaseData, Mappable>(&self, ticker: impl Into<String>, resolution: Resolution, market: Market, alias: Option<&str>, fill_data_forward: bool, leverage: f64) -> Security {
        let symbol = Symbol::new(&self, SecurityIdentifier::generate_base::<T>(ticker.into(), Market::USA, true), ticker);
        
        self.add_data_impl::<T>(symbol, resolution, America::New_York, fill_data_forward, leverage)
    }

    fn add_data_impl<T, Tz: chrono::TimeZone>(symbol: Symbol, resolution: Resolution, timeZone: Tz, fill_data_forward: bool, leverage: f64) -> Security {

    }
}

pub trait UnmappedDataAlgorithm : Algorithm {
    fn add_data<T: BaseData, Unmappable>(ticker: impl Into<String>, resolution: Resolution, market: Market, alias: Option<&str>, fill_data_forward: bool, leverage: f64) -> Security {

    }
}

pub trait QCAlgorithm : MappedDataAlgorithm + UnmappedDataAlgorithm {
    type CashBook;
    type DataFeeds: DataFeed<dyn BaseData>;
    type Portfolio: PortfolioProvider;
    type Securities;
    type ActiveSecurities;

    type MarketHoursDatabase: MarketHoursDatabaseProvider;
    type SubscriptionManager: SubscriptionManagerProvider;

    fn initialize(&self);
    fn on_data(&self, data: HashMap<&Symbol, &dyn BaseData>);

    // /// Triggered at the end of the day. This is as defined in the
    // /// BaseData implementation
    // fn on_end_of_day(&self) {}
    // /// Any order event that occurs will be brought here
    // fn on_order(&self, order: &OrderEvent) {}

    // Default methods

    /// Logs a message to the console
    fn log<'a, T: Into<String>>(&self, msg: &T) {
        println!("Log:: {}", msg.into());
    }

    /// Logs an error message to the console
    fn debug<'a, T: Into<String>>(&self, msg: &T) {
        println!("Debug:: {}", msg.into());
    }

    fn add_data_impl<T, Tz: chrono::TimeZone>(&self, symbol: Symbol, resolution: Resolution, time_zone: Tz, fill_data_forward: bool, leverage: f64) -> Security {
        let alias = symbol.id.symbol;

        Self::MarketHoursDatabase.set_entry_always_open(Market::USA, alias, SecurityType::Base, time_zone);

        //Add this new generic data as a tradeable security:
        let config = Self::SubscriptionManager.subscription_data_config_service.push((symbol, resolution, fill_data_forward, true, true));
        let security = Self::Securities.create_security(symbol, config, leverage, false);

        self.add_to_user_defined_universe(security, vec![config]);
        
        security
    }
}