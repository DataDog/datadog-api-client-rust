// Schedule a downtime until date
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_downtimes::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        Downtime::new()
            .end(Some((Utc::now() + chrono::Duration::hours(1)).timestamp()))
            .message(Some("Example-Downtime".to_string()))
            .monitor_tags(vec!["tag0".to_string()])
            .mute_first_recovery_notification(true)
            .notify_end_states(vec![NotifyEndState::ALERT])
            .notify_end_types(vec![NotifyEndType::CANCELED])
            .recurrence(
                Some(
                    DowntimeRecurrence::new()
                        .period(1)
                        .type_("weeks".to_string())
                        .until_date(Some((Utc::now() + chrono::Duration::days(21)).timestamp()))
                        .week_days(
                            Some(
                                vec![
                                    "Mon".to_string(),
                                    "Tue".to_string(),
                                    "Wed".to_string(),
                                    "Thu".to_string(),
                                    "Fri".to_string()
                                ],
                            ),
                        ),
                ),
            )
            .scope(vec!["*".to_string()])
            .start((Utc::now()).timestamp())
            .timezone("Etc/UTC".to_string());
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.create_downtime(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
