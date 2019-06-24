use std::vec::IntoIter;
use crate::data::{BaseData, EpochTime, Time};
use crate::engine::Algorithm;
use crate::engine::data_feeds::{Subscription, SubscriptionData, SubscriptionFrontierTimeProvider, TimeSlice, TimeSliceFactory};
use crate::engine::data_feeds::subscriptions::SubscriptionSynchronizer;

pub(crate) struct DataSynchronizer<'a, 'b, 'c, B, I, T>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    algorithm: T,
    subscription_manager: Vec<Subscription<'a, B, I>>,
    /// Manages subscription synchronization (i.e. generation of TimeSlice items)
    pub subscription_synchronizer: SubscriptionSynchronizer<'a, 'b, 'c, B, I>,
    time_slice_factory: TimeSliceFactory,
    time_provider: EpochTime,
    frontier_time_provider: EpochTime,

    previous_emit_time: EpochTime,
}

impl<'a, 'b, 'c, B, I, T> DataSynchronizer<'a, 'b, 'c, B, I, T>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    fn get_time_provider(&self) -> impl Time {
        SubscriptionFrontierTimeProvider::new(self.get_initial_frontier_time(), &self.subscription_manager)
    }

    fn get_initial_frontier_time(&self) -> EpochTime {
        let frontier = u64::max_value();

        for subscription in self.subscription_manager.into_iter() {
            
        }

        return EpochTime {
            time: frontier
        }
    }
}

impl<'a, 'b, 'c, B, I, T> Iterator for DataSynchronizer<'a, 'b, 'c, B, I, T>
where
    B: BaseData + 'a,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    type Item = TimeSlice<'a, B>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.subscription_synchronizer.sync(&self.subscription_manager) {
            Ok(time_slice) => {
                if time_slice.time.time != u64::max_value() {
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