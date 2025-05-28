use ic_cdk::post_upgrade;
use ic_cdk::println;

use crate::services::TimerService;

#[post_upgrade]
async fn post_upgrade() {
    let timers = TimerService::get_all();

    for timer in timers {
        TimerService::restore_timer(timer).await;
    }

    println!("Restored {} timers", TimerService::size());
}
