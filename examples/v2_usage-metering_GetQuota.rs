// Get a usage quota returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::GetQuotaOptionalParams;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    // there is a valid "quota" in the system
    let quota_data_0_id = std::env::var("QUOTA_DATA_0_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_quotas", true);
    configuration.set_unstable_operation_enabled("v2.delete_quota", true);
    configuration.set_unstable_operation_enabled("v2.get_quota", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_quota(
            "ai_credits".to_string(),
            quota_data_0_id.clone(),
            GetQuotaOptionalParams::default().include_descendants(false),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
