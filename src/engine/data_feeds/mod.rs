use std::collections::HashMap;
use crate::data::{BaseData, SecurityPrice, Slice, SubscriptionDataConfig};
use crate::data::base_data_collection::BaseDataCollection;
use crate::data::universe::{SecurityChanges, Universe};
use crate::engine::Algorithm;

pub(crate) mod local;
pub(crate) mod subscriptions;
pub(crate) mod synchronizer;

pub(crate) trait Synchronizer<'a, B>
where
    B: BaseData
{
    type SynchronizeIterator;

    fn new(algorithm: impl Algorithm);
    fn stream_data(&mut self) -> Self::SynchronizeIterator;
}

pub(crate) trait TimeProvider {
    fn get_utc_now() -> u128;
}

pub(crate) struct Subscription<'a, 'b> {
    removed_from_universe: bool,
    subscription_requests: Vec<SubscriptionRequest>,
    universes: Vec<Universe>,
    security: SecurityPrice,
    configuration: &'a SubscriptionDataConfig<'b>,
}

pub(crate) struct SubscriptionRequest {

}

pub(crate) struct TimeSlice<'a, T> 
where 
    T: BaseData
{
    data_point_count: i64,
    time: u128,
    data: Vec<DataFeedPacket<'a, T>>,
    slice: &'a Slice,
    security_changes: SecurityChanges,
    universe_data: HashMap<Universe, BaseDataCollection<T>>
}

pub(crate) struct DataFeedPacket<'a, T>
where
    T: BaseData
{
    is_removed: bool,

    security: SecurityPrice,
    configuration: SubscriptionDataConfig<'a>,
    data: Vec<&'a T>
}

pub(crate) struct TimeSliceFactory {

}

pub(crate) struct SubscriptionFrontierTimeProvider<'a, T>
where
    T: Subscriptions
{
    utc_now: u128,
    subscription_manager: Vec<&'a T>,
}

impl<'a, T> SubscriptionFrontierTimeProvider<'a, T> {
    fn new(utc_now: u128, subscriptions: Vec<&'a Subscription>) -> Self {
        Self {
            utc_now,
            subscription_manager: subscriptions 
        }
    }
}