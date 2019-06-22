use std::collections::HashMap;
use crate::data::BaseData;
use crate::data::collections::BaseDataCollection;
use crate::data::universe::{SecurityChanges, SubscriptionRequest, Universe};
use crate::engine::data_feeds::{DataFeedPacket, TimeSlice, Subscription};
use crate::engine::frontier::ManualTimeProvider;

pub(crate) mod subscription_reader;

pub trait Subscriptions {

}

pub struct SubscriptionData<'a, B> 
where
    B: BaseData
{
    data: &'a B,
    emit_time_utc: u64,
}

pub struct SubscriptionDataSource {

}

pub struct SubscriptionSynchronizer<'a, 'b, 'c, B, I>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<'a, B>>
{
    frontier_time_provider: ManualTimeProvider,
    subscription_finished: Vec<Box<FnOnce(&'a Self, &'b Subscription<'c, B, I>)>>
}

impl<'a, 'b, 'c, B, I> SubscriptionSynchronizer<'a, 'b, 'c, B, I>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<'a, B>>
{
    fn sync(&self, subscriptions: Vec<Subscription<'a, B, I>>) -> TimeSlice<'a, B>
    where
        B: BaseData
    {
        let temp_data = Vec::with_capacity(1);

        let universe_data: HashMap<Universe, BaseDataCollection<B>> = HashMap::new();
        let frontier = self.frontier_time_provider.current_time;

        let mut newChanges: Option<SecurityChanges> = None;

        loop {
            let sub_len = subscriptions.len();

            for (i, subscription) in subscriptions.into_iter().enumerate() {
                if i == sub_len {
                    self.on_subscription_finished(&subscription);
                }

                let mut packet = None;

                for data in subscription.data {
                    while data.emit_time_utc <= frontier {
                        match packet {
                            Some(data) => data.push(data.data),
                            None => packet = DataFeedPacket::new(&subscription)
                        };
                    }
                }
            }
        }
    }

    pub fn on_subscription_finished(&self, subscription: &Subscription<'c, B, I>) {
        for handler in self.subscription_finished {
            (*handler)(&self, subscription)
        }
    }
}