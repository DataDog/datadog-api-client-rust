// List your temporary grant requests returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::ListMyTemporaryGrantRequestsOptionalParams;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_my_temporary_grant_requests", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let response = api.list_my_temporary_grant_requests_with_pagination(
        ListMyTemporaryGrantRequestsOptionalParams::default(),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
