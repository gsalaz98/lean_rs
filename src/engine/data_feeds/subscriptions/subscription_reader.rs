use crate::data::SubscriptionDataConfig;

pub(crate) struct SubscriptionDataReader<'a, 'b> {
    config: &'a SubscriptionDataConfig<'b>,
    
}