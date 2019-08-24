use crate::data::{BaseData, Market, Resolution, Security, Slice};

pub trait QCAlgorithm {
    type CashBook;
    type DataFeeds;
    type Portfolio;
    type Securities;
    type ActiveSecurities;

    fn initialize(&self);
    fn on_data<T>(&self, data: Vec<T>);

    // Optional methods

    /// Triggered at the end of the day. This is as defined in the
    /// BaseData implementation
    fn on_end_of_day(&self) {}
    /// Any order event that occurs will be brought here
    fn on_order(&self, order: &OrderEvent) {}

    // Default methods

    /// Logs a message to the console
    fn log(&self, msg: &str) {
        println!("Log:: {}", &msg.to_owned());
    }

    /// Logs an error message to the console
    fn debug(&self, msg: &str) {
        println!("Debug:: {}", &msg.to_owned());
    }

    /// We redesign add_data<T> here to be more usable with custom data
    fn add_data<T: BaseData>(ticker: &str, resolution: Resolution, market: Market, alias: Option<&str>) -> Security {
        
    }
}

pub struct OrderEvent {}