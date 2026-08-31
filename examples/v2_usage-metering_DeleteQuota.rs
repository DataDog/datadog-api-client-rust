// Delete a usage quota returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteQuota", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .delete_quota(
            "ai_credits".to_string(),
            "MjAfYWlfY3JlZGl0c1911c2VyX2hhbmRsZTpfX0FMTF9f".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
