use common::types::EmailArgs;
use ic_cdk::{
    api::{
        management_canister::http_request::{HttpHeader, HttpMethod, HttpResponse, TransformArgs},
        time,
    },
    println, query, update,
};
use serde_json::json;

use crate::services::NetworkCallService;

const COURIER_URL: &str = "https://api.courier.com/send";
const COURIER_AUTH_TOKEN: &str = "dk_prod_DYH1W2CSCS4NN4GJTSCSFKGD6673";
const TEMPLATE_ID: &str = "TFE0BHBVT0M47SKQAQFYKDPMY8HD";

#[update]
async fn send_email(args: EmailArgs) -> String {
    println!("{:?}", args);

    let courier_payload = json!({
        "message": {
            "to": {
                "email": args.to
            },
            "template": TEMPLATE_ID,
            "data": {
                "subject": args.subject,
                "message": args.message,
                "dao_name": "Dao Forge",
                "notification_type": "Update",
                "action_url": "https://google.com",
            },
        }
    });

    let idempotency_key = format!("{} {}", time(), args.to);

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", COURIER_AUTH_TOKEN),
        },
        HttpHeader {
            name: "Idempotency-Key".to_string(),
            value: idempotency_key,
        },
    ];

    let response = NetworkCallService::send_http_request(
        COURIER_URL.to_string(),
        HttpMethod::POST,
        request_headers,
        Some(courier_payload.to_string().into_bytes()),
    )
    .await;

    let message: String;

    match response {
        Ok((response,)) => {
            if response.status == 200u64 || response.status == 202u64 {
                message = "Email sent via Courier".to_string();
            } else {
                message = format!("Courier API error: {}", response.status);
            }
        }
        Err((r, m)) => {
            message = format!("HTTP outcall failed: {:?} {}", r, m);
        }
    }

    println!("{}", message);
    message
}

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers: vec![],
    }
}
