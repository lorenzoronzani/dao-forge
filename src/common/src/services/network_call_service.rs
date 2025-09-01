use candid::Principal;

use crate::{services::InterCanisterService, types::EmailArgs};

pub struct NetworkCallService;

impl NetworkCallService {
    pub async fn send_email(
        canister_id: Principal,
        to: String,
        subject: String,
        message: String,
        action_url: String,
    ) -> Result<String, String> {
        let args = EmailArgs {
            to,
            subject,
            message,
            action_url,
        };

        return InterCanisterService::call(canister_id, &"send_email", (args,)).await;
    }
}

#[cfg(test)]
#[path = "network_call_service_tests.rs"]
mod network_call_service_tests;
