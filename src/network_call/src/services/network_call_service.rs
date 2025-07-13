use ic_cdk::api::{
    call::CallResult,
    management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
        TransformContext,
    },
};

const CYCLES: u128 = 25_000_000_000;
const MAX_RESPONSE_BYTES: u64 = 2048;

pub struct NetworkCallService;

impl NetworkCallService {
    pub async fn send_http_request(
        url: String,
        http_method: HttpMethod,
        headers: Vec<HttpHeader>,
        body: Option<Vec<u8>>,
    ) -> CallResult<(HttpResponse,)> {
        let request = CanisterHttpRequestArgument {
            url: url,
            method: http_method,
            headers: headers,
            body: body,
            max_response_bytes: Some(MAX_RESPONSE_BYTES),
            transform: Some(TransformContext::from_name("transform".to_string(), vec![])),
        };

        return http_request(request, CYCLES).await;
    }
}
