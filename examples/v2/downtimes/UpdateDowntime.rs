// Update a downtime returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_downtimes::DowntimesAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "downtime_v2" in the system
    let downtime_v2_data_id = std::env::var("DOWNTIME_V2_DATA_ID").unwrap();
    let body =
        DowntimeUpdateRequest::new(
            DowntimeUpdateRequestData::new(
                DowntimeUpdateRequestAttributes::new().message(Some("light speed".to_string())),
                downtime_v2_data_id,
                DowntimeResourceType::DOWNTIME,
            ),
        );
    let configuration = Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.update_downtime(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
