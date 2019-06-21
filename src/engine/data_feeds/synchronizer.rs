use std::vec::IntoIter;
use crate::data::BaseData;
use crate::engine::Algorithm;
use crate::engine::data_feeds::{SubscriptionFrontierTimeProvider, Synchronizer, TimeProvider, TimeSlice, TimeSliceFactory};
use crate::engine::data_feeds::subscriptions::{Subscription, SubscriptionSynchronizer};

pub(crate) struct DataSynchronizer<'a, B, T>
where
    B: BaseData,
    T: Algorithm,
{
    algorithm: T,
    subscriptions : Vec<Subscription<'a, B>>,
    synchronizer: SubscriptionSynchronizer,
}

impl<'a, 'b, B, T> Synchronizer<'a, 'b, B, T> for DataSynchronizer<'a, B, T> 
where
    B: BaseData + 'a,
    T: Algorithm,
{
    type SynchronizeIterator = IntoIter<TimeSlice<'b, B>>;

    fn new(algorithm: T, subscriptions: Vec<Subscription<'a, B>>, synchronizer: SubscriptionSynchronizer) -> Self {
        Self {
            algorithm,
            subscriptions,
            synchronizer
        }
    }

    fn stream_data(&mut self) -> Self::SynchronizeIterator
    {
        self.post_initialize();

        // get_time_provider() will call get_initial_frontier_time() which
        // will consume added subscriptions so we need to do this after initialization
        let time_provider = self.get_time_provider();

        // Just return an empty vec for the time being
        Vec::<TimeSlice<'a, B>>::default().into_iter()
    }
}

impl<'a, B, T, V, W> DataSynchronizer<'a, B, T, V, W>
where
    B: BaseData,
    T: Algorithm,
    V: TimeProvider,
    W: TimeProvider 
{
    fn get_time_provider(&self) -> impl TimeProvider {
        SubscriptionFrontierTimeProvider::<'a, B>::new(self.get_initial_frontier_time(), self.subscription_manager)
    }

    fn get_initial_frontier_time(&self) {
        let frontier = u128::max_value();

        for subscription in self.subscription_manager.into_iter() {
            
        }
    }
}

