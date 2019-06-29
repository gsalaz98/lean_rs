use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::data::{BaseData, EpochTime};
use crate::data::collections::BaseDataCollection;
use crate::data::universe::{SecurityChanges, Universe};
use crate::engine::data_feeds::{DataFeedPacket, TimeSlice, Subscription};
use crate::engine::frontier::ManualTimeProvider;

pub(crate) mod subscription_reader;

pub struct SubscriptionData<B> 
where
    B: BaseData
{
    data: Rc<B>,
    emit_time_utc: u64,
}

pub struct SubscriptionDataSource {

}

pub struct SubscriptionSynchronizer<B, I>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>
{
    frontier_time_provider: Rc<ManualTimeProvider>,
    subscription_finished: Vec<Box<FnOnce(Self, Rc<Subscription<B, I>>)>>
}

pub struct SubscriptionErr {
    message: String,
    frontier_step: EpochTime,
}

impl<B, I> SubscriptionSynchronizer<B, I>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>
{
    pub(crate) fn sync(&self, subscriptions: &Vec<Rc<Subscription<B, I>>>) -> Result<TimeSlice<B>, SubscriptionErr> {
        let temp_data = Vec::<B>::with_capacity(1);

        let universe_data: HashMap<Universe, BaseDataCollection<B>> = HashMap::new();
        let frontier = self.frontier_time_provider.current_time;

        let mut newChanges: Option<SecurityChanges> = None;
        let sub_len = subscriptions.len() - 1;

        loop {
            let mut i = 0;
            for subscription in subscriptions {
                if i == sub_len {
                    //self.on_subscription_finished(subscription);
                }

                let packet: Rc<RefCell<Option<DataFeedPacket<B>>>> = Rc::new(RefCell::new(None));

                /*
                for subscription_data in subscription.data {
                    while subscription_data.emit_time_utc <= frontier {
                        let packet_clone = packet.clone();

                        if packet_clone.borrow().is_none() {
                            packet.replace(Some(DataFeedPacket::<B>::new(subscription.clone())));
                        }
                        
                        /*
                        match packet_clone {
                            Some(data) => data.push(subscription_data.data.clone()),
                            None => 
                        }
                        packet_clone.data.push(subscription_data.data.clone());
                        */
                    }
                }
                */
            }
        }
    }

    pub fn on_subscription_finished(&self, subscription: Rc<Subscription<B, I>>) {
        for handler in self.subscription_finished.iter() {
            /*
            handler(Self {
                frontier_time_provider: self.frontier_time_provider.clone(),
                subscription_finished: vec![]
            }, subscription.clone())
            */
        }
    }
}