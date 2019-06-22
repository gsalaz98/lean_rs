use std::vec::IntoIter;
use crate::data::{BaseData, EpochTime};
use crate::engine::Algorithm;
use crate::engine::data_feeds::{Subscription, SubscriptionData, SubscriptionFrontierTimeProvider, TimeSlice, TimeSliceFactory};
use crate::engine::data_feeds::subscriptions::SubscriptionSynchronizer;

pub(crate) struct DataSynchronizer<'a, 'b, 'c, B, I, T>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<'a, B>>,
    T: Algorithm,
{
    algorithm: T,
    subscription_manager: Vec<Subscription<'a, B, I>>,
    subscription_synchronizer: SubscriptionSynchronizer<'a, 'b, 'c, B, I>,
    time_slice_factory: TimeSliceFactory,
    time_provider: EpochTime,
    frontier_time_provider: EpochTime,

    previous_emit_time: u64,
}

impl<'a, 'b, 'c, B, I, T, V, W> DataSynchronizer<'a, 'b, 'c, B, I, T>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<'a, B>>,
    T: Algorithm,
{
    fn get_time_provider(&self) -> impl TimeProvider {
        SubscriptionFrontierTimeProvider::<'a, Subscription<'a, B, I>>::new(self.get_initial_frontier_time(), self.subscription_manager)
    }

    fn get_initial_frontier_time(&self) {
        let frontier = u128::max_value();

        for subscription in self.subscription_manager.into_iter() {
            
        }
    }
}

impl<'a, B, T, V, W> Iterator for DataSynchronizer<'a, B, T, V, W>
where
    B: BaseData,
    T: Algorithm,
    V: TimeProvider,
    W: TimeProvider 
{

    type Item = TimeSlice<'a, B>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.subscription_synchronizer.sync() {
            Ok(time_slice) => {
                if time_slice.time != u64.max_value() {
                    self.previous_emit_time = time_slice.time;
                    return Some(time_slice)
                }
                match time_slice.security_changes {
                    Some(change) => return Some(time_slice),
                    None => return None
                }
            },
            Err(runtime_err) => self.algorithm_error(runtime_err)
        }
    }
}