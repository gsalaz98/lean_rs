use std::hash::Hash;
use crate::data::{Security, SubscriptionDataConfig};

/// Universe
#[derive(Eq, PartialEq, Hash)]
pub struct Universe {}

/// Changes to security
pub struct SecurityChanges {

}

/// Subscription request
pub struct SubscriptionRequest {
    start_time: u128,
    end_time: u128,
    universe_subscription: bool,
    universe: Option<Universe>,
    security: Option<Security>,
    configuration: SubscriptionDataConfig,
}