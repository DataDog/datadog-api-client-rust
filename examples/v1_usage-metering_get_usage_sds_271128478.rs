// Get hourly usage for Sensitive Data Scanner returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api::api_usage_metering::GetUsageSDSOptionalParams;
use datadog_api_client::datadogV1::api::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_usage_sds(
            "2021-11-06T11:11:11+00:00".to_string(),
            GetUsageSDSOptionalParams::default().end_hr("2021-11-08T11:11:11+00:00".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
