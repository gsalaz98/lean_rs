use std::vec::IntoIter;
use crate::data::BaseData;
use crate::engine::Algorithm;
use crate::engine::data_feeds::{Subscription, SubscriptionFrontierTimeProvider, TimeProvider, TimeSlice, TimeSliceFactory};
use crate::engine::data_feeds::subscriptions::SubscriptionSynchronizer;

pub(crate) struct DataSynchronizer<'a, B, T, V, W> 
where
    B: BaseData,
    T: Algorithm,
    V: TimeProvider,
    W: TimeProvider
{
    algorithm: T,
    subscription_manager: Vec<Subscription<'a, B>>,
    subscription_synchronizer: SubscriptionSynchronizer,
    time_slice_factory: TimeSliceFactory,
    time_provider: V,
    frontier_time_provider: W,

    previous_emit_time: u64,
}

impl<'a, B, T, V, W> DataSynchronizer<'a, B, T, V, W>
where
    B: BaseData,
    T: Algorithm,
    V: TimeProvider,
    W: TimeProvider 
{
    fn get_time_provider(&self) -> impl TimeProvider {
        SubscriptionFrontierTimeProvider::<'a, Subscription<'a, B>>::new(self.get_initial_frontier_time(), self.subscription_manager)
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
                    None => return None;
                }
            },
            Err(runtime_err) => AlgorithmErr(runtime_err)
        }
    }
}