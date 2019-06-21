use std::collections::HashMap;
use crate::data::{BaseData, SecurityPrice, Slice, SubscriptionDataConfig};
use crate::data::collections::BaseDataCollection;
use crate::data::universe::{SecurityChanges, Universe};
use crate::engine::data_feeds::subscriptions::{Subscriptions, SubscriptionData};

pub(crate) mod local;
pub(crate) mod subscriptions;
pub(crate) mod synchronizer;

pub(crate) trait TimeProvider {
    fn get_utc_now() -> u128;
}

pub struct Subscription<'a, B> 
where
    B: BaseData
{
    removed_from_universe: bool,
    data: Vec<SubscriptionData<'a, B>>,
    requests: Vec<SubscriptionRequest>,
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

impl<'a, T> SubscriptionFrontierTimeProvider<'a, T>
where
    T: Subscriptions 
{
    fn new(utc_now: u128, subscriptions: Vec<&'a T>) -> Self {
        Self {
            utc_now,
            subscription_manager: subscriptions 
        }
    }
}

impl Subscriptions for Subscription {

}