use candid::Principal;

use crate::{services::InterCanisterService, types::EmailArgs};

pub struct NetworkCallService;

impl NetworkCallService {
    pub async fn send_email(
        to: String,
        subject: String,
        message: String,
    ) -> Result<String, String> {
        let args = EmailArgs {
            to,
            subject,
            message,
        };

        return InterCanisterService::call(
            Principal::from_text("haulv-eiaaa-aaaaa-aaaaq-a2y").unwrap(),
            &"send_email",
            (args,),
        )
        .await;
    }
}
