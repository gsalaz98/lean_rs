use std::collections::HashMap;
use crate::data::{BaseData, SecurityPrice, Slice, SubscriptionDataConfig};
use crate::data::base_data_collection::BaseDataCollection;
use crate::data::universe::{SecurityChanges, Universe};

pub(crate) mod local;
pub(crate) mod subscriptions;
pub(crate) mod synchronizer;

pub(crate) trait Synchronizer {
    fn stream_data<'a, T>(&mut self) -> dyn Iterator<Item = TimeSlice<'a, T>>
    where
        T: BaseData;
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

pub(crate) struct CancelationToken {

}

pub(crate) struct TimeSliceFactory {

}