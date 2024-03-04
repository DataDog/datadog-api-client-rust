// Schedule a downtime once a year
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_downtimes::DowntimesAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        Downtime::new()
            .end(
                Some(
                    SystemTime::now()
                        .add(Duration::from_secs(1 * 3600))
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as
                        i64,
                ),
            )
            .message(Some("Example-Downtime".to_string()))
            .monitor_tags(vec!["tag0".to_string()])
            .mute_first_recovery_notification(true)
            .notify_end_states(vec![NotifyEndState::ALERT, NotifyEndState::WARN])
            .notify_end_types(vec![NotifyEndType::EXPIRED])
            .recurrence(Some(DowntimeRecurrence::new().period(1).type_("years".to_string())))
            .scope(vec!["*".to_string()])
            .start(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
            .timezone("Etc/UTC".to_string());
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.create_downtime(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
