use std::rc::Rc;
use crate::data::{BaseData, EpochTime, Time};
use crate::engine::Algorithm;
use crate::engine::data_feeds::{DataFeedSubscriptionManager, Subscription, SubscriptionData, SubscriptionFrontierTimeProvider, TimeSlice, TimeSliceFactory};
use crate::engine::data_feeds::subscriptions::SubscriptionSynchronizer;

pub(crate) struct DataSynchronizer<B, D, I, T>
where
    B: BaseData,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    algorithm: T,
    subscriptions: Vec<Rc<Subscription<B, I>>>,
    /// Manages subscription synchronization (i.e. generation of TimeSlice items)
    pub subscription_synchronizer: SubscriptionSynchronizer<B, I>,
    time_slice_factory: TimeSliceFactory,
    time_provider: SubscriptionFrontierTimeProvider<D>,
    frontier_time_provider: EpochTime,

    previous_emit_time: EpochTime,
}

impl<B, D, I, T> DataSynchronizer<B, D, I, T>
where
    B: BaseData,
    D: DataFeedSubscriptionManager<B, I>,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    fn get_initial_frontier_time(&self) -> EpochTime {
        let frontier = u64::max_value();

        for subscription in self.subscriptions.iter() {
            
        }

        return EpochTime {
            time: frontier
        }
    }
}

impl<B, D, I, T> Iterator for DataSynchronizer<B, D, I, T>
where
    B: BaseData,
    D: DataFeedSubscriptionManager<B, I>,
    I: Iterator<Item = SubscriptionData<B>>,
    T: Algorithm,
{
    type Item = TimeSlice<B>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Bad practice; remove need to collect
        match self.subscription_synchronizer.sync(&self.subscriptions) {
            Ok(time_slice) => {
                if time_slice.time.time != u64::max_value() {
                    self.previous_emit_time = time_slice.time;
                    return Some(time_slice)
                }
                if time_slice.security_changes.is_some() {
                    return Some(time_slice)
                }
                return None
            },
            Err(runtime_err) => None //self.algorithm_error(runtime_err)
        }
    }
}