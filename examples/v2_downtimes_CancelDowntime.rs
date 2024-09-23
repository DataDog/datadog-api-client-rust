// Cancel a downtime returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_downtimes::DowntimesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "downtime_v2" in the system
    let downtime_v2_data_id = std::env::var("DOWNTIME_V2_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = DowntimesAPI::with_config(configuration);
    let resp = api.cancel_downtime(downtime_v2_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
