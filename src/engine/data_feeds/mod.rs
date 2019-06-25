use std::collections::HashMap;
use std::time::Duration;
use std::rc::Rc;
use crate::data::{BaseData, EpochTime, Time, Security, Slice, SubscriptionDataConfig};
use crate::data::collections::BaseDataCollection;
use crate::data::universe::{SecurityChanges, Universe};
use crate::engine::data_feeds::subscriptions::SubscriptionData;
use chrono::prelude::*;

pub(crate) mod local;
pub(crate) mod subscriptions;
pub(crate) mod synchronizer;

pub trait DataFeedSubscriptionManager<'a, B, I>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>
{
    fn add_subscription(&self, subscription: Subscription<'a, B, I>);
    fn remove_subscription(&self, subscription: Subscription<'a, B, I>);
}

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
    configuration: &'a SubscriptionDataConfig<'a>,
    data: Vec<Rc<B>>
}

pub(crate) struct TimeSliceFactory {

}

pub(crate) struct SubscriptionFrontierTimeProvider<T> {
    utc_now: EpochTime,
    subscription_manager: T,
}

impl<'a, B> DataFeedPacket<'a, B>
where 
    B: BaseData 
{
    fn new<'b: 'a, I>(subscription: &'b Subscription<'a, B, I>) -> Self
    where
        I: Iterator<Item = SubscriptionData<B>>
    {
        Self {
            security: &subscription.security,
            configuration: &subscription.config,
            data: Vec::with_capacity(64),
            is_removed: subscription.removed_from_universe
        }
    }
}

impl<T> SubscriptionFrontierTimeProvider<T> {
    fn new(utc_now: EpochTime, subscriptions: T) -> Self {
        Self {
            utc_now,
            subscription_manager: subscriptions 
        }
    }
}

impl<T> Time for SubscriptionFrontierTimeProvider<T> {
    fn to_chrono(&self) -> DateTime<Utc> {
        let duration = Duration::from_millis(self.utc_now.time);
        
        DateTime::from_utc(NaiveDateTime::from_timestamp(duration.as_secs() as i64, duration.subsec_millis()), Utc)
    }
}