use std::rc::Rc;
use crate::data::SubscriptionDataConfig;

pub(crate) struct SubscriptionDataReader {
    config: Rc<SubscriptionDataConfig>,
    
}