use std::vec::IntoIter;
use crate::data::{BaseData, EpochTime, Time};
use crate::engine::Algorithm;
use crate::engine::data_feeds::{DataFeedSubscriptionManager, Subscription, SubscriptionData, SubscriptionFrontierTimeProvider, TimeSlice, TimeSliceFactory};
use crate::engine::data_feeds::subscriptions::SubscriptionSynchronizer;

pub(crate) struct DataSynchronizer<'a, 'b, 'c, B, D, I, T>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    algorithm: T,
    subscriptions: Vec<Subscription<'a, B, I>>,
    /// Manages subscription synchronization (i.e. generation of TimeSlice items)
    pub subscription_synchronizer: SubscriptionSynchronizer<'a, 'b, 'c, B, I>,
    time_slice_factory: TimeSliceFactory,
    time_provider: SubscriptionFrontierTimeProvider<D>,
    frontier_time_provider: EpochTime,

    previous_emit_time: EpochTime,
}

impl<'a, 'b, 'c, B, D, I, T> DataSynchronizer<'a, 'b, 'c, B, D, I, T>
where
    B: BaseData,
    D: DataFeedSubscriptionManager<'a, B, I>,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    fn get_initial_frontier_time(&self) -> EpochTime {
        let frontier = u64::max_value();

        for subscription in self.subscriptions.into_iter() {
            
        }

        return EpochTime {
            time: frontier
        }
    }
}

impl<'a, 'b, 'c, B, D, I, T> Iterator for DataSynchronizer<'a, 'b, 'c, B, D, I, T>
where
    B: BaseData + 'c,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    type Item = TimeSlice<'c, B>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.subscription_synchronizer.sync(&mut self.subscriptions) {
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
            Err(runtime_err) => None //self.algorithm_error(runtime_err)
        }
    }
}