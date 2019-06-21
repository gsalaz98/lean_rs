use std::vec::IntoIter;
use crate::data::BaseData;
use crate::data::universe::SubscriptionRequest;

pub(crate) mod subscription_reader;

pub trait Subscriptions: Iterator {
    
}

pub struct Subscription<'a, T> 
where
    T: BaseData
{
    removed_from_universe: bool,
    data: IntoIter<SubscriptionData<'a, T>>,
    requests: Vec<SubscriptionRequest<'a>>,
}

pub struct SubscriptionData<'a, T> 
where
    T: BaseData
{
    data: &'a T,
    emit_time_utc: u128,
}

pub struct SubscriptionDataSource {

}

pub struct SubscriptionSynchronizer {

}