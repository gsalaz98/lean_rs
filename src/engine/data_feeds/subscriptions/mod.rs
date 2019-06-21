use std::collections::HashMap;
use crate::data::BaseData;
use crate::data::collections::BaseDataCollection;
use crate::data::universe::{Universe, SubscriptionRequest};
use crate::engine::data_feeds::{TimeSlice, Subscription};
use crate::engine::frontier::ManualTimeProvider;

pub(crate) mod subscription_reader;

pub trait Subscriptions {

}

pub struct SubscriptionData<'a, B> 
where
    B: BaseData
{
    data: &'a B,
    emit_time_utc: u128,
}

pub struct SubscriptionDataSource {

}

pub struct SubscriptionSynchronizer {
    frontier_time_provider: ManualTimeProvider,
}

impl SubscriptionSynchronizer {
    fn sync<'a, B>(&self, subscriptions: Vec<Subscription<'a, B>>) -> TimeSlice<'a, B>
    where
        B: BaseData
    {
        let data = Vec::with_capacity(1);

        let universe_data: HashMap<Universe, BaseDataCollection<B>> = HashMap::new();
        let frontier = self.frontier_time_provider.get_utc_now();

    }
}