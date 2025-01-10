// Cancel a historical job returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    // there is a valid "historical_job" in the system
    let historical_job_data_id = std::env::var("HISTORICAL_JOB_DATA_ID").unwrap();

    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CancelHistoricalJob", true);
    configuration.set_unstable_operation_enabled("v2.RunHistoricalJob", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .cancel_historical_job(historical_job_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
