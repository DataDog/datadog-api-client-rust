// Schedule a downtime with until occurrences
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
            .recurrence(
                Some(
                    DowntimeRecurrence::new()
                        .period(1)
                        .type_("weeks".to_string())
                        .until_occurrences(Some(3))
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
