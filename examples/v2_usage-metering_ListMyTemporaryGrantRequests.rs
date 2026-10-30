// List your temporary grant requests returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::ListMyTemporaryGrantRequestsOptionalParams;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_my_temporary_grant_requests", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .list_my_temporary_grant_requests(ListMyTemporaryGrantRequestsOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
