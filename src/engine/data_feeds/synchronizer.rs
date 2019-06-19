use crate::data::BaseData;
use crate::engine::Algorithm;
use crate::engine::data_feeds::{Synchronizer, TimeProvider, TimeSlice, TimeSliceFactory};
use crate::engine::data_feeds::subscriptions::{DataFeedSubscriptionManager, SubscriptionSynchronizer};

pub(crate) struct DataSynchronizer<T, U, V, W> 
where
    T: Algorithm,
    U: DataFeedSubscriptionManager,
    V: TimeProvider,
    W: TimeProvider
{
    algorithm: T,
    subscription_manager: U,
    subscription_synchronizer: SubscriptionSynchronizer,
    time_slice_factory: TimeSliceFactory,
    time_provider: V,
    frontier_time_provider: W,
}

impl<T, U, V, W> Synchronizer for DataSynchronizer<T, U, V, W> 
where
    T: Algorithm,
    U: DataFeedSubscriptionManager,
    V: TimeProvider,
    W: TimeProvider
{
    fn stream_data<'a, D>(&mut self) -> dyn Iterator<Item = TimeSlice<'a, D>> 
    where
        D: BaseData
    {
        return Vec::<TimeSlice<'a, D>>::default();
    }
}