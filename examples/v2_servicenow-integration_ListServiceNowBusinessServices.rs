// List ServiceNow business services returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_now_integration::ServiceNowIntegrationAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListServiceNowBusinessServices", true);
    let api = ServiceNowIntegrationAPI::with_config(configuration);
    let resp = api
        .list_service_now_business_services(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
