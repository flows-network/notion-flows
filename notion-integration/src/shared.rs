use chrono::Duration;
use once_cell::sync::OnceCell;
use reqwest::Client;

use crate::POLLING_INTERVAL_SECS;

pub fn get_polling_interval() -> &'static Duration {
    static INS: OnceCell<Duration> = OnceCell::new();
    INS.get_or_init(|| Duration::seconds(POLLING_INTERVAL_SECS))
}

pub fn get_client() -> &'static Client {
    static INS: OnceCell<Client> = OnceCell::new();
    INS.get_or_init(Client::new)
}
