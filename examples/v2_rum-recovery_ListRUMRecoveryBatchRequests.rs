// List batch recovery requests returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_recovery::ListRUMRecoveryBatchRequestsOptionalParams;
use datadog_api_client::datadogV2::api_rum_recovery::RUMRecoveryAPI;
use datadog_api_client::datadogV2::model::RumRecoveryBatchRequestType;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = RUMRecoveryAPI::with_config(configuration);
    let resp = api
        .list_rum_recovery_batch_requests(
            RumRecoveryBatchRequestType::BATCH,
            ListRUMRecoveryBatchRequestsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
