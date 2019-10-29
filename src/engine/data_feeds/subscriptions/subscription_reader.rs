use std::rc::Rc;
use crate::data::SubscriptionDataConfig;

pub(crate) struct SubscriptionDataReader<'a, 'b> {
    config: &'b SubscriptionDataConfig<'a>,
}