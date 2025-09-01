#[cfg(test)]
mod email_service_controller_integration_tests {
    use candid::{decode_one, encode_args, Principal};
    use common::types::EmailArgs;
    use pocket_ic::PocketIc;
    use std::fs::read;

    // imports needed for HTTP-outcall mocking
    use pocket_ic::common::rest::{
        CanisterHttpReply, CanisterHttpResponse, MockCanisterHttpResponse,
    };

    use network_call::Configuration;

    // Helper function to set up the canister
    fn setup_canister() -> (PocketIc, Principal) {
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        // plenty of cycles for install + outcall
        pic.add_cycles(canister_id, 2_000_000_000_000);

        // Load your compiled WASM file
        let wasm = read("../../target/wasm32-unknown-unknown/release/network_call.wasm")
            .expect("Could not read WASM file");

        let init_args = create_test_configuration();
        pic.install_canister(canister_id, wasm, encode_args((init_args,)).unwrap(), None);

        (pic, canister_id)
    }

    fn create_test_configuration() -> Configuration {
        Configuration::new(
            "https://api.mocked-courier.com/send".to_string(),
            "mocked-token".to_string(),
            "test-template-id".to_string(),
        )
    }

    #[test]
    fn test_send_email_success_case() {
        let (pic, canister_id) = setup_canister();

        let email_args = EmailArgs {
            to: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
            action_url: "https://example.com".to_string(),
        };

        // 1) Submit the call (do NOT use update_call here)
        let call_id = pic
            .submit_call(
                canister_id,
                Principal::anonymous(),
                "send_email",
                candid::encode_args((email_args,)).unwrap(),
            )
            .expect("submit_call failed");

        // 2) Give PocketIC a couple ticks to enqueue the canister HTTP request
        pic.tick();
        pic.tick();

        // 3) Capture the pending outcall
        let pending = pic.get_canister_http();
        assert_eq!(pending.len(), 1, "expected exactly one HTTP outcall");
        let req = &pending[0];

        // (Optional) You could inspect `req.url`, `req.method`, `req.headers`, `req.body` here
        // to assert that your service built the request correctly.

        // 4) Build a deterministic mocked response
        let mock = MockCanisterHttpResponse {
            subnet_id: req.subnet_id,
            request_id: req.request_id,
            response: CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
                status: 200,
                headers: vec![], // headers your transform may keep/clear
                body: b"{\"ok\":true}".to_vec(), // small body (< MAX_RESPONSE_BYTES)
            }),
            additional_responses: vec![], // leave empty unless you want divergence tests
        };

        // 5) Inject the mock
        pic.mock_canister_http_response(mock);

        // 6) Await the original call to finish and decode the controller's return value
        let reply_blob = pic.await_call(call_id).expect("await_call failed");
        let result_string: String = decode_one(&reply_blob).unwrap();

        assert_eq!(result_string, "Email sent via Courier");
    }
}
