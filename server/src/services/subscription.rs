use std::result::Result;

use opcua_core::types::*;
use opcua_core::services::*;
use opcua_core::comms::*;

use types::*;
use server::ServerState;

pub struct SubscriptionService {}

// TODO a low value on purpose because this should be a server configurable setting, not a const
const MAX_SUBSCRIPTIONS: usize = 5;

impl SubscriptionService {
    pub fn new() -> SubscriptionService {
        SubscriptionService {}
    }

    pub fn create_subscription(&self, _: &mut ServerState, session_state: &mut SessionState, request: &CreateSubscriptionRequest) -> Result<SupportedMessage, &'static StatusCode> {
        debug!("create_subscription {:#?}", request);

        if session_state.subscriptions.len() >= MAX_SUBSCRIPTIONS {
            return Err(&BAD_TOO_MANY_SUBSCRIPTIONS);
        }

        let subscription_id = session_state.last_subscription_id + 1;
        // TODO server settings could revise these in some way
        let revised_publishing_interval = request.requested_publishing_interval;
        let revised_lifetime_count = request.requested_lifetime_count;
        let revised_max_keep_alive_count = request.requested_max_keep_alive_count;

        // Create a new subscription
        let subscription = Subscription {
            subscription_id: subscription_id,
            publishing_interval: revised_publishing_interval,
            lifetime_count: revised_lifetime_count,
            keep_alive_count: revised_max_keep_alive_count,
            priority: request.priority,
            monitored_items: Vec::new(),
        };
        session_state.last_subscription_id += 1;
        session_state.subscriptions.push(subscription);

        // Create the response
        let response = CreateSubscriptionResponse {
            response_header: ResponseHeader::new(&DateTime::now(), request.request_header.request_handle),
            subscription_id: subscription_id,
            revised_publishing_interval: revised_publishing_interval,
            revised_lifetime_count: revised_lifetime_count,
            revised_max_keep_alive_count: revised_max_keep_alive_count,
        };
        Ok(SupportedMessage::CreateSubscriptionResponse(response))
    }

    pub fn publish(&self, _: &mut ServerState, _: &mut SessionState, request: &PublishRequest) -> Result<SupportedMessage, &'static StatusCode> {
        debug!("publish {:#?}", request);
        Err(&BAD_SERVICE_UNSUPPORTED)
    }
}