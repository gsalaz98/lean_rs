use std::hash::Hash;
use crate::data::{Security, SubscriptionDataConfig};

#[derive(Eq, PartialEq, Hash)]
pub struct Universe {}

pub struct SecurityChanges {}
pub struct SubscriptionRequest<'a> {
    start_time: u128,
    end_time: u128,
    universe_subscription: bool,
    universe: Option<Universe>,
    security: Option<Security>,
    configuration: SubscriptionDataConfig<'a>,
}