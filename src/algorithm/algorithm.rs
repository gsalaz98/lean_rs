use std::collections::HashMap;
use crate::data::{BaseData, Market, Resolution, Security, Slice, Symbol};

pub trait Algorithm {
    type CashBook;
    type DataFeeds;
    type Portfolio;
    type Securities;
    type ActiveSecurities;

    /// Logs a message to the console
    fn log(&self, msg: str) {
        println!("Log:: {}", msg);
    }

    /// Logs an error message to the console
    fn debug(&self, msg: str) {
        println!("Debug:: {}", msg);
    }

    /// We redesign add_data<T> here to be more usable with custom data
    fn add_data<T: BaseData>(ticker: &str, resolution: Resolution, market: Market, alias: Option<&str>) -> Security {
        
    }
}