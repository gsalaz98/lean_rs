pub(crate) mod subscription_reader;

pub trait DataFeedSubscriptionManager {
    fn on_subscription_added(subscription: Subscription);
    fn on_subscription_removed(subscription: Subscription);
}

pub struct Subscription {

}

pub struct SubscriptionDataSource {

}

pub struct SubscriptionSynchronizer {

}