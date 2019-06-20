use std::vec::IntoIter;
use crate::data::BaseData;
use crate::engine::Algorithm;
use crate::engine::data_feeds::{SubscriptionFrontierTimeProvider, Synchronizer, TimeProvider, TimeSlice, TimeSliceFactory};
use crate::engine::data_feeds::subscriptions::{Subscription, SubscriptionSynchronizer};

pub(crate) struct DataSynchronizer<'a, B, T, V, W> 
where
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
}

impl<'a, 'b, B, T, U, V, W> Synchronizer<'a, B> for DataSynchronizer<'b, B, T, V, W> 
where
    B: BaseData + 'a,
    T: Algorithm,
    V: TimeProvider,
    W: TimeProvider
{
    type SynchronizeIterator = IntoIter<TimeSlice<'a, B>>;

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

