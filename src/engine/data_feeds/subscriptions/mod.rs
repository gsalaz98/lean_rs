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

        let mut newChanges: Option<SecurityChanges> = None;

        loop {
            let sub_len = subscriptions.len();

            for (i, subscription) in subscriptions.enumerate().into_iter() {
                if i == sub_len {
                    self.on_subscription_finished(&subscription);
                }

                let mut packet = None;

                match subscription.current {
                    Some(sub) => {
                        while data.emit_time_utc <= frontier {
                            match packet {
                                Some(data) => data.push(sub.data),
                                None => packet = DataFeedPacket::new(&subscription);
                            };
                        }
                    }
                }
            }
        }
    }
}