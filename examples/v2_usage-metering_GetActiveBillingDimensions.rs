// Get active billing dimensions for cost attribution returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::GetActiveBillingDimensionsOptionalParams;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_active_billing_dimensions(GetActiveBillingDimensionsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
