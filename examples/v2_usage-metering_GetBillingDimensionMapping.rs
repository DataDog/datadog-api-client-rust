// Get billing dimension mapping for usage endpoints returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::GetBillingDimensionMappingOptionalParams;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetBillingDimensionMapping", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_billing_dimension_mapping(GetBillingDimensionMappingOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
