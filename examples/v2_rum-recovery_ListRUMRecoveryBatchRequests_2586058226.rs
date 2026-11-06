// List batch history filters request creation time independently of session time
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
            ListRUMRecoveryBatchRequestsOptionalParams::default()
                .from(1)
                .to(2)
                .query("service:rum18600-fixture AND @usr.id:(123 OR 456)".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
