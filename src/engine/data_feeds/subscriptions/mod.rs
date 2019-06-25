use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::data::{BaseData, EpochTime};
use crate::data::collections::BaseDataCollection;
use crate::data::universe::{SecurityChanges, SubscriptionRequest, Universe};
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

pub struct SubscriptionSynchronizer<'a, 'b, 'c, B, I>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>
{
    frontier_time_provider: ManualTimeProvider,
    subscription_finished: Vec<Box<FnOnce(Self, &'b Subscription<'c, B, I>)>>
}

struct SubscriptionErr {
    message: String,
    frontier_step: EpochTime,
}

impl<'a, 'b, 'c, B, I> SubscriptionSynchronizer<'a, 'b, 'c, B, I>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>
{
    pub fn sync(&self, subscriptions: &Vec<Subscription<'a, B, I>>) -> Result<TimeSlice<'c, B>, SubscriptionErr> {
        let temp_data = Vec::<B>::with_capacity(1);

        let universe_data: HashMap<Universe, BaseDataCollection<B>> = HashMap::new();
        let frontier = self.frontier_time_provider.current_time;

        let mut newChanges: Option<SecurityChanges> = None;
        let sub_len = subscriptions.len() - 1;

        loop {
            let mut i = 0;
            for subscription in subscriptions.iter() {
                if i == sub_len {
                    //self.on_subscription_finished(subscription);
                }

                let packet: Rc<RefCell<Option<DataFeedPacket<'_, B>>>> = Rc::new(RefCell::new(None));

                for subscription_data in subscription.data.by_ref() {
                    while subscription_data.emit_time_utc <= frontier {
                        let packet_clone = packet.clone();

                        if packet_clone.borrow().is_none() {
                            packet.replace(Some(DataFeedPacket::new(&subscription)));
                        }
                        
                        packet_clone.borrow_mut().unwrap().data.push(subscription_data.data.clone());
                    }
                }
            }
        }
    }

    pub fn on_subscription_finished(&self, subscription: &'b Subscription<'c, B, I>) {
        for handler in self.subscription_finished {
            (*handler)(Self {
                frontier_time_provider: self.frontier_time_provider,
                subscription_finished: vec![]
            }, subscription)
        }
    }
}