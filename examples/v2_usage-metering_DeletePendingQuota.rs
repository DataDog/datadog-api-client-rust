// Cancel a scheduled usage quota limit returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeletePendingQuota", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .delete_pending_quota(
            "ai_credits".to_string(),
            "MTIzNB9haV9jcmVkaXRzH3VzZXJfaGFuZGxlOl9fQUxMX18".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
