// Cancel your temporary grant request returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_temporary_grant_request", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .delete_temporary_grant_request(
            Uuid::parse_str("12345678-1234-1234-1234-123456789012").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
