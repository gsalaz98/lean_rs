use std::collections::HashMap;
use crate::data::{BaseData, EpochTime, Time, Security, Slice, SubscriptionDataConfig};
use crate::data::collections::BaseDataCollection;
use crate::data::universe::{SecurityChanges, Universe};
use crate::engine::data_feeds::subscriptions::{Subscriptions, SubscriptionData};
use chrono::prelude::*;

pub(crate) mod local;
pub(crate) mod subscriptions;
pub(crate) mod synchronizer;

pub struct Subscription<'a, B, I> 
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>
{
    removed_from_universe: bool,
    requests: Vec<SubscriptionRequest>,

    security: Security,
    config: SubscriptionDataConfig<'a>,
    data: I,
}

pub(crate) struct SubscriptionRequest {

}

pub(crate) struct TimeSlice<'a, B> 
where 
    B: BaseData
{
    data_point_count: i64,
    time: EpochTime,
    data: Vec<DataFeedPacket<'a, B>>,
    slice: &'a Slice,
    security_changes: Option<SecurityChanges>,
    universe_data: HashMap<Universe, BaseDataCollection<B>>
}

pub(crate) struct DataFeedPacket<'a, B>
where
    B: BaseData
{
    is_removed: bool,

    security: &'a Security,
    configuration: SubscriptionDataConfig<'a>,
    data: Vec<&'a B>
}

pub(crate) struct TimeSliceFactory {

}

pub(crate) struct SubscriptionFrontierTimeProvider<'a, T>
where
    T: Subscriptions
{
    utc_now: EpochTime,
    subscription_manager: &'a Vec<T>,
}

impl<'a, B> DataFeedPacket<'a, B>
where 
    B: BaseData 
{
    fn new<I>(subscription: &Subscription<'a, B, I>) -> Self
    where
        I: Iterator<Item = SubscriptionData<B>>
    {
        let security = subscription.security;

        Self {
            security: &security,
            configuration: subscription.config,
            data: Vec::with_capacity(64),
            is_removed: subscription.removed_from_universe
        }
    }
}

impl<'a, T> SubscriptionFrontierTimeProvider<'a, T>
where
    T: Subscriptions 
{
    fn new(utc_now: EpochTime, subscriptions: &'a Vec<T>) -> Self {
        Self {
            utc_now,
            subscription_manager: subscriptions 
        }
    }
}

impl<'a, T> Time for SubscriptionFrontierTimeProvider<'a, T> 
where
    T: Subscriptions
{
    fn to_chrono(&self) -> DateTime<Utc> {
        use chrono;

        
        self.utc_now.time
    }
}

impl<'a, B, I> Subscriptions for Subscription<'a, B, I> 
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>
{

}