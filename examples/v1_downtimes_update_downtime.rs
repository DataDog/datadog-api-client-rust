// Update a downtime returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_downtimes::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "downtime" in the system
    let downtime_id: i64 = std::env::var("DOWNTIME_ID").unwrap().parse().unwrap();
    let body =
        Downtime::new()
            .message(Some("Example-Downtime-updated".to_string()))
            .mute_first_recovery_notification(true)
            .notify_end_states(vec![NotifyEndState::ALERT, NotifyEndState::NO_DATA, NotifyEndState::WARN])
            .notify_end_types(vec![NotifyEndType::CANCELED, NotifyEndType::EXPIRED]);
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.update_downtime(downtime_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
